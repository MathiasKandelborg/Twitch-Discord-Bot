use std::time::*;
use std::env::var;
use tungstenite::{connect, Message};

use notify_rust::Notification;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

fn nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(18).collect()
}

#[derive(Serialize, Deserialize)]
struct DataObj {
    topics: Vec<String>,
    auth_token: String,
}

#[derive(Serialize, Deserialize)]
struct TopicListener {
    #[serde(rename = "type")]
    event: String,
    nonce: String,
    data: DataObj,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsRewardMaxPerStream {
    is_enabled: bool,
    max_per_stream: i16,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsRewardImage {
    url_1x: String,
    url_2x: String,
    url_4x: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsRewardDefaultImage {
    url_1x: String,
    url_2x: String,
    url_4x: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsRedemptionUser {
    id: String,
    login: String,
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsRedemptionReward {
    channel_id: String,
    title: String,
    prompt: String,
    cost: i16,
    is_user_input_required: bool,
    is_sub_only: bool,
    default_image: ChannelPointsRewardDefaultImage,
    background_color: String,
    is_enabled: bool,
    is_paused: bool,
    is_in_stock: bool,
    max_per_stream: ChannelPointsRewardMaxPerStream,
    should_redemptions_skip_request_queue: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsRedemption {
    id: String,
    user: ChannelPointsRedemptionUser,
    channel_id: String,
    redeemed_at: String,
    reward: ChannelPointsRedemptionReward,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_input: Option<String>,
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsMsgData {
    timestamp: String,
    redemption: ChannelPointsRedemption,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsMsg {
    #[serde(rename = "type")]
    event: String,
    data: ChannelPointsMsgData,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsMetaMsg {
    topic: String,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChannelPointsRes {
    #[serde(rename = "type")]
    event: String,
    data: ChannelPointsMetaMsg,
}

fn main() -> Result<()> {
    env_logger::init();

    const WS_URL: &'static str = "wss://pubsub-edge.twitch.tv";
    // TODO: Read from ENV, use soft-coded values
    let channel_id = var("T_CHANNEL_ID").expect("Twitch channel id not found");
    let twitch_auth_token = var("T_AUTH_TOKEN").expect("Twitch auth token not found");
    
    let nonce_str = nonce();

    let listen_msg = TopicListener {
        event: "LISTEN".to_string(),
        nonce: nonce_str,
        data: DataObj {
            topics: vec![
                format!("channel-points-channel-v1.{}", &channel_id),
                format!("following.{}", &channel_id),
            ],
            auth_token: twitch_auth_token,
        },
    };

    let (mut socket, _response) = connect(WS_URL).unwrap();

    // To keep the server from closing the connection, clients must send a PING command at least once every 5 minutes.
    // Clients must LISTEN on at least one topic within 15 seconds of establishing the connection, or they will be disconnected by the server.
    // Clients may receive a RECONNECT message at any time.
    // This indicates that the server is about to restart (typically for maintenance) and will disconnect the client within 30 seconds.
    // During this time, we recommend that clients reconnect to the server; otherwise, the client will be forcibly disconnected.
    let mut last_ping = Instant::now();
    let mut expected_pong = None;
    
    // If a client does not receive a PONG message within 10 seconds of issuing a PING command, it should reconnect to the server.
    let pong_timeout = Duration::from_secs(15);

    let listen_msg_str =
        serde_json::to_string(&listen_msg).expect("Failed to serialize listen msg");

    // Start List msg
    socket.write_message(Message::Text(listen_msg_str)).unwrap();

    std::thread::sleep(Duration::from_millis(20));

    // Tear apart socket
    match socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    // Set to non-blocking
    .set_nonblocking(true)
    .unwrap();

    // Main loop
    loop {
        match socket.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // we're blocking
            }
            // Panic for no reason because we're handling errors properly
            Err(err) => panic!(err),
            Ok(Message::Text(res)) => {
                // Playing ping pong (this is the pong part)
                if res.contains("PONG") {
                    expected_pong = None;
                    println!("Recived PONG!");
                }

                // Look at topic messages!

                // Channel Redemption msg
                if res.contains("data") {
                    let redemption_meta_msg: ChannelPointsRes = serde_json::from_str(&res.trim())?;
                    // Channel point redemption msg
                    let redemption_msg: ChannelPointsMsg =
                        serde_json::from_str(&redemption_meta_msg.data.message.to_string())?;

                    if redemption_msg.data.redemption.reward.title.contains("Hydrate!"){
                        // Send notification for hydration events
                        Notification::new().summary(&redemption_msg.data.redemption.reward.title).body(&redemption_msg.data.redemption.reward.prompt).show().unwrap();
                    };

                    println!("{:?}", redemption_msg);
                }
            }
            Ok(..) => {
                // Other things Twitch doesn't do
            }
        }

        // Thanks to `museun` for making this
        // Every minute send a ping to the socket
        // Twitch is special so we send a Text containing PING
        if last_ping.elapsed() > Duration::from_secs(1 * 60) {
            socket
                .write_message(Message::Text(r#"{"type": "PING"}"#.to_string()))
                .unwrap();

            println!("Sending PING");
            expected_pong = Some(Instant::now());
            last_ping = Instant::now();
        }

        // Thanks to `museun` for making this
        // If pong timed out, stop listening and break the loop
        if let Some(dt) = expected_pong {
            if dt.elapsed() > pong_timeout {
                println!("PONG timed out");
                break;
            }
        }

        std::thread::sleep(Duration::from_millis(60));
    }
    Ok(())
}

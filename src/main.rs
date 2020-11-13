use std::env::var;
use std::time::{Duration, Instant};

use tungstenite::{connect, Message};

use serde_json::Result;

use channel_points_redemption::channel_points_redemption;
use common_structs::ChannelPointsRes;
use twitch_discord_bot::{
    channel_points_redemption, check_ping, common_structs, generate_listen_msg, nonce,
};

fn main() -> Result<()> {
    env_logger::init();

    const WS_URL: &'static str = "wss://pubsub-edge.twitch.tv";

    let channel_id = var("T_CHANNEL_ID").expect("Twitch channel id not found");
    let twitch_auth_token = var("T_AUTH_TOKEN").expect("Twitch auth token not found");

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

    // Generate Listen Message
    let listen_msg_str = serde_json::to_string(&generate_listen_msg(
        nonce(),
        channel_id.to_string(),
        twitch_auth_token,
    ))
    .expect("Failed to serialize listen msg");

    // Start web socket
    socket.write_message(Message::Text(listen_msg_str)).unwrap();

    // Wait a bit before doing stuff
    // I.e, the web socket needs to connect etc.
    std::thread::sleep(Duration::from_millis(20));

    // 1. Tear apart socket
    // 2. Set to non-blocking
    match socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
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
            Err(err) => panic!(err),
            Ok(Message::Text(res)) => {
                if res.contains("PONG") {
                    expected_pong = None;
                    println!("Recived PONG!");
                }

                // println!("{:?}", res); // for debugging

                if res.contains("data") {
                    let res_msg: ChannelPointsRes =
                        serde_json::from_str(res.trim()).expect("Could not deserialize meta msg");
                    
                    let topic_str = &res_msg.data.topic;
                    let channel_points_topic = format!("channel-points-channel-v1.{}", &channel_id);

                    let channel_follower_topic = format!("following.{}", &channel_id);
                    

                    if topic_str.contains(channel_points_topic.as_str()) {
                        channel_points_redemption(&res_msg);
                    } else if topic_str.contains(channel_follower_topic.as_str()) {
                        println!("Follower topic caught");
                    } else {
                        println!("Did topic string");
                    }
                }
            }
            Ok(..) => {
                // Other things Twitch doesn't do
            }
        }

        match check_ping(
            pong_timeout,
            &mut last_ping,
            &mut expected_pong,
            &mut socket,
        ) {
            Ok(_msg) => {
                // println!("{}", msg);
            }
            Err(err) => {
                println!("ERROR: {}", err);
                break;
            }
        };
    }

    std::thread::sleep(Duration::from_millis(60));

    Ok(())
}

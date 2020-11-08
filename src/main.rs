use native_tls::TlsStream;
use std::env::var;
use std::net::TcpStream;
use std::process::Command;
use std::str::from_utf8;
use std::time::{Duration, Instant};
use tungstenite::stream::Stream;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use tungstenite::{connect, Message, WebSocket};

use serde_json::Result;

use notify_rust::Notification;

mod generate_listen_message;

use self::generate_listen_message::channel_points_reward_msg::listen_msg_structs::{
    ChannelPointsMsg, ChannelPointsRes,
};

use generate_listen_message::generate_listen_msg;

fn nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(18).collect()
}

fn check_ping(
    pong_timeout: Duration,
    last_ping: &mut Instant,
    expected_pong: &mut Option<Instant>,
    socket: &mut WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
) -> std::result::Result<String, String> {
    if last_ping.elapsed() > Duration::from_secs(1 * 60) {
        socket
            .write_message(Message::Text(r#"{"type": "PING"}"#.to_string()))
            .unwrap();

        println!("Sending PING");
        *expected_pong =  Some(Instant::now());
        *last_ping =  Instant::now();
    }

    // Thanks to `museun` for making this
    // If pong timed out, stop listening and break the loop
    if let Some(dt) = expected_pong {
        if dt.elapsed() > pong_timeout {
            println!("PONG timed out");
            Err("PONG Timed out".to_string())
        } else {
            Ok("Recived PONG".to_string())
        }
    } else {
        Ok("".to_string())
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let test = Command::new("sh")
        .arg("-c")
        .arg("echo hello")
        .output()
        .expect("failed to execute process");

    println!("{:?}", from_utf8(&test.stdout).unwrap().trim());

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
    let listen_msg_str =
        serde_json::to_string(&generate_listen_msg(nonce(), channel_id, twitch_auth_token))
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
                    let redemption_meta_msg: ChannelPointsRes = serde_json::from_str(&res.trim())
                        .expect("Could not deserialize Channel Points meta message");
                    // Channel point redemption msg
                    let redemption_msg: ChannelPointsMsg =
                        serde_json::from_str(&redemption_meta_msg.data.message.to_string())
                            .expect("Could not deserialize Channel Points data message");

                    if redemption_msg
                        .data
                        .redemption
                        .reward
                        .title
                        .contains("Hydrate!")
                    {
                        // Send notification for hydration events
                        Notification::new()
                            .summary(&redemption_msg.data.redemption.reward.title)
                            .body(&redemption_msg.data.redemption.reward.prompt)
                            .show()
                            .unwrap();
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
        match check_ping(pong_timeout, &mut last_ping, &mut expected_pong, &mut socket) {
            Ok(msg) => {
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

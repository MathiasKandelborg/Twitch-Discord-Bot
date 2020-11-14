use std::env::var;
use std::time::{Duration, Instant};

use tungstenite::{connect, Message};

use serde_json::Result;

use channel_points_redemption::channel_points_redemption;
use common_structs::ChannelPointsRes;
use new_follower::new_follower;
use parse_twitch_msg::parse_twitch_msg;
use twitch_discord_bot::{
    channel_points_redemption, check_ping, common_structs, generate_listen_msg, new_follower,
    nonce, parse_twitch_msg,
};

fn main() -> Result<()> {
    env_logger::init();

    const WS_URL: &'static str = "wss://pubsub-edge.twitch.tv";
    const WS_TWITCH_CHAT: &'static str = "wss://irc-ws.chat.twitch.tv:443";

    let channel_id = var("T_CHANNEL_ID").expect("Twitch channel id not found");
    let twitch_auth_token = var("T_AUTH_TOKEN").expect("Twitch auth token not found");
    let twitch_oauth_token = var("T_OAUTH_TOKEN").expect("Twitch chat token not found");

    let (mut chat_socket, _chat_response) = connect(WS_TWITCH_CHAT).unwrap();
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

    chat_socket
        .write_message(Message::Text(
            "CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership".to_string(),
        ))
        .unwrap();
    chat_socket
        .write_message(Message::Text(format!("PASS {}", &twitch_oauth_token)))
        .unwrap();

    chat_socket
        .write_message(Message::Text(format!("NICK TDBOT")))
        .unwrap();

    chat_socket
        .write_message(Message::Text(format!("JOIN #neonraytracer")))
        .unwrap();

    // Start web socket
    socket.write_message(Message::Text(listen_msg_str)).unwrap();

    match chat_socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();
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
        // Twitch chat
        match chat_socket.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // we're blocking
            }
            Err(err) => panic!(err),

            Ok(Message::Text(res)) => {
               //  println!("{}", res.as_str());

                match parse_twitch_msg(res) {
                    Some(msg) => println!(
                        "{}",
                        format!(
                            "user {} channnel_name {} message \n{}",
                            msg.display_name, msg.channel_name, msg.message
                        )
                    ),
                    None => {}
                };
            }
            Ok(..) => {}
        }

        // Twitch websockets
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

                if res.contains("data") {
                    let res_msg: ChannelPointsRes =
                        serde_json::from_str(res.trim()).expect("Could not deserialize meta msg");
                    // println!("{:#?}", &res_msg); // for debugging
                    let topic_str = &res_msg.data.topic;

                    match topic_str.as_str().split(".").collect::<Vec<&str>>()[0] {
                        "channel-points-channel-v1" => channel_points_redemption(&res_msg),

                        "following" => new_follower(&res_msg),
                        _ => {}
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
        // MAKE SURE THIS IS IN THE MAIN LOOP
        // YOUR PROCESSOR WILL GO BRRRRRRRRRRRRRRRRRR OTHERWISE
        std::thread::sleep(Duration::from_millis(60));
        // 👇 this is the end of the main loop
    }

    drop(socket);
    Ok(())
}

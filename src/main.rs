#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
use config::{Config, File};

use serde_json::{to_string, Result};

use std::env::var;
use std::time::{Duration, Instant};
use tungstenite::{connect, Message};

// Crate files
use channel_points_redemption::channel_points_redemption;
use common_structs::Res;
use new_follower::new_follower;
use parse_twitch_msg::parse_twitch_msg;
use twitch_chat_connect::twitch_chat_connect_messages;
use twitch_discord_bot::{
    channel_points_redemption, chat_command, check_ping, common_structs, generate_listen_msg,
    new_follower, nonce, parse_twitch_msg, twitch_chat_connect,
};

fn main() -> Result<()> {
    env_logger::init();

    let mut settings = Config::default();
    let mut commands = Config::default();

    settings
        .merge(File::with_name("config"))
        .expect("Couldn't read or find configuration file");

    commands
        .merge(File::with_name("commands"))
        .expect("Couldn't read or find commands file");

    const WS_URL: &'static str = "wss://pubsub-edge.twitch.tv";
    const WS_TWITCH_CHAT: &'static str = "wss://irc-ws.chat.twitch.tv:443";

    // TODO: Use env variables in settings obj
    let channel_id = var("T_CHANNEL_ID").expect("Twitch channel id not found");
    let twitch_auth_token = var("T_AUTH_TOKEN").expect("Twitch auth token not found");
    let twitch_oauth_token = var("T_OAUTH_TOKEN").expect("Twitch chat token not found");

    // Create a websocket for twitch PubSub
    let (mut ws_twitch, _response) = connect(WS_URL).unwrap();
    // Create a websocket for Twitch chat
    let (mut ws_twitch_chat, _chat_response) = connect(WS_TWITCH_CHAT).unwrap();


    let mut last_ping = Instant::now();
    let mut expected_pong = None;

    // If a client does not receive a PONG message within 10 seconds of issuing a PING command, it should reconnect to the server.
    let pong_timeout = Duration::from_secs(15);

    // Generate Listen Message
    let listen_msg_str = to_string(&generate_listen_msg(
        nonce(),
        channel_id.to_string(),
        twitch_auth_token,
    ))
    .expect("Failed to serialize listen msg");

    std::thread::sleep(Duration::from_millis(200));
    // Write connect messages to twitch chat ws
    twitch_chat_connect_messages(&mut ws_twitch_chat, &twitch_oauth_token);

    // 1. Tear socket apart
    // 2. Set to non-blocking
    match ws_twitch.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();

    // Same as above TODO: abstract this
    match ws_twitch_chat.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();

    // Start web socket
    ws_twitch
        .write_message(Message::Text(listen_msg_str))
        .unwrap();

    // Wait a bit before doing stuff
    // I.e, the web sockets needs to connect
    std::thread::sleep(Duration::from_millis(200));

    loop {
        // Twitch websockets
        match ws_twitch.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // it's a faaaaaake
                // we're blocking
            }
            Err(err) => println!("{}", err),
            Ok(Message::Text(res)) => {
                if res.contains("PONG") {
                    expected_pong = None;

                    // println!("Recived PONG!");
                }

                if res.contains("data") {
                    let res_msg: Res =
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
                // println!("{:#?}", test);
            }
        }

        match ws_twitch_chat.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // we're blocking
                // it's a faaaaaake
            }
            Err(err) => panic!(err),

            Ok(Message::Text(res)) => {
                // println!("{}", res.as_str()); // For debugging

                match parse_twitch_msg(res) {
                    Some(msg) => {
                        println!(
                            "\nUser \"{}\"\nIn {}'s channel\nWrote: \"{}\"",
                            msg.display_name,
                            msg.channel_name,
                            msg.message.trim()
                        );
                        chat_command::chat_commands::cmd_response(
                            msg,
                            &mut ws_twitch_chat,
                            &commands,
                        );
                    }
                    None => {}
                };
            }
            Ok(..) => {}
        }

        match check_ping(
            pong_timeout,
            &mut last_ping,
            &mut expected_pong,
            &mut ws_twitch,
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
        std::thread::sleep(Duration::from_millis(12));
        // 👇 main loop ends
    }

    Ok(())
}

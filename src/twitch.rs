use config::Config;
use native_tls::TlsStream;
use std::env::var;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::stream::Stream;
use tungstenite::{connect, Message, WebSocket};

use rand::{thread_rng, Rng};
use crate::{
    channel_points_redemption, chat_command::chat_commands, common_structs,common_structs::{DataObj, TopicListener},
    new_follower, nonce, parse_twitch_msg::parse_twitch_msg,
    twitch_chat_connect::twitch_chat_connect_messages,
};
use common_structs::Res;

pub fn generate_connect_msg(
    nonce: String,
    channel_id: String,
    twitch_auth_token: String,
) -> TopicListener {
    TopicListener {
        event: "LISTEN".to_string(),
        nonce,
        data: DataObj {
            topics: vec![
                format!("channel-points-channel-v1.{}", &channel_id),
                format!("following.{}", &channel_id),
            ],
            auth_token: twitch_auth_token,
        },
    }
}

pub struct TwidshTshadBott {
    socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
}

pub fn setup_twitch_chat_ws() -> TwidshTshadBott {
    const WS_TWITCH_CHAT: &'static str = "wss://irc-ws.chat.twitch.tv:443";
    let twitch_oauth_token = var("T_OAUTH_TOKEN").expect("Twitch chat token not found");

    let (mut socket, _response) = connect(WS_TWITCH_CHAT).unwrap();
    // Write connect messages to twitch chat ws
    twitch_chat_connect_messages(&mut socket, &twitch_oauth_token);

    // Same as above TODO: abstract this
    match socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();

    TwidshTshadBott { socket }
}
impl TwidshTshadBott {
    pub fn read_message(&mut self, commands: &Config) {
        // Twitch websockets
        match self.socket.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // we're blocking
                // it's a faaaaaake
            }
            Err(err) => {
                let _ = println!("TWITCH CHAT WS ERROR MSG:\n{}", err);
            }
            Ok(Message::Text(res)) => {
                if res.contains("PING") {
                // Send PONG if Twitch is going PING
                    self.socket
                        .write_message(Message::Text("PONG :tmi.twitch.tv".to_string()))
                        .expect("Could not write pong msg to twitch");

                    println!("Recived Twitch Chat PING! Sent PONG!");
                }

                // println!("{}", res.as_str()); // For debugging
                match parse_twitch_msg(res) {
                    Some(msg) => {
                        println!(
                            "\nUser \"{}\"\nIn {}'s channel\nWrote: \"{}\"",
                            msg.display_name,
                            msg.channel_name,
                            msg.message.trim()
                        );
                        chat_commands::cmd_response(msg, &mut self.socket, &commands);
                    }
                    None => {}
                };
            }
            Ok(..) => {}
        }
    }
}

pub struct TwidsgPubSubBott {
    channel_id: String,
    socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
    expected_pong: Option<Instant>,
    last_ping: Instant,
    pong_timeout: Duration,
}

pub fn create_twitch_pubsub_ws() -> TwidsgPubSubBott {
    let channel_id = var("T_CHANNEL_ID").expect("Twitch channel id not found");
    const WS_URL: &'static str = "wss://pubsub-edge.twitch.tv";

    // If a client does not receive a PONG message within 10 seconds of issuing a PING command, it should reconnect to the server.
    let pong_timeout = Duration::from_secs(15);
    let expected_pong = None;

    // Create a websocket for twitch PubSub
    let (socket, _response) = connect(WS_URL).unwrap();

    // 1. Tear socket apart
    // 2. Set to non-bloc0king
    match socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();

    TwidsgPubSubBott {
        channel_id,
        pong_timeout,
        expected_pong,
        socket,
        last_ping: Instant::now(),
    }
}

impl TwidsgPubSubBott {

    pub fn setup(&mut self) {
        let twitch_auth_token = var("T_AUTH_TOKEN").expect("Twitch auth token not found");

        // Generate Listen Message
        let t_ws_connect_msg = serde_json::to_string(&generate_connect_msg(
            nonce(),
            self.channel_id.to_string(),
            twitch_auth_token,
        ))
        .expect("Failed to serialize listen msg");

        // Start web socket
        self.socket
            .write_message(Message::Text(t_ws_connect_msg))
            .unwrap();

        // Wait a bit before doing stuff
        // I.e, the web sockets needs to connect
        std::thread::sleep(Duration::from_millis(100));

        self.socket
            .write_message(Message::Text(r#"{"type": "PING"}"#.to_string()))
            .unwrap();
    }

    pub fn read_message(&mut self) {
        match self.socket.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // it's a faaaaaake
                // we're blocking
            }
            Err(err) => {
                let _ = println!("I AM A TWITCH WS ERROR\n{}", err);
            }
            Ok(Message::Text(res)) => {
                if res.contains("PONG") {
                    self.expected_pong = None;

                    println!("Recived Twitch WS PONG!");
                }

                println!("{}", res.trim());
                if res.contains("data") {
                    let res_msg: Res =
                        serde_json::from_str(res.trim()).expect("Could not deserialize meta msg");
                    // println!("{:#?}", &res_msg); // for debugging
                    let topic_str = &res_msg.data.topic;

                    match topic_str.as_str().split(".").collect::<Vec<&str>>()[0] {
                        "channel-points-channel-v1" => {
                            channel_points_redemption::channel_points_redemption(&res_msg)
                        }

                        "following" => new_follower::new_follower(&res_msg),
                        _ => {}
                    }
                }
            }
            Ok(..) => {
                // Other things Twitch doesn't do
                // println!("{:#?}", test);
            }
        }
    }

    pub fn ping_pong(&mut self) {
        match check_ping(
            self.pong_timeout,
            &mut self.last_ping,
            &mut self.expected_pong,
            &mut self.socket,
        ) {
            Ok(_msg) => {
                // println!("{}", msg);
            }
            Err(err) => {
                println!("ERROR: {}", err);
            }
        };
    }
}


/* Courtesy of museun
    - Check when last ping was sent
    - Respond to pong
    - Set timers
*/
pub fn check_ping(
    pong_timeout: Duration,
    last_ping: &mut Instant,
    expected_pong: &mut Option<Instant>,
    socket: &mut WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
) -> std::result::Result<String, String> {

    let jitter = thread_rng().gen_range(100, 300);

   if last_ping.elapsed() > Duration::from_millis(100 * 10 * 60 * 4) + Duration::from_millis(jitter) {
       socket
            .write_message(Message::Text(r#"{"type": "PING"}"#.to_string()))
            .unwrap();

        *expected_pong = Some(Instant::now());
        *last_ping = Instant::now();
    }

    // If pong timed out, stop listening and break the loop
    // TODO: We probably want to handle reconnecting: https://dev.twitch.tv/docs/pubsub#connection-management
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

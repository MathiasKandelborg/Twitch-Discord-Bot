use config::Config;
use native_tls::TlsStream;
use std::env::var;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::stream::Stream;
use tungstenite::{connect, Message, WebSocket};

use crate::{
    channel_points_redemption,
    chat_command::chat_commands,
    common_structs,
    common_structs::{DataObj, TopicListener},
    new_follower, nonce,
    parse_twitch_msg::parse_twitch_msg,
};
use common_structs::Res;
use rand::{thread_rng, Rng};

pub struct Disconnected;

type Result<T> = std::result::Result<T, Disconnected>;

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
    oauth_token: String,
    socket_url: String,
    last_back_off: Option<Instant>,
    back_off_timer: Duration,
    settings: &Config,
}

pub fn setup_twitch_chat_ws() -> TwidshTshadBott {
    let url = "wss://irc-ws.chat.twitch.tv:443";
    let oauth_token = var("T_OAUTH_TOKEN").expect("Twitch chat token not found");

    let back_off_timer = Duration::from_secs(2);
    let last_back_off = None;

    let socket = setup_socket(url.to_string());

    TwidshTshadBott {
        socket,
        socket_url: url.to_string(),
        oauth_token,
        back_off_timer,
        last_back_off,
    }
}

impl TwidshTshadBott {
    pub fn main(&mut self, commands: &Config, settings: &Config) {
        if let Err(Disconnected) = self.read_message(commands) {
            self.back_off()
        } else {
           // println!("Chat read msg successful");
        }
        self.settings = settings;
    }

    pub fn send_ping(&mut self) -> Result<()> {
        // Send PONG if Twitch is going PING
       // println!("Recived Twitch Chat PING! Sent PONG!");

        let send_pong = self
            .socket
            .write_message(Message::Text("PONG :tmi.twitch.tv".to_string()));

        match send_pong {
            Ok(_) => {
                println!("Sent Twitch chat pong");
                Ok(())
            }
            Err(err) => {
                println!("Could not write pong msg to twitch:\n {}\n", err);
                Err(Disconnected)
            }
        }
    }

    pub fn read_message(&mut self, commands: &Config) -> Result<()> {
        if !self.socket.can_read() {
            println!("Chat Cats can't read!!!");
            return Err(Disconnected);
        }
        if !self.socket.can_write() {
            println!("Can't write!!!");
            return Err(Disconnected);
        }
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
                return Err(Disconnected);
            }
            Ok(Message::Text(res)) => {
                if res.contains("PING") {
                    if let Err(Disconnected) = self.send_ping() {
                       return Err(Disconnected);
                    }
                }

         //       println!("{}", res.trim()); // For debugging
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
        Ok(())
    }
   pub fn send_listen_msg(&mut self) {
        self.socket
            .write_message(Message::Text(
                "CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership".to_string(),
            ))
            .unwrap();
        self.socket
            .write_message(Message::Text(format!("PASS {}", self.oauth_token)))
            .unwrap();

        self.socket
            .write_message(Message::Text(format!("NICK idontmatterlol")))
            .unwrap();

        self.socket
            .write_message(Message::Text(format!("JOIN #neonraytracer")))
            .unwrap();

       self.last_back_off = None;
    }

    fn back_off(&mut self) {
        let max_back_off: Duration = Duration::from_secs(120);

        if let Some(last) = self.last_back_off {
            if last.elapsed() > self.back_off_timer && !(self.back_off_timer > max_back_off) {
                println!("Backing off chat for REEEEEEEEEEEEEEEEALZ");
                self.back_off_timer = self.back_off_timer * 2;
                self.socket = setup_socket(self.socket_url.to_string());
                self.send_listen_msg();

                self.last_back_off = Some(Instant::now());
            } else {
                println!("I GIBE UP ON CHAT 🤬");
            }
        } else {
            println!("No last chat back off");
            self.last_back_off = Some(Instant::now());
        }
    }
}

pub struct TwidshPubSubBott {
    channel_id: String,
    socket_url: String,
    socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
    last_back_off: Option<Instant>,
    back_off_timer: Duration,
    expected_pong: Option<Instant>,
    last_ping: Instant,
    pong_timeout: Duration,
    settings: &Config,
}

impl TwidshPubSubBott {

    pub fn main(&mut self, settings: &Config) {
       // println!("Running main");
        if let Err(Disconnected) = self.read_message() {
            println!("Recieved Disconnect, backing off");
            self.back_off();
        } else {
         //   println!("Sending ping pong");
            if let Err(Disconnected) = self.ping_pong() {
                println!("Recieved PubSub ping Disconnect, backing off");
                self.back_off();
            } else {
           //     println!("PubSub Ping successfull");
            }
        }
        self.settings = settings;
    }

    pub fn send_listen_msg(&mut self) {
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

        self.last_back_off = None;

        self.socket
            .write_message(Message::Text(r#"{"type": "PING"}"#.to_string()))
            .unwrap();
    }

    pub fn read_message(&mut self) -> Result<()> {
        if !self.socket.can_read() {
            println!("Cats can't read!!!");
            return Err(Disconnected);
        }
        if !self.socket.can_write() {
            println!("Can't write!!!");
            return Err(Disconnected);
        }
        //  println!("Reading PubSub message!!!");
        match self.socket.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // it's a faaaaaake
                // we're blocking
            }
            Err(err) => {
                println!("Twitch PubSub WS disconnect:\n{}", err);
                return Err(Disconnected);
            }

            Ok(Message::Text(res)) => {
                if res.contains("PONG") {
                    self.expected_pong = None;

                    println!("Recived Twitch WS PONG!");
                }

               // println!("{}", res.trim());
                if res.contains("data") {
                    let res_msg: Res =
                        serde_json::from_str(res.trim()).expect("Could not deserialize meta msg");
                    // println!("{:#?}", &res_msg); // for debugging
                    let topic_str = &res_msg.data.topic;

                    match topic_str.as_str().split(".").collect::<Vec<&str>>()[0] {
                        "channel-points-channel-v1" => {
                            channel_points_redemption::channel_points_redemption(&res_msg, &settings)
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
        Ok(())
    }

    pub fn ping_pong(&mut self) -> Result<()> {
        match &mut self.check_ping() {
            Ok(_msg) => {
                // println!("{}", msg);
                Ok(())
            }
            Err(err) => {
                println!("Twitch PubSub WS ERROR: {}", err);
                Err(Disconnected)
            }
        }
    }
    /* Courtesy of museun
        - Check when last ping was sent
        - Respond to pong
        - Set timers
    */
    pub fn check_ping(&mut self) -> std::result::Result<String, String> {
        let jitter = thread_rng().gen_range(100, 300);

        if self.last_ping.elapsed()
            > Duration::from_millis(100 * 10 * 60 * 4) + Duration::from_millis(jitter)
        {
            self.socket
                .write_message(Message::Text(r#"{"type": "PING"}"#.to_string()))
                .unwrap();

            self.expected_pong = Some(Instant::now());
            self.last_ping = Instant::now();
        }

        // If pong timed out, stop listening and break the loop
        // TODO: We probably want to handle reconnecting: https://dev.twitch.tv/docs/pubsub#connection-management
        if let Some(dt) = self.expected_pong {
            if dt.elapsed() > self.pong_timeout {
                println!("PONG timed out");
                Err("PONG Timed out".to_string())
            } else {
                Ok("Recived PONG".to_string())
            }
        } else {
            Ok("".to_string())
        }
    }

    fn back_off(&mut self) {
        let max_back_off: Duration = Duration::from_secs(120);

        if let Some(last) = self.last_back_off {
            if last.elapsed() > self.back_off_timer && !(self.back_off_timer > max_back_off) {
                println!("Backing off for REEEEEEEEEEEEEEEEALZ");
                self.back_off_timer = self.back_off_timer * 2;
                self.socket = setup_socket(self.socket_url.to_string());
                self.send_listen_msg();

                self.last_back_off = Some(Instant::now());
            } else {
                println!("I GIBE UP 🤬");
            }
        } else {
            println!("No last back off");
            self.last_back_off = Some(Instant::now());
        }
    }
}

pub fn setup_socket(url: String) -> WebSocket<Stream<TcpStream, TlsStream<TcpStream>>> {
    // Create a websocket for twitch PubSub
    let (socket, _response) = connect(url).unwrap();

    // 1. Tear socket apart
    // 2. Set to non-blocking
    match socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();

    return socket;
}

pub fn create_twitch_pubsub_ws() -> TwidshPubSubBott {
    let channel_id = var("T_CHANNEL_ID").expect("Twitch channel id not found");
    let socket_url = "wss://pubsub-edge.twitch.tv";

    // If a client does not receive a PONG message within 10 seconds of issuing a PING command, it should reconnect to the server.
    let pong_timeout = Duration::from_secs(15);
    let expected_pong = None;
    let back_off_timer = Duration::from_secs(2);
    let last_back_off = None;

    let socket = setup_socket(socket_url.to_string());

    TwidshPubSubBott {
        channel_id,
        socket_url: socket_url.to_string(),
        pong_timeout,
        back_off_timer,
        last_back_off,
        expected_pong,
        socket,
        last_ping: Instant::now(),
    }
}

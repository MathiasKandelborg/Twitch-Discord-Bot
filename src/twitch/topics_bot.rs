use std::{rc::Rc, env::var};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use native_tls::TlsStream;
use tungstenite::stream::Stream;
use tungstenite::{Message, WebSocket};

use log::*;
use rand::{thread_rng, Rng};

pub mod channel_point_redemption;
pub mod follower;
use channel_point_redemption::points_redeemed;
use follower::new_follower;

use crate::{
    common_structs::{
        socket::*,
        twitch::pubsub_topics_msg::*,
    },
    nonce,
};

pub struct TwidshPubSubBott {
   pub channel_id: String,
   pub socket_url: String,
   pub socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
   pub last_back_off: Option<Instant>,
   pub back_off_timer: Duration,
   pub expected_pong: Option<Instant>,
   pub last_ping: Instant,
   pub pong_timeout: Duration,
   pub settings: Rc<config::Config>,
}

impl TwidshPubSubBott {
    pub fn main(&mut self) {
        // println!("Running main");
        if let Err(Disconnected) = self.read_message() {
            error!("Recieved Disconnect, backing off");
            self.back_off();
        } else {
            //   println!("Sending ping pong");
            if let Err(Disconnected) = self.ping_pong() {
                warn!("Recieved PubSub ping Disconnect, backing off");
                self.back_off();
            } else {
                //     println!("PubSub Ping successfull");
            }
        }
    }

    pub fn read_message(&mut self) -> Result<()> {
        if !self.socket.can_read() {
            error!("Cats can't read!!!");
            return Err(Disconnected);
        }
        if !self.socket.can_write() {
            error!("Can't write!!!");
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
                error!("Twitch PubSub WS disconnect:\n{}", err);
                return Err(Disconnected);
            }

            Ok(Message::Text(socket_res)) => {
                if socket_res.contains("PONG") {
                    self.expected_pong = None;

                    info!("Recived Twitch WS PONG!");
                }

                // println!("{}", res.trim());
                if socket_res.contains("data") {
                    let msg: TopicsResMetaMsg = serde_json::from_str(socket_res.as_str().into())
                        .expect("Could not deserialize meta msg");
                    // info!("{:#?}", &res_msg); // for debugging
                    let topic_str = &msg.data.topic;

                    match topic_str.as_str().split(".").collect::<Vec<&str>>()[0] {
                        "channel-points-channel-v1" => {
                            points_redeemed(&msg, self.settings.clone());
                        }

                        "following" => new_follower(&msg),
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

    pub fn generate_topics_msg(
        &mut self,
        nonce: String,
        channel_id: String,
        twitch_auth_token: String,
    ) -> TopicListenerMeta {
        TopicListenerMeta {
            event: "LISTEN".to_string(),
            nonce,
            data: TopicListenerData {
                topics: vec![
                    format!("channel-points-channel-v1.{}", &channel_id),
                    format!("following.{}", &channel_id),
                ],
                auth_token: twitch_auth_token,
            },
        }
    }

    pub fn send_listen_msg(&mut self) {
        let twitch_auth_token = var("T_AUTH_TOKEN").expect("Twitch auth token not found");

        // Generate Listen Message
        let t_ws_connect_msg = serde_json::to_string(&self.generate_topics_msg(
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

    pub fn ping_pong(&mut self) -> Result<()> {
        match &mut self.check_ping() {
            Ok(_msg) => {
                // println!("{}", msg);
                Ok(())
            }
            Err(err) => {
                error!("Twitch PubSub WS ERROR: {}", err);
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
        if let Some(dt) = self.expected_pong {
            if dt.elapsed() > self.pong_timeout {
                warn!("PONG timed out");
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
                info!("Backing off for REEEEEEEEEEEEEEEEALZ");
                self.back_off_timer = self.back_off_timer * 2;
                self.socket = setup_socket(self.socket_url.to_string());
                self.send_listen_msg();

                self.last_back_off = Some(Instant::now());
            } else {
                error!("I GIBE UP 🤬");
            }
        } else {
            info!("No last back off");
            self.last_back_off = Some(Instant::now());
        }
    }
}

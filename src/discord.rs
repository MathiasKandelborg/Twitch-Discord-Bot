use native_tls::TlsStream;
use std::env::var;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::stream::Stream;
use tungstenite::{Message, WebSocket};
use rand::{thread_rng, Rng};
use log::*;

use crate::common_structs::socket::setup_socket;
pub mod parse_message;
pub use parse_message::*;
use parse_message::{Disconnected, Result};


pub struct DiscordBot {
    pub bot_token: String,
    pub ws_url: String,
    pub session_id: String,
    pub socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
    pub reconnect_duration: Duration,
    pub last_reconnect_try: Option<Instant>,
    pub heartbeat_interval: Duration,
    pub last_heartbeat: Instant,
    pub last_sequence: Option<i64>,
}

impl DiscordBot {
    pub fn resume(&mut self) -> Result<()> {
        let seq = self.last_sequence;
        let resume_msg = r#"{"op": 6,"d": {"token": ""#.to_string()
            + self.bot_token.as_str()
            + r#"","session_id": ""#
            + self.session_id.as_str()
            + r#"","seq": "#
            + seq.unwrap_or(0).to_string().as_str()
            + r#"}"#;

        match self.socket.write_message(Message::Text(resume_msg)) {
            Err(err) => {
                error!("Failed to write resume message:\n{}", err);
                return Err(Disconnected);
            }

            Ok(_) => {
                info!("Sending resume message");
            }
        }
        Ok(())
    }

    pub fn main(&mut self) {
        if let Err(Disconnected) = self.read_message() {
            error!("Discord read message disconnect");

            self.socket = setup_socket(self.ws_url.to_string());
            self.setup_again();
        } else {
            if let Err(Disconnected) = self.send_heartbeat() {}
        };
    }

    pub fn read_message(&mut self) -> Result<()> {
        if !self.socket.can_read() {
            error!("Discord Cats can't read!!!");
            return Err(Disconnected);
        }
        if !self.socket.can_write() {
            error!("Discord can't write!!!");
            return Err(Disconnected);
        }

        match self.socket.read_message() {
            Err(tungstenite::error::Error::Io(err))
                if err.kind() == std::io::ErrorKind::WouldBlock =>
            {
                // we're blocking
                // it's a faaaaaake
            }
            Err(err) => {
                error!("Discord WS Error MSG:\n{}", err);
                return Err(Disconnected);
            }
            Ok(Message::Text(res)) => {
                let msg = DiscordMsgParser::parse(self, res.as_str());
                if let Err(Disconnected) = msg {
                    error!("Failed to parse Discord msg");
                    return Err(Disconnected);
                } else {
                    info!("Discord msg parsing successful");
                    return Ok(());
                }
            }
            Ok(..) => {}
        }
        Ok(())
    }

    pub fn send_heartbeat(&mut self) -> Result<()> {
        /*  Used to maintain an active gateway connection.
        Must be sent every heartbeat_interval milliseconds after the Opcode 10 Hello payload is received.
        The inner d key is the last sequence number—s—received by the client.
        If you have not yet received one, send null. */
        let heartbeat = r#"
            {
                "op": 1,
                "d": null
            }"#;

        if self.last_heartbeat.elapsed() >= self.heartbeat_interval {
            match self
                .socket
                .write_message(Message::Text(heartbeat.to_string()))
            {
                Err(err) => {
                    error!("Discord WS Write failed: {}", err);
                    return Err(Disconnected);
                }
                Ok(_) => {
                    info!("My heart beats");
                    self.last_heartbeat = Instant::now();
                }
            }

            Ok(())
        } else {
            Err(Disconnected)
        }
    }

    pub fn setup_again(&mut self) {
        info!("Establishing new Discord connection");

        if let Some(dur) = self.last_reconnect_try {
            if dur.elapsed() >= self.reconnect_duration {
                self.socket = setup_socket(self.ws_url.to_string());

                self.setup();
            }
        } else {
            self.last_reconnect_try = Some(Instant::now());
        }
    }

    pub fn setup(&mut self) {
        let connect_msg = r#"
{
  "op": 2,
  "d": {
    "token": ""#
            .to_string()
            + &self.bot_token.to_string()
            + r#"",
    "intents": 513,
    "properties": {
      "$os": "linux",
      "$browser": "NeonBot",
      "$device": "NeonBot"
    }
  }
}"#;
        loop {
            match self.socket.read_message() {
                Err(tungstenite::error::Error::Io(err))
                    if err.kind() == std::io::ErrorKind::WouldBlock =>
                {
                    // it's a faaaaaake
                    // we're blocking
                }
                Err(err) => {
                    error!("Discord errr {}", err);
                }

                Ok(Message::Text(res)) => {
                    // println!("{}", res.trim());
                    if res.contains(r#""op":10"#) {
                        let hb_msg: DiscordHeatBeat = serde_json::from_str(res.trim())
                            .expect("Discord Heartbeat could not be deserialized");

                        self.heartbeat_interval =
                            Duration::from_millis(hb_msg.d.heartbeat_interval);

                        info!("Identifying with Discord after heartbeart");

                        self.socket
                            .write_message(Message::Text(connect_msg.to_string()))
                            .expect("Could not write discord connect message");

                        break;
                    }
                }
                Ok(..) => {
                    info!("");
                }
            }
        }
    }
}

pub fn create_discord_bot() -> DiscordBot {
    let bot_token = var("D_BOT_TOKEN").unwrap();
    let url = "wss://gateway.discord.gg/?v=8&encoding=json".to_string();
    let socket = setup_socket(url.to_string());
    let jitter = thread_rng().gen_range(1000, 5000);
    let reconnect_duration = Duration::from_millis(jitter);

    DiscordBot {
        session_id: "".to_string(),
        ws_url: url,
        reconnect_duration,
        last_reconnect_try: None,
        bot_token,
        socket,
        last_heartbeat: Instant::now(),
        heartbeat_interval: Duration::from_millis(44500),
        last_sequence: None,
    }
}

use config::Config;
use native_tls::TlsStream;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::stream::Stream;
use tungstenite::{Message, WebSocket};
use log::*;

pub mod command_parser;
pub mod msg_parser;
pub mod send_msg;

use msg_parser::parse_twitch_msg;
use command_parser::chat_commands;

use crate::common_structs::socket::{Disconnected, Result, setup_socket};

pub struct TwidshTshadBott {
   pub socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
   pub oauth_token: String,
   pub socket_url: String,
   pub last_back_off: Option<Instant>,
   pub back_off_timer: Duration,
}

impl TwidshTshadBott {
    pub fn main(&mut self, commands: &Config) {
        if let Err(Disconnected) = self.read_message(commands) {
            self.back_off()
        } else {
            // println!("Chat read msg successful");
        }
    }

    pub fn send_ping(&mut self) -> Result<()> {
        // Send PONG if Twitch is going PING
        // println!("Recived Twitch Chat PING! Sent PONG!");

        let send_pong = self
            .socket
            .write_message(Message::Text("PONG :tmi.twitch.tv".to_string()));

        match send_pong {
            Ok(_) => {
                info!("Sent Twitch chat pong");
                Ok(())
            }
            Err(err) => {
                error!("Could not write pong msg to twitch:\n {}\n", err);
                Err(Disconnected)
            }
        }
    }

    pub fn read_message(&mut self, commands: &Config) -> Result<()> {
        if !self.socket.can_read() {
            error!("Chat Cats can't read!!!");
            return Err(Disconnected);
        }
        if !self.socket.can_write() {
            error!("Can't write!!!");
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
                error!("TWITCH CHAT WS ERROR MSG:\n{}", err);
                return Err(Disconnected);
            }
            Ok(Message::Text(res)) => {
                if res.contains("PING") {
                    if let Err(Disconnected) = self.send_ping() {
                        return Err(Disconnected);
                    }
                }

                // println!("{}", res.trim()); // For debugging
                match parse_twitch_msg(res) {
                    Some(msg) => {
                        info!(
                            "#{} <{}>: \"{}\"",
                            msg.channel_name,
                            msg.display_name,
                            msg.message.trim()
                        );
                        // Respond to commands
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
                info!("Backing off chat for REEEEEEEEEEEEEEEEALZ");
                self.back_off_timer = self.back_off_timer * 2;
                self.socket = setup_socket(self.socket_url.to_string());
                self.send_listen_msg();

                self.last_back_off = Some(Instant::now());
            } else {
                info!("I GIBE UP ON CHAT 🤬");
            }
        } else {
            info!("No last chat back off");
            self.last_back_off = Some(Instant::now());
        }
    }
}

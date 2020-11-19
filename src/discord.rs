use native_tls::TlsStream;
use serde::{Deserialize, Serialize};
use std::env::var;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::stream::Stream;
use tungstenite::{connect, Message, WebSocket};

#[derive(Deserialize, Serialize)]
struct HeatBeatMessage {
    op: u16,
    d: Option<i16>,
}

#[derive(Deserialize, Serialize)]
struct HeartBeatData {
    heartbeat_interval: u64,
}

#[derive(Deserialize, Serialize)]
struct DiscordHeatBeat {
    op: u8,
    d: HeartBeatData,
}

pub struct DiscordBot {
    pub bot_token: String,
    pub heartbeat_interval: Duration,
    pub session_id: String,
    pub socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
    pub last_heartbeat: Instant,
}

impl DiscordBot {
    pub fn send_heartbeat(&mut self) {
        /*  Used to maintain an active gateway connection.
        Must be sent every heartbeat_interval milliseconds after the Opcode 10 Hello payload is received.
        The inner d key is the last sequence number—s—received by the client.
        If you have not yet received one, send null. */
        let heartbeat = r#"
            {
                "op": 1,
                "d": null
            }"#;

        let heatbeat_msg: HeatBeatMessage = serde_json::from_str(heartbeat).unwrap();

        if self.last_heartbeat.elapsed() >= self.heartbeat_interval {
            println!("My heart beats");

            self.socket
                .write_message(Message::Text(serde_json::to_string(&heatbeat_msg).unwrap()))
                .unwrap();

            self.last_heartbeat = Instant::now();
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
                    let _ = dbg!("Discord errr {}", err);
                }

                Ok(Message::Text(res)) => {
                    if res.contains(r#""op":10"#) {
                        let hb_msg: DiscordHeatBeat = serde_json::from_str(res.trim())
                            .expect("Discord Heartbeat could not be deserialized");

                        self.heartbeat_interval =
                            Duration::from_millis(hb_msg.d.heartbeat_interval);

                        println!("Sending connect after receiving heartbeart");

                        self.socket
                            .write_message(Message::Text(connect_msg.to_string()))
                            .expect("Could not write discord connect message");

                        break;
                    }
                }
                Ok(..) => {
                    print!("");
                }
            }
            std::thread::sleep(Duration::from_millis(120));
        }
    }
}

pub fn create_discord_bot() -> DiscordBot {
    let bot_token = var("D_BOT_TOKEN").unwrap();

    let (socket, _res) = connect("wss://gateway.discord.gg/?v=8&encoding=json").unwrap();

    // Same as above TODO: abstract this
    match socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();

    DiscordBot {
        session_id: "".to_string(),
        bot_token,
        socket,
        last_heartbeat: Instant::now(),
        heartbeat_interval: Duration::from_millis(44500),
    }
}

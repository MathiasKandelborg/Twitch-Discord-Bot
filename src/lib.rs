pub mod common_structs;
use crate::common_structs::{DataObj, TopicListener};

pub mod channel_points_redemption;
pub mod new_follower;
pub mod parse_twitch_msg;
use crate::parse_twitch_msg::TwitchChatMsg;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(18).collect()
}

pub fn generate_listen_msg(
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

use std::time::{Duration, Instant};
use native_tls::TlsStream;
use std::net::TcpStream;
use tungstenite::stream::Stream;
use tungstenite::{Message, WebSocket};

pub fn check_ping(
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
        *expected_pong = Some(Instant::now());
        *last_ping = Instant::now();
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

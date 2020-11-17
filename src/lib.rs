#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
pub mod common_structs;
use crate::common_structs::{DataObj, TopicListener};

pub mod channel_points_redemption;
pub mod chat_command;
pub mod new_follower;
pub mod parse_twitch_msg;
pub mod send_msg;
pub mod twitch_chat_connect;

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

use native_tls::TlsStream;
use std::net::TcpStream;
use std::time::{Duration, Instant};
use tungstenite::stream::Stream;
use tungstenite::{Message, WebSocket};

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
    if last_ping.elapsed() > Duration::from_millis(60 * 1000) {
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

use std::env::var;
use std::time::{Duration, Instant};

pub mod chat_bot;
pub mod topics_bot;
use chat_bot::TwidshTshadBott;
use topics_bot::TwidshPubSubBott;

use crate::common_structs::socket::setup_socket;

use log::*;

pub fn setup_twitch_chat_ws() -> TwidshTshadBott {
    let url = "wss://irc-ws.chat.twitch.tv:443";
    let oauth_token = var("T_OAUTH_TOKEN").expect("Twitch chat token not found");

    let back_off_timer = Duration::from_secs(2);
    let last_back_off = None;

    info!("Setting up Twitch Chat WS");
    let socket = setup_socket(url.to_string());

    TwidshTshadBott {
        socket,
        socket_url: url.to_string(),
        oauth_token,
        back_off_timer,
        last_back_off,
    }
}

pub fn create_twitch_pubsub_ws() -> TwidshPubSubBott {
    let channel_id = var("T_CHANNEL_ID").expect("Twitch channel id not found");
    let socket_url = "wss://pubsub-edge.twitch.tv";

    // If a client does not receive a PONG message within 10 seconds of issuing a PING command, it should reconnect to the server.
    let pong_timeout = Duration::from_secs(15);
    let expected_pong = None;
    let back_off_timer = Duration::from_secs(2);
    let last_back_off = None;

    info!("Setting up Twitch PubSub Topics WS");
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

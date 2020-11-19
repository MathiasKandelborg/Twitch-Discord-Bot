#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
pub mod common_structs;

pub mod channel_points_redemption;
pub mod chat_command;
pub mod new_follower;
pub mod parse_twitch_msg;
pub mod send_msg;
pub mod twitch_chat_connect;
pub mod discord;
pub mod twitch;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(18).collect()
}

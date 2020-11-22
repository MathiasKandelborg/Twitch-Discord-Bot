#![allow(unused_imports)]
use serde::{Deserialize, Serialize};

use notify_rust::Hint;
#[cfg(all(
    feature = "images",
    unix,
    not(target_os = "macos"),
    not(target_os = "windows")
))]
use notify_rust::Image;
use notify_rust::Notification;

use std::process::Command;

use crate::common_structs::{MetaMsg, Res};

#[derive(Deserialize, Serialize)]
struct NewFollower {
    display_name: String,
    username: String,
    user_id: String,
}

pub fn new_follower(res_msg: &Res) {
    let new_follower: NewFollower =
        serde_json::from_str(res_msg.data.message.as_str()).expect("Could not deserialize Twitch new follower");

    println!("{}", serde_json::to_string(&new_follower).expect(""));

    #[cfg(any(macos, windows))]
    Notification::new()
        .summary(format!("Tron awaits you, {}!", &new_follower.display_name).as_str())
        .body("YOU ARE AMAZING! 🥰")
        .unwrap()
        .show()
        .unwrap();


    #[cfg(linux)]
    Notification::new()
        .summary(format!("Tron awaits you, {}!", &new_follower.display_name).as_str())
        .body("YOU ARE AMAZING! 🥰")
        .image("/home/mathias/Pictures/hackerman.jpg")
        .unwrap()
        .show()
        .unwrap();
}

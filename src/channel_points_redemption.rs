#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(unused_imports)]
use notify_rust::Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image;
use notify_rust::Notification;

use log::*;

use std::process::Command;

use crate::{
    common_structs::{Msg, Res},
    notif,
};

pub fn channel_points_redemption(res_msg: &Res) {
    println!("Channgel points redeemed!!");
    let redemption_msg: Msg = serde_json::from_str(&res_msg.data.message.to_string())
        .expect("Could not deserialize Channel Points data message");

    let redemption_title = &redemption_msg.data.redemption.reward.title;

    // logging redemtion [info]
    log!(format!("<{}> redeemed {}", &redemption_msg.data.redemption.user, redemption_title));

    if redemption_title.contains("Hydrate!") {
        // Send notification for hydration events
        notif!(
            &redemption_title,
            &redemption_msg.data.redemption.reward.prompt,
            "/home/mathias/Pictures/water.jpg",
        );
    }

    if redemption_title.contains("Initiate") {
        notif!(
            &redemption_title,
            &redemption_msg.data.redemption.reward.prompt,
            "/home/mathias/Pictures/terminal.jpg",
        );

        Command::new("sh")
            .arg("-c")
            .arg("cool-retro-term")
            .spawn()
            .expect("Failed to execute Futuristic retro term D:");
    }
}

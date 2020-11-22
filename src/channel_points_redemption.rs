#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(unused_imports)]
use notify_rust::Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image;
use notify_rust::Notification;

use log::*;

use std::process::Command;

use crate::common_structs::{Res, Msg};

pub fn channel_points_redemption(res_msg: &Res) {
    println!("Channgel points redeemed!!");
    let redemption_msg: Msg = serde_json::from_str(&res_msg.data.message.to_string())
        .expect("Could not deserialize Channel Points data message");

    let redemption_title = &redemption_msg.data.redemption.reward.title;

    // logging redemtion [info]
    log!(format!("<{}> redeemed {}", &redemption_msg.data.redemption.user, redemption_title));

    if redemption_title.contains("Hydrate!") {
        // Send notification for hydration events
        Notification::new()
            .summary(&redemption_title)
            .body(&redemption_msg.data.redemption.reward.prompt)
            .image("/home/mathias/Pictures/water.jpg")
            .unwrap()
            .show()
            .unwrap();
    }

    if redemption_title.contains("Initiate") {
        Notification::new()
            .summary(&redemption_title)
            .body(&redemption_msg.data.redemption.reward.prompt)
            .image("/home/mathias/Pictures/terminal.jpg")
            .unwrap()
            .show()
            .unwrap();

        Command::new("sh")
            .arg("-c")
            .arg("cool-retro-term")
            .spawn()
            .expect("Failed to execute Futuristic retro term D:");

    }
}

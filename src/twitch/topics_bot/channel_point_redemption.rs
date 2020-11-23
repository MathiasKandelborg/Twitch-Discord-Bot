#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(unused_imports)]
use notify_rust::Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image;
use notify_rust::Notification;

use log::*;
use simplelog::*;

use std::process::Command;

use crate::{
    common_structs::twitch::pubsub_topics_msg::{TopicsResMsg, TopicsResMetaMsg},
    notif,
};


pub fn points_redeemed(msg: &TopicsResMetaMsg) {
    let redemption_msg: TopicsResMsg = serde_json::from_str(&msg.data.message.to_string())
        .expect("Could not deserialize Channel Points data message");

    let redemption_title = &redemption_msg.data.redemption.reward.title;

    info!("Channgel points redeemed!!");
    // logging redemtion [info]
    info!("<{}> redeemed {}", &redemption_msg.data.redemption.user.display_name, redemption_title);

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

        Command::new("cool-retro-term")
            .spawn()
            .expect("Failed to execute Futuristic retro term D:");
    }
}

#![allow(unused_imports)]
use notify_rust::Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image;
use notify_rust::Notification;

use std::process::Command;

use crate::common_structs::{ChannelPointsRes, ChannelPointsMsg};

pub fn channel_points_redemption(res_msg: &ChannelPointsRes) {
    let redemption_msg: ChannelPointsMsg = serde_json::from_str(&res_msg.data.message.to_string())
        .expect("Could not deserialize Channel Points data message");

    println!("testubg");
    let redemption_title = &redemption_msg.data.redemption.reward.title;

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
            .output()
            .expect("failed to execute process");
    }
}

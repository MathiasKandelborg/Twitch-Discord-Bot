#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(unused_imports)]
use notify_rust::Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image;
use notify_rust::Notification;

use std::{process::Command, path::Path};

use crate::common_structs::{Res, Msg};

pub fn channel_points_redemption(res_msg: &Res) {
    println!("Channgel points redeemed!!");
    let redemption_msg: Msg = serde_json::from_str(&res_msg.data.message.to_string())
        .expect("Could not deserialize Channel Points data message");

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

    if redemption_title.contains("Suggest Side") {
        let pathForSides = Path::new("sides.txt");
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let sideSuggestion = format!("[{}] <{}>: {}", redemption_msg.data.timestamp, redemption_msg.data.redemption.user, redemption_msg.data.redemption.reward.prompt)

        match file.write_all(sideSuggestion.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
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

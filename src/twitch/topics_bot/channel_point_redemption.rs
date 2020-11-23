#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(unused_imports)]
use notify_rust::Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image;
use notify_rust::Notification;

use log::*;
use simplelog::*;

use std::{collections::HashMap, env::temp_dir, fs::File, path::Path, path::PathBuf, process::Command, rc::Rc, io::Write};

use crate::{
    common_structs::twitch::pubsub_topics_msg::{TopicsResMetaMsg, TopicsResMsg},
    notif,
};

pub fn points_redeemed(msg: &TopicsResMetaMsg, settings: Rc<config::Config>) {
    let redemption_msg: TopicsResMsg = serde_json::from_str(&msg.data.message.to_string())
        .expect("Could not deserialize Channel Points data message");

    let redemption_title = &redemption_msg.data.redemption.reward.title;

    info!("Channgel points redeemed!!");
    // logging redemtion [info]
    info!(
        "<{}> redeemed {}",
        &redemption_msg.data.redemption.user.display_name, redemption_title
    );

    if redemption_title.contains("Hydrate!") {
        // Send notification for hydration events
        notif!(
            &redemption_title,
            &redemption_msg.data.redemption.reward.prompt,
            "/home/mathias/Pictures/water.jpg",
        );
    }

    // [SuggestSide] - redemtion, will add a side, if redeemed, to a specified file
    // if file does not exist it will create file, otherwise it will truncate to it
    if redemption_title.contains("Suggest Side") {
        // Check if a file path has been supplied, if not use a default path
        let filepath = match settings.get_str("side_suggestions_file") {
            Ok(value) => PathBuf::from(value),
            Err(e) => {
                error!("Error loading config: {}", e);
                // If there is no specified path, we will create a temporary directory and push a default file name in there
                let mut tempdir = temp_dir();
                tempdir.push("sides.txt");
                tempdir
            }
        };
        // Creating the path to the file
        let pathForSides = Path::new(&filepath);
        let display = pathForSides.display();
        // creating/ opening the file in write mode
        match File::create(pathForSides) {
            Err(why) => error!("error: [suggest side] couldn't create {}: {}", display, why),
            Ok(mut file) => {
                // format side suggestion message
                let sideSuggestion = format!(
                    "[{}] <{}>: {}\n",
                    redemption_msg.data.redemption.redeemed_at,
                    redemption_msg.data.redemption.user.display_name,
                    redemption_msg.data.redemption.reward.prompt
                );
                // writing side suggestion to file
                match file.write_all(sideSuggestion.as_bytes()) {
                    Err(why) => error!(
                        "error: [suggest side] couldn't write to {}: {}",
                        display, why
                    ),
                    Ok(_) => info!(
                        "[suggest side] successfully wrote suggestion from <{}> to {}",
                        redemption_msg.data.redemption.user.display_name, display
                    ),
                }
            }
        };
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

#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(unused_imports)]
use notify_rust::Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image;
use notify_rust::Notification;

use std::{env::temp_dir, path::Path, process::Command, path::PathBuf};

use crate::common_structs::{Res, Msg};

pub fn channel_points_redemption(res_msg: &Res, settings: &Config) {
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

    // [SuggestSide] - redemtion, will add a side, if redeemed, to a specified file
    // if file does not exist it will create file, otherwise it will truncate to it
    if redemption_title.contains("Suggest Side") {
        // Check if a file path has been supplied, if not use a default path
        let _settingsMap = settings.try_into::<HashMap<String, String>>().unwrap();
        let filepath = match _settingsMap.get("side_suggestions_file") {
            Some(value) => PathBuf::new().push(value),
            None => {
                // If there is no specified path, we will create a temporary directory and push a default file name in there
                let mut tempdir = temp_dir();
                tempdir.push("sides.txt");
                tempdir
            },
        };
        // Creating the path to the file
        let pathForSides = Path::new(&filepath);
        let display = path.display();
        // creating/ opening the file in write mode
        let mut file = match File::create(&path) {
            Err(why) => println!("error: [suggest side] couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        // format side suggestion message
        let sideSuggestion = format!("[{}] <{}>: {}\n", redemption_msg.data.timestamp, redemption_msg.data.redemption.user, redemption_msg.data.redemption.reward.prompt)
        // writing side suggestion to file
        match file.write_all(sideSuggestion.as_bytes()) {
            Err(why) => println!("error: [suggest side] couldn't write to {}: {}", display, why),
            Ok(_) => println!("[suggest side] successfully wrote suggestion from <{}> to {}", redemption_msg.data.redemption.user, display),
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

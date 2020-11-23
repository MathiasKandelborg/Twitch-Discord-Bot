use log::*;
use serde::{Deserialize, Serialize};

use crate::{common_structs::twitch::pubsub_topics_msg::TopicsResMetaMsg, notif};

#[derive(Deserialize, Serialize)]
struct NewFollower {
    display_name: String,
    username: String,
    user_id: String,
}

pub fn new_follower(res_msg: &TopicsResMetaMsg) {
    let new_follower: NewFollower = serde_json::from_str(res_msg.data.message.as_str())
        .expect("Could not deserialize Twitch new follower");

    info!("{}", serde_json::to_string(&new_follower).expect(""));

    notif!(
        format!("Tron awaits you, {}!", &new_follower.display_name).as_str(),
        "YOU ARE AMAZING! 🥰",
        "/home/mathias/Pictures/hackerman.jpg",
    );
}

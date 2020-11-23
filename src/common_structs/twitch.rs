pub mod pubsub_topics_msg {
    use super::redemption_structs::RedemptionData;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct TopicListenerData {
        pub topics: Vec<String>,
        pub auth_token: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TopicListenerMeta {
        #[serde(rename = "type")]
        pub event: String,
        pub nonce: String,
        pub data: TopicListenerData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TopicsMsg {
        pub topic: String,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TopicsResMetaMsg {
        #[serde(rename = "type")]
        pub event: String,
        pub data: TopicsMsg,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TopicsResMsg {
        #[serde(rename = "type")]
        pub event: String,
        pub data: RedemptionData,
    }
}

pub mod redemption_structs {
    pub use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RewardMaxPerStream {
        pub is_enabled: bool,
        pub max_per_stream: u32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RewardImage {
        pub url_1x: String,
        pub url_2x: String,
        pub url_4x: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RewardDefaultImage {
        pub url_1x: String,
        pub url_2x: String,
        pub url_4x: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RedemptionUser {
        pub id: String,
        pub login: String,
        pub display_name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RedemptionReward {
        channel_id: String,
        pub title: String,
        pub prompt: String,
        pub cost: i32,
        pub is_user_input_required: bool,
        pub is_sub_only: bool,
        pub default_image: RewardDefaultImage,
        pub background_color: String,
        pub is_enabled: bool,
        pub is_paused: bool,
        pub is_in_stock: bool,
        pub max_per_stream: RewardMaxPerStream,
        pub should_redemptions_skip_request_queue: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Redemption {
        pub id: String,
        pub user: RedemptionUser,
        channel_id: String,
        pub redeemed_at: String,
        pub reward: RedemptionReward,
        #[serde(default)]
        pub user_input: Option<String>,
        pub status: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RedemptionData {
        timestamp: String,
        pub redemption: Redemption,
    }
}

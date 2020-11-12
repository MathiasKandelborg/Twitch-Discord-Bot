pub use self::meta::*;
pub use self::channel_points::*;

pub mod channel_points {
    pub use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsRewardMaxPerStream {
        pub is_enabled: bool,
        pub max_per_stream: i16,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsRewardImage {
        pub url_1x: String,
        pub url_2x: String,
        pub url_4x: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsRewardDefaultImage {
        pub url_1x: String,
        pub url_2x: String,
        pub url_4x: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsRedemptionUser {
        pub id: String,
        pub login: String,
        pub display_name: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsRedemptionReward {
        channel_id: String,
        pub title: String,
        pub prompt: String,
        pub cost: i16,
        pub is_user_input_required: bool,
        pub is_sub_only: bool,
        pub default_image: ChannelPointsRewardDefaultImage,
        pub background_color: String,
        pub is_enabled: bool,
        pub is_paused: bool,
        pub is_in_stock: bool,
        pub max_per_stream: ChannelPointsRewardMaxPerStream,
        pub should_redemptions_skip_request_queue: bool,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsRedemption {
        pub id: String,
        pub user: ChannelPointsRedemptionUser,
        channel_id: String,
        pub redeemed_at: String,
        pub reward: ChannelPointsRedemptionReward,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub user_input: Option<String>,
        pub status: String,
    }
}

pub mod meta {
    use serde::{Deserialize, Serialize};
    use super::ChannelPointsRedemption;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsMsgData {
        timestamp: String,
        pub redemption: ChannelPointsRedemption,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsMetaMsg {
        pub topic: String,
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsMsg {
        #[serde(rename = "type")]
        pub event: String,
        pub data: ChannelPointsMsgData,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChannelPointsRes {
        #[serde(rename = "type")]
        pub event: String,
        pub data: ChannelPointsMetaMsg,
    }
}

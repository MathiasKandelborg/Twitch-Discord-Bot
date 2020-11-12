pub mod common_structs;
use crate::common_structs::{DataObj, TopicListener};



use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(18).collect()
}

pub fn generate_listen_msg(
    nonce: String,
    channel_id: String,
    twitch_auth_token: String,
) -> TopicListener {
    TopicListener {
        event: "LISTEN".to_string(),
        nonce,
        data: DataObj {
            topics: vec![
                format!("channel-points-channel-v1.{}", &channel_id),
                format!("following.{}", &channel_id),
            ],
            auth_token: twitch_auth_token,
        },
    }
}

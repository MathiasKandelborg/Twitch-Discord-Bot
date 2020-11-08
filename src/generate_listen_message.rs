pub mod channel_points_reward_msg;
pub use channel_points_reward_msg::listen_msg_structs::{TopicListener,DataObj};


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

    // Return the structured listen message
}

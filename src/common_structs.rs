pub mod channel_points;
pub use channel_points::*;

pub use self::ws_structs::*;
pub mod ws_structs {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct DataObj {
        pub topics: Vec<String>,
        pub auth_token: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TopicListener {
        #[serde(rename = "type")]
        pub event: String,
        pub nonce: String,
        pub data: DataObj,
    }
}


#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
pub mod common_structs;

pub mod channel_points_redemption;
pub mod chat_command;
pub mod discord;
pub mod new_follower;
pub mod parse_twitch_msg;
pub mod send_msg;
pub mod twitch;
pub mod twitch_chat_connect;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

/// Displays a notification with the given summary, body and optionally an image
/// images are not supported on win and mac and will be ignored
/// rust-analyzer on win says that a macro call to this is disabled, but that is incorrect as only the image part is
#[macro_export]
macro_rules! notif {
    (
        $summary:expr,
        $body:expr,
        $($image:expr)?$(,)?
    ) => {{
        let mut notif = notify_rust::Notification::new();
        notif.summary($summary);
        notif.body($body);

        $(
            #[cfg(all(unix, not(target_os = "macos")))]
            notif.image($image).unwrap();
        )?

        notif.show().unwrap();
    }};
}

pub fn nonce() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(18).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn notif() {
        notif!(
            "Testing",
            "More testing!",
            "/home/mathias/Pictures/hackerman.jpg",
        );
        
        // should work with no image
        notif!(
            "Testing",
            "More testing!",
        );
    }
}

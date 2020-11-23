/* Courtesy of Togglebit, the parser master! */

pub struct TwitchChatMsg {
    pub display_name: String,
    pub channel_name: String,
    pub message: String,
}

// Takes pre-parsed WS Text::Message from twitch chat
pub fn parse_twitch_msg(chat_msg: String) -> Option<TwitchChatMsg> {
    if chat_msg.contains("PRIVMSG") {
        // Parse tags
        let tag_idx = chat_msg.to_string().find(" :").unwrap();
        let tag_arr = chat_msg.to_owned().drain(..tag_idx + 1).collect::<String>();

        // Create key pair values for each tag
        let tag_values = tag_arr
            .split(";")
            .map(|tags| tags.split("=").collect::<Vec<_>>());

        // Parse message
        let message = chat_msg.splitn(3, " :").last();

        // Parse channel name
        let priv_msg = &chat_msg.split("PRIVMSG").collect::<Vec<&str>>()[1].to_string();
        let channel_name = priv_msg.split(" :").collect::<Vec<&str>>()[0];

        // Parse user name
        let display_name = tag_values
            .filter_map(|tag| {
                if tag[0].contains("display-name") {
                    Some(tag[1])
                } else {
                    None
                }
            })
            .collect::<String>();

        // println!("{}", format!("User {} \nWrote: {}\nIn the {} channel", display_name, message.unwrap(), channel_name)); // For debugging

        return Some(TwitchChatMsg {
            display_name,
            channel_name: channel_name.to_string(),
            message: message.unwrap().to_string(),
        });
    } else {
        return None;
    }
}

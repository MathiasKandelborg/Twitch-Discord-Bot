pub mod chat_commands {
    use crate::twitch::chat_bot::msg_parser::TwitchChatMsg;
    use crate::twitch::chat_bot::send_msg::send_msg;

    use config::Config;
    use native_tls::TlsStream;
    use std::collections::HashMap;
    use std::net::TcpStream;
    use tungstenite::stream::Stream;
    use tungstenite::WebSocket;
    use log::*;

    pub fn cmd_response(
        msg: TwitchChatMsg,
        socket: &mut WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
        commands: &Config,
    ) {

        let parsed_commands = commands
            .to_owned()
            .try_into::<HashMap<String, String>>()
            .expect("Could not parse commands file");

        info!("<{}>: {}", msg.display_name, msg.message);
        for (command_key, command_res) in parsed_commands.iter() {
            if msg.message.trim().eq(command_key) {
                send_msg(socket, &msg.channel_name, command_res.to_string());
            }
        }

    }
}

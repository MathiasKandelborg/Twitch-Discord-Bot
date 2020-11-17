use native_tls::TlsStream;
use std::net::TcpStream;
use tungstenite::stream::Stream;
use tungstenite::{Message, WebSocket};

/* To keep the server from closing the connection, clients must send a PING command at least once every 5 minutes.

 Clients must LISTEN on at least one topic within 15 seconds of establishing the connection, or they will be disconnected by the server.

 Clients may receive a RECONNECT message at any time.
 This indicates that the server is about to restart (typically for maintenance) and will disconnect the client within 30 seconds.

 During this time, we recommend that clients reconnect to the server; otherwise, the client will be forcibly disconnected.
*/
pub fn twitch_chat_connect_messages(
    ws_twitch_chat: &mut WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
    twitch_oauth_token: &String,
) {
    ws_twitch_chat
        .write_message(Message::Text(
            "CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership".to_string(),
        ))
        .unwrap();
    ws_twitch_chat
        .write_message(Message::Text(format!("PASS {}", twitch_oauth_token)))
        .unwrap();

    ws_twitch_chat
        .write_message(Message::Text(format!("NICK idontmatterlol")))
        .unwrap();

    ws_twitch_chat
        .write_message(Message::Text(format!("JOIN #neonraytracer")))
        .unwrap();
}

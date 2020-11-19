use native_tls::TlsStream;
use std::net::TcpStream;
use tungstenite::stream::Stream;
use tungstenite::{Message, WebSocket};

pub fn send_msg(
    ws_chat: &mut WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
    channel_id: &String,
    msg: String
) {
    // PRIVMSG #<channel> :This is a sample message
    let msg_id = format!("PRIVMSG{} :", channel_id);

    let msg = format!("{}{}", msg_id, msg);

    println!("Sending message:\n{}\n", &msg);
    ws_chat.write_message(Message::Text(msg)).unwrap()
}

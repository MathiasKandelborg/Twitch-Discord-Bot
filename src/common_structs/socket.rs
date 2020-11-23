use std::net::TcpStream;
use native_tls::TlsStream;
use tungstenite::stream::Stream;
use tungstenite::{connect, WebSocket};

pub struct Disconnected;

pub type Result<T> = std::result::Result<T, Disconnected>;

pub fn setup_socket(url: String) -> WebSocket<Stream<TcpStream, TlsStream<TcpStream>>> {
    // Create a websocket for twitch PubSub
    let (socket, _response) = connect(url).unwrap();

    // 1. Tear socket apart
    // 2. Set to non-blocking
    match socket.get_ref() {
        tungstenite::stream::Stream::Plain(s) => s,
        tungstenite::stream::Stream::Tls(s) => s.get_ref(),
    }
    .set_nonblocking(true)
    .unwrap();

    return socket;
}



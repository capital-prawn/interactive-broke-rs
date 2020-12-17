//! client contains things related to or needed by EClient. This is what you should work with to establish a connection with the server, and not
//! a TCP connection directly

use crate::socket::IBSocket;
use std::collections::VecDeque;

type Message = String;
/// EClient is a struct representing a client that is connected to the server. It handles the message queue, sending messages, and other lower-level
/// details related to sending/receiving messages. It contains the TCP connection struct.
pub struct EClient {
    msg_queue: VecDeque<Message>,
    host: String,
    port: usize,
    conn: IBSocket,
    extra_auth: bool,
    client_id: Option<usize>,
    server_version: Option<String>,
    connection_state: ConnectionState,
    asynchronous: bool,
}

impl EClient {
    pub fn new<S: Into<String>>(hostname: S, port: usize) -> EClient {
        let hostname = hostname.into();
        EClient {
            msg_queue: VecDeque::new(),
            host: hostname.clone(),
            port: port,
            conn: IBSocket::new(hostname, port as u16),
            extra_auth: false,
            client_id: None,
            server_version: None,
            asynchronous: false,
            connection_state: ConnectionState::Disconnected,
        }
    }
}
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Redirect,
}

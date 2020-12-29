//! client contains things related to or needed by EClient. This is what you should work with to establish a connection with the server, and not
//! a TCP connection directly

use anyhow::*;

use crate::socket::IBSocket;
use std::collections::VecDeque;
use crate::message;
use crate::message::IBField;
use log::*;
use std::convert::TryInto;

type ClientID = u32;
type Message = String;

static V100_PREFIX: &'static str = "API\x00";
static MIN_CLIENT_VER: u32 = 100;
static MAX_CLIENT_VER: u32 = 157;
static VERSION: u32 = 2;
static DELIMITER: u8 = 0;
/// EClient is a struct representing a client that is connected to the server. It handles the message queue, sending messages, and other lower-level
/// details related to sending/receiving messages. It contains the TCP connection struct.
pub struct EClient {
    msg_queue: VecDeque<Message>,
    host: String,
    port: u16,
    conn: IBSocket,
    extra_auth: bool,
    client_id: ClientID,
    server_version: Option<String>,
    connection_state: ConnectionState,
    asynchronous: bool,
}

impl EClient {
    pub fn new<S: Into<String>>(hostname: S, port: u16, client_id: u32) -> EClient {
        let hostname = hostname.into();
        EClient {
            msg_queue: VecDeque::new(),
            host: hostname.clone(),
            port: port,
            conn: IBSocket::new(hostname, port as u16),
            extra_auth: false,
            client_id: client_id,
            server_version: None,
            asynchronous: false,
            connection_state: ConnectionState::Disconnected,
        }
    }

    /// Starts the API layer. Negotiates the server version, etc. 
    pub fn start_api(&mut self) -> Result<(), Error> {
        let mut msg: message::Message = message::Message::new();
        let f: u32 = message::OutboundMessages::StartApi{}.into();
        msg.add_field(IBField::IBInteger(f));
        msg.add_field(IBField::IBInteger(VERSION));
        msg.add_field(IBField::IBInteger(self.client_id));

        self.conn.send(&msg)?;
        match self.conn.receive_once() {
            Ok(r) => {
                info!("Response bytes are: {:?}", r);
                Ok(())
            },
            Err(e) => {
                error!("Error starting API: {:?}", e);
                Err(e)
            }
        }
    }

    /// This function must be called before any other. It connects the socket to the API server.
    pub fn connect(&mut self) -> Result<(), Error> {
        match self.connection_state {
            ConnectionState::Connected => {
                return Err(anyhow!("Client is already connected"));
            },
            ConnectionState::Connecting => {
                return Err(anyhow!("Client is currently connecting"));
            }
            _ => {}
        }

        match self.conn.connect() {
            Ok(r) => {
                info!("Socket connected: {:?}", r);
            },
            Err(e) => {
                error!("{:?}", e);
                return Err(e);
            }
        }
        self.connection_state = ConnectionState::Connecting;

        let connOpts = "";
        let header = V100_PREFIX.as_bytes();
        let conn_message = format!("v{}..{}{}", MIN_CLIENT_VER, MAX_CLIENT_VER, connOpts).as_bytes().to_vec();
        let l = conn_message.len() as u32;
        let l = l.to_be_bytes().to_vec();

        
        info!("Sending connection message: {:?}", conn_message);
        self.conn.send_raw(&header)?;
        self.conn.send_raw(&l)?;
        self.conn.send_raw(&conn_message)?;
        match self.conn.receive_once() {
            Ok(r) => {
                info!("Connection response received: {:?}", r);
                self.connection_state = ConnectionState::Connected;
                let msg_size: [u8; 4] = r[0..4].try_into()?;
                let msg_size: u32 = u32::from_be_bytes(msg_size);
                let msg: Vec<u8> = r[4..msg_size as usize].to_vec();

                let fields = msg.split(|field| {
                    *field == DELIMITER
                });

                let mut results: Vec<String> = vec![];
                for field in fields {
                    let s_field = String::from_utf8(field.to_vec())?;
                    debug!("Got field: {:#?}", s_field);
                    results.push(s_field);
                }

                self.server_version = Some(results.remove(0));
                Ok(())
            },
            Err(e) => {
                error!("Error receiving response from connection attempt: {:?}", e);
                Err(e)
            }
        }
    }

}
enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}
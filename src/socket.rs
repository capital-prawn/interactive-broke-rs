use crate::message::{InboundMessage, Message};
use anyhow::Error;
use anyhow::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use log::{debug, error, info};
use std::net::TcpStream;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};
use std::{
    io::{self, Read},
    thread::park,
};
static PRODUCTION_HOST: &'static str = "localhost";
static PRODUCTION_PORT: u16 = 7496;
type RxChan = Arc<Mutex<Receiver<Message>>>;
type TxChan = Arc<Mutex<Sender<Message>>>;
pub struct IBSocket {
    stream: Option<TcpStream>,
    host: String,
    port: u16,
    outbound_rx: Option<RxChan>,
    outbound_tx: Option<TxChan>,
    inbound_rx: Option<RxChan>,
    inbound_tx: Option<TxChan>,
}

impl IBSocket {
    /// Creates and returns a new IBSocket. This handles the network-level connectivity and
    /// data send/receive to the API server.
    pub fn new<S: Into<String>>(host: S, port: u16) -> Self {
        let (otx, orx) = unbounded();
        let (itx, irx) = unbounded();
        Self {
            stream: None,
            host: host.into(),
            port: port,
            outbound_rx: Some(Arc::new(Mutex::new(orx))),
            outbound_tx: Some(Arc::new(Mutex::new(otx))),
            inbound_rx: Some(Arc::new(Mutex::new(irx))),
            inbound_tx: Some(Arc::new(Mutex::new(itx))),
        }
    }

    /// Connects the `IBSocket` to the API server
    pub fn connect(&mut self) -> Result<(), Error> {
        let addr = format!("{}:{}", self.host, self.port);
        match TcpStream::connect(addr) {
            Ok(stream) => {
                self.stream = Some(stream);
                info!("Socket connected!");
                return Ok(());
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    /// Sends a `Message` to the API server
    pub fn send(&mut self, message: &Message) -> Result<(), Error> {
        match &mut self.stream {
            Some(ref mut s) => {
                s.write(&message.to_bytes())?;
                Ok(())
            }
            None => Err(anyhow!("No stream to API server exists")),
        }
    }

    pub fn send_raw(&mut self, message: &[u8]) -> Result<(), Error> {
        match &mut self.stream {
            Some(ref mut s) => {
                let b = s.write(&message)?;
                debug!("Wrote {} bytes", b);
                Ok(())
            }
            None => Err(anyhow!("No stream to API server exists")),
        }
    }
    /// Tries to receive one batch of input; some amount of text until an EOF is encountered.
    /// Will block until it does.
    pub fn receive_once(&mut self) -> Result<Vec<u8>, Error> {
        match &mut self.stream {
            Some(ref mut s) => {
                let mut buf = [0; 128];
                debug!("Waiting to read data...");
                match s.read(&mut buf[..]) {
                    Ok(b) => {
                        debug!("Read {} bytes", b);
                        Ok(buf.to_vec())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            None => Err(anyhow!("No stream to API server exists")),
        }
    }

    /// Starts the send and receive loop
    pub fn start(&self) -> Result<(), Error> {
        let mut s = if let Some(s) = &self.stream {
            s
        } else {
            return Err(anyhow!("No connected socket found!"));
        };
        s.set_nonblocking(true)?;

        loop {
            let mut buf: String = String::new();
            match s.read_to_string(&mut buf) {
                Ok(b) => {
                    debug!("Read {} bytes from server!", b);
                    match InboundMessage::from_bytes(&buf.into_bytes()) {
                        Ok(msg) => match &self.inbound_tx {
                            Some(tx) => match tx.lock() {
                                Ok(l) => match l.send(Message::Inbound(msg)) {
                                    Ok(_) => {}
                                    Err(e) => {
                                        error!("Error dispatching InboundMessage: {}", e);
                                    }
                                },
                                Err(e) => {
                                    error!("Error dispatching received message: {}", e);
                                    break;
                                }
                            },
                            None => {
                                error!("No inbound_tx channel found!");
                                break;
                            }
                        },
                        Err(e) => {
                            error!("Error extracting InboundMessage: {}", e);
                        }
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    error!("Error receiving data: {}", e);
                    break;
                }
            }

            match &self.outbound_rx {
                Some(rx) => match rx.lock() {
                    Ok(l) => {
                        while l.len() > 0 {
                            match l.try_recv() {
                                Ok(msg) => match s.write(&msg.to_bytes()) {
                                    Ok(b) => {
                                        debug!("Write {} bytes to socket", b);
                                    }
                                    Err(e) => {
                                        anyhow!("Error writing to socket: {}", e);
                                    }
                                },
                                Err(e) => {
                                    error!("Error receiving outbound message to socket: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error locking outbound RX channel: {}", e);
                    }
                },
                None => {}
            }
            park();
        }
        Ok(())
    }
}

impl Default for IBSocket {
    fn default() -> Self {
        Self {
            stream: None,
            host: PRODUCTION_HOST.to_string(),
            port: PRODUCTION_PORT,
            outbound_rx: None,
            outbound_tx: None,
            inbound_rx: None,
            inbound_tx: None,
        }
    }
}

#[cfg(test)]
mod tests {
    static TEST_HOST: &'static str = "localhost";
    static TEST_PORT: u16 = 5555;
    use super::*;
    #[test]
    fn create_ibsocket() {
        let ib = IBSocket::new(TEST_HOST, TEST_PORT);
        assert_eq!(ib.host, "localhost");
    }
}


use crate::message::Message;
use anyhow::Error;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

use anyhow::*;
use log::{info, debug};
static PRODUCTION_HOST: &'static str = "localhost";
static PRODUCTION_PORT: u16 = 7496;
pub struct IBSocket {
    stream: Option<TcpStream>,
    host: String,
    port: u16,
}

impl IBSocket {
    /// Creates and returns a new IBSocket. This handles the network-level connectivity and
    /// data send/receive to the API server.
    pub fn new<S: Into<String>>(host: S, port: u16) -> Self {
        Self {
            stream: None,
            host: host.into(),
            port: port,
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
            },
            None => {
                Err(anyhow!("No stream to API server exists"))
            }
        }
    }

    pub fn send_raw(&mut self, message: &[u8]) -> Result<(), Error> {
        match &mut self.stream {
            Some(ref mut s) => {
                let b = s.write(&message)?;
                debug!("Wrote {} bytes", b);
                Ok(())
            },
            None => {
                Err(anyhow!("No stream to API server exists"))
            }
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
                    },
                    Err(e) => {
                        Err(e.into()) 
                    }
                }
            },
            None => {
                Err(anyhow!("No stream to API server exists"))
            }
        }
    }
}

impl Default for IBSocket {
    fn default() -> Self {
        Self {
            stream: None,
            host: PRODUCTION_HOST.to_string(),
            port: PRODUCTION_PORT,
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

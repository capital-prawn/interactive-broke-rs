use std::io::Error;
use std::net::TcpStream;

static PRODUCTION_HOST: &'static str = "localhost";
static PRODUCTION_PORT: u16 = 7496;
pub struct IBSocket {
    stream: Option<TcpStream>,
    host: String,
    port: u16,
}

impl IBSocket {
    pub fn new<S: Into<String>>(host: S, port: u16) -> Self {
        Self {
            stream: None,
            host: host.into(),
            port: port,
        }
    }

    pub fn connect(&mut self) -> Result<(), Error> {
        let addr = format!("{}:{}", self.host, self.port);
        match TcpStream::connect(addr) {
            Ok(stream) => {
                self.stream = Some(stream);
                return Ok(());
            }
            Err(e) => {
                return Err(e);
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

    #[test]
    fn connect_to_local() {
        let mut ib = IBSocket::new(TEST_HOST, TEST_PORT);
        assert!(ib.connect().is_ok());
    }
}

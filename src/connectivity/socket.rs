use std::io::Error;
use std::net::TcpStream;

static PRODUCTION_HOST: &'static str = "localhost";
static PRODUCTION_PORT: u16 = 7496;
pub struct IBSocket {
    stream: Option<TcpStream>,
    host: &'static str,
    port: u16,
}

impl IBSocket {
    fn new() -> Self {
        Self {
            stream: None,
            host: PRODUCTION_HOST,
            port: PRODUCTION_PORT,
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

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn create_ibsocket() {
        let ib = IBSocket::new();
        assert_eq!(ib.host, "localhost");
    }

    #[test]
    fn connect_to_local() {
        let mut ib = IBSocket::new();
        assert!(ib.connect().is_ok());
    }
}

use env_logger;
use ibkrust;
use log::{error, info};

pub fn main() {
    env_logger::init();
    let mut client = ibkrust::client::EClient::new("localhost", 4001, 100);
    match client.connect() {
        Ok(_) => match client.start_api() {
            Ok(_) => {
                info!("API Started!");
            }
            Err(e) => {
                error!("Error starting API: {}", e);
            }
        },
        Err(e) => {
            error!("{}", e);
        }
    }
}

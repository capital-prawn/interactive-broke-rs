

use ibkrust::*;
use env_logger;

#[test]
fn test_connect_to_tws() {
  env_logger::init();
  let mut test_client = client::EClient::new("localhost", 4001, 100);
  let result = test_client.connect();
  println!("Result is: {:?}", result);
}




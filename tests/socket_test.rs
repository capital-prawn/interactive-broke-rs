

use ibkrust::*;
use env_logger;

#[test]
fn test_connect_to_tws() {
  env_logger::init();
  let mut test_client = client::EClient::new("localhost", 4001, 100);
  assert_eq!(test_client.connect().is_ok(), true);
  assert_eq!(test_client.start_api().is_ok(), true);
}




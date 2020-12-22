use crate::message::Message;
use anyhow::*;

pub trait FromBytes {
    fn from_bytes(b: &[u8]) -> Result<Message>;
}

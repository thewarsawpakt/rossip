use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    version: u64, // Version number, required for gossip protocol
    data: String, // This is just garbage data that will be used as a 'payload'
}

impl Message {
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Message> {
        match bincode::deserialize::<Message>(&bytes) {
            Ok(deserialized) => Some(deserialized),
            Err(_e) => {
                warn!("Error deserializing bytes");
                None
            }
        }
    }
    #[allow(dead_code)] // I haven't implemented this yet but I want the warning out of my face
    pub fn to_bytes(&mut self) -> Option<Vec<u8>> {
        match bincode::serialize(self) {
            Ok(serialized) => Some(serialized),
            Err(_e) => {
                warn!("Error serializing Message");
                None
            }
        }
    }
}

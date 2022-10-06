use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use log::warn;

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
    #[allow(dead_code)] // I haven't used this yet but I want the warning out of my face
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

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        return self.version == other.version
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.version.cmp(&other.version))
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Event {
    SyncUpdate { resource: String, version: u64 },

    HeartBeat,
}

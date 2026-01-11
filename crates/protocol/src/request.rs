use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Request {
    Ping,

    Subscribe { topic: String },

    Unsubscribe { topic: String },
}

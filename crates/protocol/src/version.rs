use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
}

impl Version {
    pub const fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

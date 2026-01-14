use serde::{Deserialize, Serialize};

pub mod codec;
pub mod event;
pub mod framer;
pub mod request;
pub mod response;
pub mod version;

pub use event::Event;
pub use request::Request;
pub use response::{ErrorCode, Response};
pub use version::Version;

pub type RequestId = u64;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Frame {
    Request {
        id: RequestId,
        version: Version,
        request: Request,
    },
    Response {
        id: RequestId,
        version: Version,
        response: Response,
    },
}

impl Frame {
    pub fn version(&self) -> Version {
        match self {
            Frame::Request { version, .. } => *version,
            Frame::Response { version, .. } => *version,
        }
    }
}

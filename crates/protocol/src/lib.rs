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
    Reponse {
        id: RequestId,
        version: Version,
        response: Request,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_frame_roundtrip() {
        let frame = Frame::Request {
            id: 1,
            version: Version { major: 1, minor: 0 },
            request: Request::Ping,
        };

        let bytes = serde_json::to_vec(&frame).unwrap();
        let decoded: Frame = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(frame, decoded);
    }
}

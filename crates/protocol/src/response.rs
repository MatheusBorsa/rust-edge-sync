use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Response {
    Pong,
    Ok,
    Error { code: ErrorCode, message: String },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ErrorCode {
    InvalidRequest,
    UnsupportedVersion,
    Unauthorized,
    InternalServerError,
}

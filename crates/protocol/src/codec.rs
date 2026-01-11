use serde_json;

pub fn encode(frame: &Frame) -> Vec<u8> {
    serde_json::to_vec(frame).unwrap()
}

pub fn decode(bytes: &[u8]) -> Frame {
    serde_json::from_slice(bytes).unwrap()
}

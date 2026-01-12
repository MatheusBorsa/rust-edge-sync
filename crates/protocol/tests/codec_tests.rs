use protocol::{Frame, Request, Version, codec};

#[test]
fn test_codec_roundtrip_request() {
    let frame = Frame::Request {
        id: 1,
        version: Version { major: 1, minor: 0 },
        request: Request::Ping,
    };

    let bytes = codec::encode(&frame);
    let decoded = codec::decode(&bytes);

    assert_eq!(frame, decoded);
}

#[test]
fn test_codec_roundtrip_response() {
    let frame = Frame::Response {
        id: 42,
        version: Version { major: 1, minor: 0 },
        response: Request::Ping,
    };

    let bytes = codec::encode(&frame);
    let decoded = codec::decode(&bytes);

    assert_eq!(frame, decoded);
}

#[test]
fn test_codec_empty_payload() {
    let frame = Frame::Request {
        id: 0,
        version: Version { major: 0, minor: 0 },
        request: Request::Ping,
    };

    let bytes = codec::encode(&frame);
    assert!(!bytes.is_empty());

    let decoded = codec::decode(&bytes);
    assert_eq!(frame, decoded);
}

#[test]
fn test_codec_invalid_json() {
    let bad_bytes = b"not json";
    let result = std::panic::catch_unwind(|| codec::decode(bad_bytes));
    assert!(result.is_err());
}

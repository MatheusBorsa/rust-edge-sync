use protocol::{Frame, Request, Version, codec, framer};
use std::io::Cursor;

#[test]
fn test_framer_roundtrip_in_memory() {
    let mut buffer = Cursor::new(Vec::new());

    let frame = Frame::Request {
        id: 1,
        version: Version { major: 1, minor: 0 },
        request: Request::Ping,
    };

    let payload = codec::encode(&frame);
    framer::write_frame(&mut buffer, &payload).unwrap();

    buffer.set_position(0);

    let read_bytes = framer::read_frame(&mut buffer).unwrap();
    let decoded = codec::decode(&read_bytes);

    assert_eq!(frame, decoded);
}

#[test]
fn test_framer_multiple_frames() {
    let mut buffer = Cursor::new(Vec::new());

    let frames = vec![
        Frame::Request {
            id: 1,
            version: Version { major: 1, minor: 0 },
            request: Request::Ping,
        },
        Frame::Response {
            id: 1,
            version: Version { major: 1, minor: 0 },
            response: Request::Ping,
        },
    ];

    for frame in &frames {
        let payload = codec::encode(frame);
        framer::write_frame(&mut buffer, &payload).unwrap();
    }

    buffer.set_position(0);

    for expected in &frames {
        let read_bytes = framer::read_frame(&mut buffer).unwrap();
        let decoded = codec::decode(&read_bytes);
        assert_eq!(decoded, *expected);
    }
}

#[test]
fn test_framer_empty_frame() {
    let mut buffer = Cursor::new(Vec::new());
    let payload = Vec::new();

    framer::write_frame(&mut buffer, &payload).unwrap();
    buffer.set_position(0);

    let read_bytes = framer::read_frame(&mut buffer).unwrap();
    assert!(read_bytes.is_empty());
}

#[test]
fn test_framer_large_frame() {
    let mut buffer = Cursor::new(Vec::new());

    let frame = Frame::Request {
        id: 999,
        version: Version { major: 1, minor: 0 },
        request: Request::Ping,
    };

    let mut payload = codec::encode(&frame);
    payload.extend(vec![0u8; 10_000]);

    framer::write_frame(&mut buffer, &payload).unwrap();
    buffer.set_position(0);

    let read_bytes = framer::read_frame(&mut buffer).unwrap();
    assert_eq!(read_bytes, payload);
}

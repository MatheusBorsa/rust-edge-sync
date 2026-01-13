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

#[test]
fn test_framer_partial_payload() {
    let frame = Frame::Request {
        id: 3,
        version: protocol::Version { major: 1, minor: 0 },
        request: protocol::Request::Ping,
    };

    let payload = codec::encode(&frame);

    let mut full_buffer = Vec::new();
    framer::write_frame(&mut full_buffer, &payload).unwrap();

    let mut partial_buffer = Cursor::new(&full_buffer[..2]);
    let result = framer::read_frame(&mut partial_buffer);
    assert!(result.is_err());
}

#[test]
fn test_framer_partial_length_prefix() {
    let prefix_len: usize = 4;
    let frame = Frame::Request {
        id: 1,
        version: Version { major: 1, minor: 0 },
        request: Request::Ping,
    };

    let payload = codec::encode(&frame);
    let mut full_buffer = Vec::new();
    framer::write_frame(&mut full_buffer, &payload).unwrap();

    for cut in 1..prefix_len {
        let mut cursor = Cursor::new(&full_buffer[..cut]);
        let result = framer::read_frame(&mut cursor);
        assert!(result.is_err());
    }
}

#[test]
fn test_framer_lying_length_prefix() {
    let fake_len: u32 = 100;
    let actual_payload = vec![1, 2, 3, 4, 5];
    let mut buffer = Vec::new();

    buffer.extend(fake_len.to_be_bytes());
    buffer.extend(&actual_payload);

    let mut cursor = Cursor::new(buffer);
    let result = framer::read_frame(&mut cursor);

    assert!(result.is_err());
}

#[test]
fn test_framer_max_size_enforcement() {
    let mut buffer = Cursor::new(Vec::new());
    let oversized_len: u32 = u32::MAX;

    buffer.get_mut().extend(oversized_len.to_be_bytes());
    buffer.get_mut().extend(vec![0u8; 1024]);
    buffer.set_position(0);

    let result = framer::read_frame(&mut buffer);

    assert!(result.is_err());
}

#[test]
fn test_framer_incremental_streaming() {
    let frame = Frame::Request {
        id: 1,
        version: Version { major: 1, minor: 0 },
        request: Request::Ping,
    };
    let payload = codec::encode(&frame);
    let mut full_buffer = Vec::new();

    framer::write_frame(&mut full_buffer, &payload).unwrap();

    let total_len = full_buffer.len();
    let midpoint = total_len / 2;

    let mut cursor = Cursor::new(&full_buffer[..midpoint]);
    let result = framer::read_frame(&mut cursor);

    assert!(result.is_err());
}

#[test]
fn test_framer_garbage_data() {
    let garbage = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xAA, 0xBB, 0xCC, 0xDD];
    let mut cursor = Cursor::new(garbage);
    let result = framer::read_frame(&mut cursor);

    assert!(result.is_err());
}

#[test]
fn test_framer_eof_handling() {
    let mut buffer = Cursor::new(Vec::new());
    let result = framer::read_frame(&mut buffer);
    assert!(result.is_err());
}

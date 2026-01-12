use protocol::{Frame, Request, Version, codec, framer};
use std::net::{TcpListener, TcpStream};
use std::thread;

#[test]
fn test_roundtrip_frame() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server_handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();

        let read_bytes = framer::read_frame(&mut stream).unwrap();
        let decoded: Frame = codec::decode(&read_bytes);

        let payload = codec::encode(&decoded);
        framer::write_frame(&mut stream, &payload).unwrap();

        decoded
    });

    let mut client_stream = TcpStream::connect(addr).unwrap();

    let frame = Frame::Request {
        id: 1,
        version: Version { major: 1, minor: 0 },
        request: Request::Ping,
    };

    let payload = codec::encode(&frame);
    framer::write_frame(&mut client_stream, &payload).unwrap();

    let echoed_bytes = framer::read_frame(&mut client_stream).unwrap();
    let echoed_frame = codec::decode(&echoed_bytes);

    assert_eq!(frame, echoed_frame);

    let server_frame = server_handle.join().unwrap();
    assert_eq!(frame, server_frame);
}

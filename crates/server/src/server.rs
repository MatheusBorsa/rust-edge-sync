use std::io;
use std::net::TcpListener;

use crate::connection::handle_connection;

pub fn run(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    println!("Server listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr()?);
                handle_connection(stream)?;
            }
            Err(e) => eprintln!("Connection error: {}", e),
        }
    }

    Ok(())
}

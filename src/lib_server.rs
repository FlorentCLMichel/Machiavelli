//! Library for the game server

use super::*;
pub use std::io::{ stdin, Read, Write };
pub use std::net::{ TcpListener, TcpStream, Shutdown };
pub use std::thread::JoinHandle;
pub use std::str::from_utf8;

const BUFFER_SIZE: usize = 50;

// bogus function to handle a client; to be replaced
pub fn handle_client(mut stream: TcpStream) {
    let mut name: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    while match stream.read(&mut name) {
        Ok(size) => {
            // echo the stream data
            let _msg = format!("Hello {}!", from_utf8(&name[..size]).unwrap());
            let msg = _msg.as_bytes();
            stream.write(msg).unwrap();
            true
        },
        Err(_)=> {
            println!("An error occured while reading the stream; terminating connection with {}", 
                     stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

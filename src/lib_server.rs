//! Library for the game server

use super::*;
pub use std::io::{ stdin, Read, Write };
pub use std::net::{ TcpListener, TcpStream, Shutdown };
pub use std::thread::JoinHandle;

const SIZE_FIRST_CLIENT_MESSAGE: usize = 6;

// bogus function to handle a client; to be replaced
pub fn handle_client(mut stream: TcpStream, i_player: u8) {
    let mut data: [u8; SIZE_FIRST_CLIENT_MESSAGE] = [0; SIZE_FIRST_CLIENT_MESSAGE];
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo the stream data
            let _msg = format!("You are player {}", i_player);
            let mut msg = _msg.as_bytes();
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

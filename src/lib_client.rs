//! Library for the game client

use super::*;
pub use std::net::TcpStream;
pub use std::io::{ Read, Write };
pub use std::str::from_utf8;

const HOST: &str = "localhost:3333";
const BUFFER_SIZE: usize = 50;

pub fn say_hello() {
    match TcpStream::connect(HOST) {
        Ok(mut stream) => {
            println!("Successfully connected to {}", &HOST);

            // get the player name
            let mut name = String::new();
            let mut cont = true;
            println!("Player name:");
            while cont {
                match get_input() {
                    Ok(s) => {
                        name = s.trim().to_string();
                        cont = false
                    },
                    Err(_) => println!("Could not parse the input")
                };
            }

            stream.write(name.as_bytes()).unwrap();
            println!("Sent the name to server; awaiting reply...");

            let mut received_data: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            match stream.read(&mut received_data) {
                Ok(_) => {
                    
                    // set the terminal appearance
                    reset_style();

                    // clear the terminal
                    print!("\x1b[2J\x1b[1;1H");

                    // print the player number
                    println!("{}", from_utf8(&received_data).unwrap());
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            loop {}
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

//! Client for the Machiavelli game

use std::env;
use std::process::exit;
use machiavelli::lib_client::*;

fn main() {
    
    // parse the command-line arguments
    let args: Vec<String> = env::args().collect();

    let mut stream: TcpStream;
    let mut single_byte_buffer: &mut [u8; 1] = &mut [0];

    if args.len() > 1 {
        match say_hello(args[1].clone()) {
            Ok(s) => stream = s,
            Err(e) => panic!("Failed to connect: {}", e)
        };
    } else {
        match say_hello("".to_string()) {
            Ok(s) => stream = s,
            Err(e) => panic!("Failed to connect: {}", e)
        };
    }

    loop {
        // handle the server request
        handle_server_request(&mut single_byte_buffer, &mut stream).unwrap_or_else(|_| {
            println!("Lost connection to the server");
            exit(1);
        });
    }
}

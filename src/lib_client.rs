//! Library for the game client

use super::*;
pub use std::net::TcpStream;
pub use std::io::{ Read, Write };
pub use std::str::from_utf8;

const HOST: &str = "localhost:3333";
const BUFFER_SIZE: usize = 50;
const MAX_N_BUFFERS: usize = 255;
const N_MILLISECONDS_WAIT: u64 = 10;

pub fn say_hello() -> Result<TcpStream,StreamError> {
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

            send_str_to_server(&mut stream, &name).unwrap();
            println!("Sent the name to server; awaiting reply...");

            match get_str_from_server(&mut stream) {
                Ok(s) => {
                    
                    // set the terminal appearance
                    reset_style();

                    // clear the terminal
                    clear_terminal();

                    // print the message sent by the server
                    println!("{}", s);
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
            Ok(stream)
        }
        Err(e) => { Err(StreamError::from(e)) }
    }
}
        
pub fn handle_server_request(single_byte_buffer: &mut [u8; 1], stream: &mut TcpStream) -> Result<(), StreamError> {
    stream.read(single_byte_buffer)?;
    match single_byte_buffer[0] {
        
        // value 1: print the message from te server
        1 => print_str_from_server(stream)?,
        
        // value 2: clear the terminal and print the message from the server
        2 => clear_and_print_str_from_server(stream)?,
        
        // value 3: print the message and return a reply in bytes
        3 => print_and_reply(stream)?,
        
        // value 4: send a message
        4 => send_message(stream)?,

        _ => ()
    };
    Ok(())
}

fn clear_and_print_str_from_server(stream:  &mut TcpStream) -> Result<(), StreamError> {
    clear_terminal();
    println!("{}", get_str_from_server(stream)?);
    Ok(())
}

fn print_str_from_server(stream:  &mut TcpStream) -> Result<(), StreamError> {
    println!("{}", get_str_from_server(stream)?);
    Ok(())
}

fn print_and_reply(stream:  &mut TcpStream) -> Result<(), StreamError> {
    println!("{}", get_str_from_server(stream)?);
    send_message(stream)
}

fn send_message(stream:  &mut TcpStream) -> Result<(), StreamError> {
    let mut reply = String::new();
    let mut cont = true;
    while cont {
        match get_input() {
            Ok(s) => {
                reply = s.trim().to_string();
                cont = false
            },
            Err(_) => println!("Could not parse the input")
        };
    }
    send_str_to_server(stream, &reply)?;
    Ok(())
}

pub fn send_str_to_server(stream: &mut TcpStream, s: &str) -> Result<(), StreamError> {
    send_bytes_to_server(stream, &s.as_bytes())?;
    Ok(())
}

pub fn send_bytes_to_server(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), StreamError> {
    
    // ensure that the number of bytes is small enough
    if bytes.len() > MAX_N_BUFFERS * BUFFER_SIZE {
        return Err(StreamError { message: format!(
                    "Stream too long: size: {}, maximum size: {}",
                    bytes.len(), MAX_N_BUFFERS*BUFFER_SIZE
                   ) })
    }

    // the first bytes will determine the number of times the buffer should be read
    let mut n_buffers: u8 = (bytes.len() / BUFFER_SIZE) as u8;
    if bytes.len() % BUFFER_SIZE != 0 {
        n_buffers += 1;
    }
    stream.write(&[n_buffers])?;

    // write the data stream
    for i in 0..((n_buffers-1) as usize) {
        stream.write(&bytes[i*BUFFER_SIZE..(i+1)*BUFFER_SIZE])?;
    }
    stream.write(&bytes[((n_buffers-1) as usize)*BUFFER_SIZE..])?;

    // wait for a reply to be sent from the receiver
    while let Err(_) = stream.read_exact(&mut [0]) {}
    
    Ok(())
}

pub fn get_str_from_server(stream: &mut TcpStream) -> Result<String, StreamError> {
    let bytes = get_bytes_from_server(stream)?;
    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(_) => Err(StreamError::from(BytesToStringError {}))
    }
}

pub fn get_bytes_from_server(stream: &mut TcpStream) -> Result<Vec<u8>, StreamError> {
    
    // buffer
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    // the first bytes will determine the number of times the buffer should be read
    let mut n_buffers: [u8; 1] = [0];
    stream.read_exact(&mut n_buffers)?;

    // vector containing the result
    let mut res = Vec::<u8>::new();

    // read the data stream
    let mut size;
    for _i in 0..n_buffers[0] {
        size = stream.read(&mut buffer)?;
        res.extend_from_slice(&buffer[..size]);
    }
   
    // send something to confirm I have received the data
    stream.write(&[0])?;

    // return the result
    Ok(res)
}

// wait a moment
pub fn wait() {
    std::thread::sleep(std::time::Duration::from_millis(N_MILLISECONDS_WAIT));
}


// errors

#[derive(Debug)]
pub struct StreamError {
    message: String
}

#[derive(Debug)]
pub struct BytesToStringError {}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "StreamError: {}", self.message)
    }
}

impl std::convert::From<std::io::Error> for StreamError {
    fn from(error: std::io::Error) -> Self {
        StreamError { message: format!("IO Error: {}", error) }
    }
}

impl std::convert::From<BytesToStringError> for StreamError {
    fn from(_error: BytesToStringError) -> Self {
        StreamError { message: "Could not convert the byte sequence to a string".to_string() }
    }
}

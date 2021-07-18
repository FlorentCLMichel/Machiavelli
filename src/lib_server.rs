//! Library for the game server

pub use super::*;
pub use std::io::{ stdin, Read, Write };
pub use std::net::{ TcpListener, TcpStream, Shutdown };
pub use std::str::from_utf8;

const BUFFER_SIZE: usize = 50;
const MAX_N_BUFFERS: usize = 255;
const N_MILLISECONDS_WAIT: u64 = 10;
const N_MILLISECONDS_LONG_WAIT: u64 = 1000;

pub fn handle_client(mut stream: TcpStream) -> (TcpStream, String, usize) {
    let mut player_name: String = "".to_string();
    match get_str_from_client(&mut stream) {
        Ok(s) => {
            // great the player
            player_name = s.clone();
            let msg = format!("Hello {}!\nWaiting for other players to join...", &s);
            stream.write(&[1]).unwrap();
            send_str_to_client(&mut stream, &msg).unwrap();
        },
        Err(_)=> {
            println!("An error occured while reading the stream; terminating connection with {}", 
                     stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    };
    (stream, player_name, 0)
}

pub fn handle_client_load(mut stream: TcpStream, names: &Vec<String>) -> (TcpStream, String, usize) {
    let mut player_name: String;
    let position: usize;
    loop {
        match get_str_from_client(&mut stream) {
            Ok(s) => {
                player_name = s.clone();
                
                // check if the name is in the list
                match names.iter().position(|x| x == &player_name) {
                    Some(i) => {
                        position = i;
                        stream.write(&[1]).unwrap();
                        let msg = format!("Hello {}!\nWaiting for other players to join...", &s);
                        send_str_to_client(&mut stream, &msg).unwrap();
                        break;
                    },
                    None => {
                        stream.write(&[0]).unwrap();
                        let msg = format!("Sorry, {} is not in the list of players!\n", &s);
                        send_str_to_client(&mut stream, &msg).unwrap();
                    }
                }

            },
            Err(_)=> {
                println!("An error occured while reading the stream; terminating connection with {}", 
                         stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
            }
        };
    }
    (stream, player_name, position)
}

pub fn start_player_turn(table: &mut Table, hands: &mut Vec<Sequence>, deck: &mut Sequence, 
                         custom_rule_jokers: bool, player_names: &Vec<String>, current_player: usize, 
                         n_players: usize, streams: &mut Vec<TcpStream>)
    -> Result<(),StreamError> {

    // copy the initial hand
    let hand_start_round = hands[current_player].clone();

    // copy the initial table
    let table_start_round = table.clone();
    
    // send the instructions
    send_message_to_client(&mut streams[current_player], "\n")?;
    send_message_to_client(&mut streams[current_player], &instructions_no_save())?;

    // get and process the player choice
    let mut message: String;
    loop {
        match get_message_from_client(&mut streams[current_player]) {
            Ok(mes) => {
                if mes.len() == 0 {
                    ()
                } else {
                    match mes[0] {
                    
                        // value 'e': end the turn
                        101 => {
                            if !hand_start_round.contains(&hands[current_player]) {
                                message = "You can't end your turn until you've played all the cards you've taken from the table!\n"
                                          .to_string();
                                send_message_to_client(&mut streams[current_player], &message)?;
                            } else if custom_rule_jokers && hands[current_player].contains_joker() {
                                message = "Jokers need to be played!\n".to_string();
                                send_message_to_client(&mut streams[current_player], &message)?;
                            } else if !hands[current_player].contains(&hand_start_round) {
                                break
                            } else {
                                match pick_a_card(&mut hands[current_player], deck) {
                                    Ok(card) => message = format!("You have picked a {}\x1b[38;2;0;0;0;1m", &card),
                                    Err(_) => message = "No more card to draw!".to_string()
                                };
                                send_message_to_client(&mut streams[current_player], &message)?;
                                break
                            }
                        },
                    
                        // value 'p': play a sequence
                        112 => {
                            match play_sequence_remote(&mut hands[current_player], table, &mut streams[current_player]) {
                                Ok(true) => {
                                    
                                    // print the situation for the current player
                                    print_situation_remote(&table, &hands, deck, player_names, current_player,
                                                           current_player, &mut streams[current_player],
                                                           true)?;

                                    // print the new situation for the other players
                                    for i in 0..n_players {
                                        if i != current_player {
                                            print_situation_remote(&table, &hands, deck, player_names, 
                                                                   i, current_player, &mut streams[i],
                                                                   false)?;
                                        }
                                    }
                                },

                                Ok(false) => (),
                                Err(_) => send_message_to_client(&mut streams[current_player], &"Communication error\n")?
                            };
                        },
                        
                        // value 't': take a sequence from the table
                        116 => {
                            match take_sequence_remote(table, &mut hands[current_player], 
                                                       &mut streams[current_player]) {
                                Ok(()) => {

                                    // print the new situation for the current player
                                    print_situation_remote(&table, &hands, deck, player_names, 
                                                           current_player, current_player, 
                                                           &mut streams[current_player], true)?;

                                    // print the new situation for the other players
                                    for i in 0..n_players {
                                        if i != current_player {
                                            print_situation_remote(&table, &hands, deck, player_names, 
                                                                   i, current_player, &mut streams[i],
                                                                   false)?;
                                        }
                                    }
                                },

                                Err(_) => send_message_to_client(&mut streams[current_player], &"Communication error\n")?
                            };
                        },
 
                        // value 'r': sort cards by rank
                        114 => {
                            hands[current_player].sort_by_rank();
                            print_situation_remote(&table, &hands, deck, player_names, current_player,
                                                   current_player, &mut streams[current_player],
                                                   true)?;
                        },
                        
                        // value 's': sort cards by suit
                        115 => {
                            hands[current_player].sort_by_suit();
                            print_situation_remote(&table, &hands, deck, player_names, current_player,
                                                   current_player, &mut streams[current_player],
                                                   true)?;
                        },
            
                        // value 'g': give up on that round and take the penalty
                        103 => {
                            if custom_rule_jokers && hands[current_player].contains_joker() {
                                message = "Jokers need to be played!\n".to_string();
                                send_message_to_client(&mut streams[current_player], &message)?;
                            } else {
                                give_up(table, &mut hands[current_player], deck, hand_start_round, table_start_round);
                                print_situation_remote(&table, &hands, deck, player_names, current_player,
                                                       current_player, &mut streams[current_player],
                                                       true)?;
                                break;
                            }
                        },

                        _ => send_message_to_client(&mut streams[current_player], &"Invalid input; please try again.")?,
                    }
                }
            },
            Err(_) => {
                send_message_to_client(&mut streams[current_player], &"Could not get your input. Please try again.")?;
            }
        };
    }
    Ok(())
}

fn play_sequence_remote(hand: &mut Sequence, table: &mut Table, stream: &mut TcpStream) 
    -> Result<bool, StreamError>
{
    send_message_to_client(stream, &"Please enter the sequence, in order, separated by spaces")?;
    
    // print the hand with indices
    let hand_and_indices = hand.show_indices();
    send_message_to_client(stream, &"\n")?;
    send_message_to_client(stream, &hand_and_indices.0)?;
    send_message_to_client(stream, &reset_style_string())?;
    send_message_to_client(stream, &"\n")?;
    send_message_to_client(stream, &hand_and_indices.1)?;
    send_message_to_client(stream, &"\n")?;
    
    let mut seq = Sequence::new();
    
    let s = String::from_utf8(get_message_from_client(stream)
                                  .unwrap_or_else(|_| {Vec::<u8>::new()}))
        .unwrap_or_else(|_| {"".to_string()});
    
    let mut seq_i = Vec::<usize>::new();
    for item in s.split(' ') {
        match item.parse::<usize>() {
            Ok(n) => {
                let mut n_i = 0;
                for &i in &seq_i {
                    if i < n {
                        n_i += 1;
                    }
                }
                let card = match hand.take_card(n-n_i) {
                    Some(c) => c,
                    None => continue
                };
                seq.add_card(card);
                seq_i.push(n);
            },
            Err(_) => ()
        }
    }

    if seq.is_valid() {
        table.add(seq);
        return Ok(true);
    } else {
        let message = format!("{}{} is not a valid sequence!\n", &seq, &reset_style_string());
        hand.merge(seq);
        send_message_to_client(stream, &message)?;
        return Ok(false);
    }
}

fn take_sequence_remote(table: &mut Table, hand: &mut Sequence, stream: &mut TcpStream) 
    -> Result<(), StreamError> 
{
    send_message_to_client(stream, &"Which sequence would you like to take?\n")?;
    let s = String::from_utf8(get_message_from_client(stream)
                                  .unwrap_or_else(|_| {Vec::<u8>::new()}))
        .unwrap_or_else(|_| {"".to_string()});
    match s.trim().parse::<usize>() {
        Ok(n) => match table.take(n) {
            Some(seq) => {
                hand.merge(seq);
                return Ok(());
            },
            None => send_message_to_client(stream, &"This sequence is not on the table\n")?
        },
        Err(_) => send_message_to_client(stream, &"Error parsing the input!\n")?
    };
    Ok(())
}

fn print_situation_remote(table: &Table, hands: &Vec<Sequence>, deck: &Sequence, 
                          player_names: &Vec<String>, player: usize, current_player: usize, 
                          stream: &mut TcpStream, print_instructions: bool) 
    -> Result<(), StreamError>
{
    // string with the number of cards each player has
    let mut string_n_cards = "\nNumber of cards:".to_string();
    for i in 0..(hands.len()) {
        string_n_cards += &format!("\n  {}: {}", &player_names[i], &hands[i].number_cards());
    }
    string_n_cards += "\n";

    clear_and_send_message_to_client(stream, 
        &format!("\x1b[1m{}'s turn:{}", player_names[current_player], &reset_style_string()))?;
    send_message_to_client(stream, &string_n_cards).unwrap();
    send_message_to_client(stream, &situation_to_string(&table, &hands[player], &deck))?;
    send_message_to_client(stream, &"\n")?;
    if print_instructions {
        send_message_to_client(stream, &instructions_no_save())?;
    }
    Ok(())
}
                    
pub fn send_str_to_client(stream: &mut TcpStream, s: &str) -> Result<(), StreamError> {
    send_bytes_to_client(stream, &s.as_bytes())?;
    Ok(())
}

fn send_bytes_to_client_no_wait(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), StreamError> {
    
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
    
    Ok(())
}

pub fn send_bytes_to_client(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), StreamError> {
    
    send_bytes_to_client_no_wait(stream, bytes)?;
    
    // wait for a reply to be sent from the receiver
    stream.read(&mut [0])?;
    
    Ok(())
}

pub fn get_str_from_client(stream: &mut TcpStream) -> Result<String, StreamError> {
    let bytes = get_bytes_from_client(stream)?;
    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(_) => Err(StreamError::from(BytesToStringError {}))
    }
}

pub fn get_bytes_from_client(stream: &mut TcpStream) -> Result<Vec<u8>, StreamError> {
    
    // buffer
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    // the first bytes will determine the number of times the buffer should be read
    let mut n_buffers: [u8; 1] = [0];
    stream.read(&mut n_buffers)?;

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

/// wait a moment
pub fn wait() {
    std::thread::sleep(std::time::Duration::from_millis(N_MILLISECONDS_WAIT));
}

/// wait a longer moment
pub fn long_wait() {
    std::thread::sleep(std::time::Duration::from_millis(N_MILLISECONDS_LONG_WAIT));
}

/// check that no players have the same name; if yes, rename players
pub fn ensure_names_are_different(player_names: &mut Vec<String>, client_streams: &mut Vec<TcpStream>) {
    let mut cont = true;
    while cont {
        cont = false;
        for i in 0..player_names.len() {
            for j in (i+1)..player_names.len() {
                if player_names[j] == player_names[i] {
                    cont = true;
                    player_names[j] = format!("{}_", &player_names[j]);
                    send_message_to_client(&mut client_streams[j], 
                                       &format!("Your name is already taken! You were renamed as {}\n", 
                                               &player_names[j])).unwrap();
                }
            }
        }
    }
}

fn get_message_from_client(stream: &mut TcpStream) -> Result<Vec<u8>, StreamError>{
    stream.write(&mut [4])?;
    get_bytes_from_client(stream)
}

pub fn clear_and_send_message_to_client(stream: &mut TcpStream, msg: &str) -> Result<(), StreamError>{
    stream.write(&mut [2])?;
    send_str_to_client(stream, msg)
}

pub fn send_message_to_client(stream: &mut TcpStream, msg: &str) -> Result<(), StreamError>{
    stream.write(&mut [1])?;
    send_str_to_client(stream, msg)
}

/// send a message and get the output 
pub fn send_message_get_reply(stream: &mut TcpStream, message: &str) 
    -> Result<Vec<u8>, StreamError>
{
    stream.write(&mut [3])?;
    send_str_to_client(stream, message)?;
    get_bytes_from_client(stream)
}

/// send the same message to all players
pub fn send_message_all_players(client_streams: &mut [TcpStream], message: &str) -> Result<(),StreamError> {

    let n_players: usize = client_streams.len();

    // send the messages
    for i in 0..n_players {
        client_streams[i].write(&mut [1])?;
        send_bytes_to_client_no_wait(&mut client_streams[i], &message.as_bytes())?;
    }

    // wait until all clients have confirmed reception
    for i in 0..n_players {
        client_streams[i].read(&mut [0])?;
    }
    
    Ok(())
}

/// clear the screens and send the same message to all players
pub fn clear_and_send_message_all_players(client_streams: &mut [TcpStream], message: &str) -> Result<(),StreamError> {

    let n_players: usize = client_streams.len();

    // send the messages
    for i in 0..n_players {
        client_streams[i].write(&mut [2])?;
        send_bytes_to_client_no_wait(&mut client_streams[i], &message.as_bytes())?;
    }

    // wait until all clients have confirmed reception
    for i in 0..n_players {
        client_streams[i].read(&mut [0])?;
    }
    
    Ok(())
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

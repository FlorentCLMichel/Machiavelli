//! Server for the Machiavelli game

use std::process;
use std::fs::File;
use std::thread;
use std::env;
use rand::{ thread_rng, Rng };
use machiavelli::lib_server::*;

const SAVE_EXTENSION: &str = ".sav";

// ask the user for the port to use
fn get_port() -> usize {
    println!("Which port should I use?");
    loop {
        match get_input() {
            Ok(s) => match s.trim().parse::<usize>() {
                Ok(p)=> return p,
                Err(_) => println!("Could not parse the input")
            }
            Err(_) => println!("Could not parse the input")
        }
    }
}

fn main() {
    
    // get the command-line arguments
    let mut args = env::args();
    args.next(); // skip the first one (name of the executable)
    
    // clear the terminal
    print!("\x1b[2J\x1b[1;1H");
    println!("Machiavelli server\n");

    // port on which to listen
    let name_file_port_server = "Config/port_server.dat";
    let port = match std::fs::read_to_string(name_file_port_server) {
        Ok(s) => match s.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => get_port()
        }
        Err(_) => get_port()
    };

    // ask if a previous game should be loaded if not provided as an argument
    let load: bool;
    let load_from_command_line: bool;
    match args.next() {
        // "1" for yes, anything else for no
        Some(s) => {
            load_from_command_line = true;
            match s.trim().parse::<u8>() {
                Ok(1) => load = true,
                _ => load = false
            };
        }
        None => {
            load_from_command_line = false;
            println!("Load a previous game? (y/n)");
            load = match get_input().unwrap().trim() {
                "y" => true,
                _ => false
            };
        }
    };
        
    let mut config = Config {
            n_decks: 0,
            n_jokers: 0,
            n_cards_to_start: 0,
            custom_rule_jokers: false,
            n_players: 0
    };

    // default save file without the sav extension
    let mut savefile = "machiavelli_save".to_string();

    if !load {

        // get the config
        match get_config_from_file(&"Config/config.dat") {
            Ok(conf) => {
                config = conf.0;
                savefile = conf.1;
            },
            Err(_) => {
                println!("Could not read the config from the file!");
                match get_config_and_savefile() {
                    Ok(conf) => {
                        config = conf.0;
                        savefile = conf.1;
                    },
                    Err(_) => {
                        println!("Invalid input!");
                        process::exit(1);
                    }
                }
            }
        };
    }
    
    let mut starting_player: u8;
    let mut table = Table::new();
    let mut deck: Sequence;
    let mut hands: Vec<Sequence>;
    let mut player: usize;
    let mut player_names = Vec::<String>::new();
    let mut rng = thread_rng();
    
    if load {
        
        let mut fname = String::new(); // filename
        let mut bytes = Vec::<u8>::new();
        // if there is a next command-line argument, use it as name for the save file
        // if not, use the default name
        if load_from_command_line {
            match args.next() {
                Some(s) => fname = s,
                None => fname = savefile.clone() + SAVE_EXTENSION
            };
        }
        
        loop {

            // get the file name if not set
            if fname.len() == 0 {
                println!("Name of the save file (nothing for the default file):");
                match stdin().read_line(&mut fname) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Could not read the input");
                        continue;
                    }
                };
            }

            fname = fname.trim().to_string();

            // if the length is equal to 0, use the default file name
            if fname.len() == 0 {
                fname = savefile.clone() + SAVE_EXTENSION;
            }

            // try to open the file
            let mut file: File; 
            match File::open(fname.clone()) {
                Ok(f) => file = f,
                Err(_) => {
                    println!("Could not open the file!");
                    fname.clear();
                    continue;
                }
            };

            // load the data from the file
            match file.read_to_end(&mut bytes) {
                Ok(_) => (),
                Err(_) => {
                    println!("Could not read from the file!");
                    bytes.clear();
                    fname.clear();
                    continue;
                }
            };
            
            // decode the sequence of bytes
            bytes = encode::xor(&bytes, &fname.as_bytes());

            // load the game
            match load_game(&bytes) {
                Ok(lg) => {
                    config = lg.0;
                    starting_player = lg.1;
                    player = lg.2 as usize; 
                    table = lg.3;
                    hands = lg.4; 
                    deck = lg.5;
                    player_names = lg.6;
                },
                Err(_) => {
                    println!("Error loading the save file!");
                    bytes.clear();
                    fname.clear();
                    continue;
                }
            };

            break;
        }

    } else {

        // build the deck
        deck = Sequence::multi_deck(config.n_decks, config.n_jokers, &mut rng);
    
        // choose the starting player randomly
        starting_player = rng.gen_range(0..config.n_players);
        player = starting_player as usize;
        
        // build the hands
        hands = vec![Sequence::new(); config.n_players as usize];
        for i in 0..config.n_players {
            for _ in 0..config.n_cards_to_start {
                hands[i as usize].add_card(deck.draw_card().unwrap());
            }
        }

    }

    // set-up the tcp listener
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    // current number of clients
    let mut n_clients: u8 = 0;

    // vector of client threads
    let mut client_threads = Vec::<thread::JoinHandle<(TcpStream, String, usize)>>::new();
    
    // vector of client streams
    let mut client_streams = Vec::<TcpStream>::new();
    
    // accept connections and process them, each in its own thread
    let names_taken = Arc::new(Mutex::new(Vec::<String>::new())); // vector of the names that are already taken
    println!("\nserver listening to port {}", port);
    for stream_res in listener.incoming() {
        match stream_res {
            Ok(stream) => {
                n_clients += 1;
                println!("New connection: {} (player {})", stream.peer_addr().unwrap(), n_clients);
                if load {
                    let player_names_ = player_names.clone();
                    let arc = names_taken.clone();
                    client_threads.push(thread::spawn(move || {
                        handle_client_load(stream, &player_names_, arc).unwrap()
                    }));
                } else {
                    client_threads.push(thread::spawn(move || {handle_client(stream).unwrap()}));
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        // exit the loop if enough players have joined
        if n_clients == config.n_players {
            break;
        }
    }
    
    // wait for all threads to finish and collect the client streams 
    if load {

        for _i in 0..config.n_players {
            client_streams.push(TcpStream::connect(format!("0.0.0.0:{}", port)).unwrap());
        }
        for thread in client_threads {
            let output = thread.join().unwrap();
            client_streams[output.2] = output.0;
        }

    } else {

        for thread in client_threads {
            let output = thread.join().unwrap();
            client_streams.push(output.0);
            player_names.push(output.1);
        }

        // check that no players have the same name; if yes, rename players
        ensure_names_are_different(&mut player_names, &mut client_streams).unwrap();
    }

    // name of the save file
    let save_name = &(savefile.clone() + SAVE_EXTENSION);
    
    // name of the backup save file
    let backup_name = &(savefile.clone() + &"_bak" + SAVE_EXTENSION);
    
    let mut play_again = true;
    while play_again {
        loop {
            
            // if all the cards have been drawn, stop the game
            if deck.number_cards() == 0 {
                send_message_all_players(&mut client_streams, &"\n\x1b[1mNo more cards in the deck—it's a draw!\x1b[0m\n")
                    .unwrap();
                break;
            }
            
            // save the game
            let mut bytes = game_to_bytes(starting_player as u8, player as u8, &table, &hands, &deck, &config, &player_names);
            bytes = encode::xor(&bytes, save_name.as_bytes());
            match File::create(save_name) {
                Ok(mut f) => match f.write_all(&bytes) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Could not write to the save file!");
                    }
                },
                Err(_) => {
                    println!("Could not create the save file!");
                }
            };
            
            // backup the save file
            match std::fs::copy(&save_name, &backup_name) {
                Ok(_) => (),
                Err(_) => println!("Could not create the backup file!")
            };
 
            // print the name of the current player 
            clear_and_send_message_all_players(&mut client_streams, 
                                               &format!("\x1b[1m{}'s turn:{}", 
                                                        &player_names[player], &reset_style_string()))
                .unwrap();
        
            // string with the number of cards each player has
            let mut string_n_cards = "\nNumber of cards:".to_string();
            for i in 0..(config.n_players as usize) {
                string_n_cards += &format!("\n  {}: {}", &player_names[i], &hands[i].number_cards());
            }
            string_n_cards += "\n";

           
            // print the situation for each player
            for i in 0..(config.n_players as usize) {
                send_message_to_client(&mut client_streams[i], &string_n_cards).unwrap();
                send_message_to_client(&mut client_streams[i], 
                                   &situation_to_string(&table, &hands[i], &deck, &Sequence::new())).unwrap();
            }

            // player turn
            match start_player_turn(&mut table, &mut hands, &mut deck, 
                              config.custom_rule_jokers, &player_names,
                              player, config.n_players as usize, &mut client_streams)
            {
                Ok(_) => (),
                Err(_) => {
                    println!("Lost connection with player {}", player+1);
                    process::exit(1);
                }
            };
            
 
            // if the player has no more cards, stop the game
            if hands[player].number_cards() == 0 {
                send_message_all_players(&mut client_streams, 
                    &format!("\n\u{0007}\u{0007}\u{0007}\x1b[1m{} wins! Congratulations!\x1b[0m{}\n\n", 
                             player_names[player], &reset_style_string()))
                    .unwrap();
                break;
            }
            
            // next player
            player += 1;
            if player >= config.n_players as usize {
                player = 0;
            }

        }

        // ask the players if they want to play again
        send_message_all_players(&mut client_streams, &"Play again? (‘y’ for yes)\n".to_string()).unwrap();
        for stream in &mut client_streams {
            let reply = match get_string_from_client(stream) {
                Ok(s) => s,
                Err(_) => "y".to_string()
            };

            // if at least one of them does not say yes, quit
            if !is_yes(reply.trim()) {
                play_again = false;
                match stream.write(&mut [5]) {
                    Ok(_) => {},
                    Err(_) => println!("Could not send the exit signal")
                };
            }
        }

        // if all of them say yes, re-initialize the game
        if play_again {
            deck = Sequence::multi_deck(config.n_decks, config.n_jokers, &mut rng);
            hands = vec![Sequence::new(); config.n_players as usize];
            table = Table::new();
            for i in 0..config.n_players {
                for _ in 0..config.n_cards_to_start {
                    hands[i as usize].add_card(deck.draw_card().unwrap());
                }
            }

            // update the starting player
            starting_player += 1;
            if starting_player >= config.n_players {
                starting_player = 0;
            }
            player = starting_player as usize;
        }
    }

    // send the exit signal to all clients
    for i in 0..config.n_players as usize {
        match client_streams[i].write(&mut [5]) {
            Ok(_) => {},
            Err(_) => println!("Could not send the exit signal to client {}", i)
        };
    }

}

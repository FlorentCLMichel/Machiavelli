//! Server for the Machiavelli game

use std::process;
use std::fs::File;
use std::thread;
use rand::thread_rng;
use machiavelli::lib_server::*;

// ask for the port
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

    // ask if a previous game should be loaded
    println!("Load a previous game? (y/n)");
    let load = match get_input().unwrap().trim() {
        "y" => true,
        _ => false
    };
        
    let mut config = Config {
            n_decks: 0,
            n_jokers: 0,
            n_cards_to_start: 0,
            custom_rule_jokers: false,
            n_players: 0
    };

    if !load {
        // get the config
        let (config, savefile) = match get_config_from_file(&"Config/config.dat") {
            Ok(conf) => conf,
            Err(_) => {
                println!("Could not read the config from the file!");
                match get_config_and_savefile() {
                    Ok(conf) => conf, 
                    Err(_) => {
                        println!("Invalid input!");
                        process::exit(1);
                    }
                }
            }
        };
    }
    
    // create the table
    let mut table = Table::new();
    let mut deck = Sequence::new();
    let mut hands = Vec::<Sequence>::new();
    let mut player: usize = 0;
    let mut player_names = Vec::<String>::new();

    if load {
        
        // load the previous game
        println!("Name of the save file:");
        let mut fname = String::new();
        let mut bytes = Vec::<u8>::new();
        let mut retry = true;
        while retry {

            retry = false;
            
            // get the file name
            match stdin().read_line(&mut fname) {
                Ok(_) => (),
                Err(_) => retry = true
            };

            fname = fname.trim().to_string();

            if !retry {

                // load the data from the file
                let mut file: File; 
                match File::open(fname.clone()) {
                    Ok(f) => file = f,
                    Err(_) => {
                        println!("Could not open the file!");
                        retry = true;
                        fname.clear();
                        continue;
                    }
                };
                match file.read_to_end(&mut bytes) {
                    Ok(_) => (),
                    Err(_) => {
                        println!("Could not read from the file!");
                        retry = true;
                        bytes.clear();
                        fname.clear();
                    }
                };
                
                // decode the sequence of bytes
                bytes = encode::xor(&bytes, &fname.as_bytes());

                match load_game(&bytes) {
                    Ok(lg) => {
                        config = lg.0;
                        player = lg.1 as usize; 
                        table = lg.2;
                        hands = lg.3; 
                        deck = lg.4;
                        player_names = lg.5;
                        bytes = Vec::<u8>::new();
                    },
                    Err(_) => {
                        println!("Error loading the save file!");
                    }
                };
            }
        }

    } else {

        // build the deck
        let mut rng = thread_rng();
        deck = Sequence::multi_deck(config.n_decks, config.n_jokers, &mut rng);
        
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
    let mut client_threads = Vec::<thread::JoinHandle<(TcpStream, String)>>::new();
    
    // vector of client streams
    let mut client_streams = Vec::<TcpStream>::new();
    
    // accept connections and process them, each in its own thread
    println!("\nserver listening to port {}", port);
    for stream_res in listener.incoming() {
        match stream_res {
            Ok(stream) => {
                n_clients += 1;
                println!("New connection: {} (player {})", stream.peer_addr().unwrap(), n_clients);
                client_threads.push(thread::spawn(move || {handle_client(stream)}));
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
    for thread in client_threads {
        let output = thread.join().unwrap();
        client_streams.push(output.0);
        player_names.push(output.1);
    }

    // check that no players have the same name; if yes, rename players
    ensure_names_are_different(&mut player_names, &mut client_streams);

    // Send a message to each player
    send_message_all_players(&mut client_streams, &"All players have joined!\n").unwrap();
         
    long_wait();

    loop {
        
        // if all the cards have been drawn, stop the game
        if deck.number_cards() == 0 {
            send_message_all_players(&mut client_streams, &"No more cards in the deck—it's a draw!\n")
                .unwrap();
            break;
        }

 
        // print the name of the current player 
        clear_and_send_message_all_players(&mut client_streams, 
                                           &format!("\x1b[1m{}'s turn:{}", 
                                                    &player_names[player], &reset_style_string()))
            .unwrap();
        
        // print the situation for each player
        for i in 0..(config.n_players as usize) {
            send_message_to_client(&mut client_streams[i], 
                               &situation_to_string(&table, &hands[i], &deck)).unwrap();
        }


        // player turn
        start_player_turn(&mut table, &mut hands, &mut deck, 
                          config.custom_rule_jokers, &player_names[player],
                          player, config.n_players as usize, &mut client_streams)
                          .unwrap();
        
 
        // if the player has no more cards, stop the game
        if hands[player].number_cards() == 0 {
            send_message_all_players(&mut client_streams, 
                                     &format!("{} wins! Congratulations!", player_names[player]))
                .unwrap();
            break;
        }
        
        // next player
        player += 1;
        if player >= config.n_players as usize {
            player = 0;
        }

    }

}

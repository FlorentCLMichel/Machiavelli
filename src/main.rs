//! # Machiavelli
//!
//! A simple machiavelli card game *(work in progress)*

use std::process;
use std::io::{ stdin, Write };
use std::fs::File;
use rand::thread_rng;
use machiavelli::*;

fn main() {

    // set the style
    reset_style();

    // clear the terminal
    print!("\x1b[2J\x1b[1;1H");

    // get the config
    println!("Hi there! Up for a game of Machiavelli?\n");
    let config = match get_config() {
        Ok(conf) => conf, 
        Err(_) => {
            println!("Invalid input!");
            process::exit(1);
        },
    };

    // build the deck
    let mut rng = thread_rng();
    let mut deck = Sequence::multi_deck(config.n_decks, config.n_jokers_per_deck, &mut rng);
    
    // build the hands
    let mut hands = vec![Sequence::new(); config.n_players as usize];
    for i in 0..config.n_players {
        for _ in 0..config.n_cards_to_start {
            hands[i as usize].add_card(deck.draw_card().unwrap());
        }
    }

    // create the table
    let mut table = Table::new();
    
    // play until a player wins, there is no card left in the deck, or the player decides to save
    // and quit
    let mut player: u8 = 0;
    let mut save_and_quit: bool;
    loop {
        if deck.number_cards() == 0 {
            println!("It's a draw!");
            break;
        }
        save_and_quit = player_turn(&mut table, &mut hands[player as usize], 
                                    &mut deck, config.custom_rule_jokers, player);
        if save_and_quit {
            
            let bytes = game_to_bytes(player, &table, &hands, &deck, &config);

            println!("Name of the save file:");
            let mut fname = String::new();
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

                    // save the data to the file
                    let mut file: File; 
                    match File::create(fname.clone()) {
                        Ok(f) => file = f,
                        Err(_) => {
                            println!("Could not create the file!");
                            retry = true;
                            continue;
                        }
                    };
                    match file.write_all(&bytes) {
                        Ok(_) => (),
                        Err(_) => {
                            println!("Could not create the file!");
                            retry = true;
                        }
                    };
                }
            }

            break;
        }
        if hands[player as usize].number_cards() == 0 {
            println!("Player {} wins! Congratulations!", player+1);
            break;
        }
        player = (player + 1) % config.n_players;
    }
    
    // reset the style
    println!("\x1b[0m");
    print!("\x1b[?25h");
}

//! # Machiavelli
//!
//! A simple machiavelli card game *(work in progress)*

use std::process;
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
    
    // play until a player wins or there is no card left in the deck
    let mut player: u8 = 0;
    loop {
        if deck.number_cards() == 0 {
            println!("It's a draw!");
            break;
        }
        player_turn(&mut table, &mut hands[player as usize], &mut deck, config.custom_rule_jokers, player);
        if hands[player as usize].number_cards() == 0 {
            println!("Player {} wins! Congratulations!", player+1);
            break;
        }
        player = (player + 1) % config.n_players;
    }
    
    // reset the style
    println!("\x1b[0m");
}

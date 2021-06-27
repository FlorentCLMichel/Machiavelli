//! # Machiavelli
//!
//! A simple machiavelli card game *(work in progress)*

use std::process;
use rand::thread_rng;
use machiavelli::*;

fn main() {
    
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
    
    // build the hand
    let mut hand = Sequence::new();
    for _ in 0..config.n_cards_to_start {
        hand.add_card(deck.draw_card().unwrap());
    }

    // create the table
    let mut table = Table::new();
    
    // play until the player wins or there is no card left in the deck
    loop {
        if deck.number_cards() == 0 {
            println!("You lost!");
            break;
        }
        player_turn(&mut table, &mut hand, &mut deck);
        if hand.number_cards() == 0 {
            println!("You win!");
            break;
        }
    }

}

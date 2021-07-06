//! # Machiavelli
//!
//! A simple machiavelli card game *(work in progress)*


use std::io::stdin;
pub mod sequence_cards;
pub mod table;
pub mod sort;
pub mod encode;
pub use sequence_cards::*;
pub use table::*;


pub fn reset_style() {
    
    // reset attributes
    print!("\x1b[0m");
    
    // set the background color
    print!("\x1b[48;2;230;230;220m");
    
    // set the foreground color
    print!("\x1b[38;2;0;0;0m");
    
    // hide the cursor
    print!("\x1b[?25l");

}


pub struct Config {
    pub n_decks: u8,
    pub n_jokers_per_deck: u8,
    pub n_cards_to_start: u16,
    pub custom_rule_jokers: bool,
    pub n_players: u8
}


pub fn get_config() -> Result<Config,InvalidInputError> {
    
    println!("Number of decks (integer between 1 and 255): ");
    let mut n_decks: u8 = 0;
    while n_decks == 0 {
        n_decks = match get_input()?.trim().parse::<u8>() {
            Ok(0) => {
                println!("You need at least one deck");
                0
            },
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                0
            }
        };
    }
    
    println!("Number of jokers per deck (integer between 1 and 255): ");
    let mut n_jokers_per_deck: u8 = 0; 
    let mut set = false;
    while !set {
        n_jokers_per_deck = match get_input()?.trim().parse::<u8>() {
            Ok(n) => {
                set = true;
                n
            },
            Err(_) => {
                println!("Invalid input");
                0
            }
        };
    }
    
    println!("Number of cards to start with (integer): ");
    let mut n_cards_to_start: u16 = 0;
    while n_cards_to_start == 0 {
        n_cards_to_start = match get_input()?.trim().parse::<u16>() {
            Ok(n) => {
                let mut res = 0;
                if n==0 {
                    println!("You need to start with at least one card");
                } else if n > (52+n_jokers_per_deck as u16) * (n_decks as u16) {
                    println!("You can't draw more cards than there are in the deck");
                } else {
                    res = n;
                }
                res
            },
            Err(_) => return Err(InvalidInputError {})
        };
    }
    
    println!("Custom rule—jokers must be played immediately (y/n): ");
    let custom_rule_jokers = match get_input()?.trim() {
        "y" => true,
        _ => false
    };
    
    println!("Number of players: ");
    let mut n_players = 0;
    while n_players == 0 {
        n_players = match get_input()?.trim().parse::<u8>() {
            Ok(0) => {
                println!("I need at least one player!");
                0
            }
            Ok(n) => n,
            Err(_) => {
                println!("Could not parse the input");
                0
            }
        };
    }

    Ok(Config {
        n_decks, 
        n_jokers_per_deck,
        n_cards_to_start,
        custom_rule_jokers,
        n_players
    })
}


pub fn player_turn(table: &mut Table, hand: &mut Sequence, deck: &mut Sequence, 
                   custom_rule_jokers: bool, player: u8) {

    // copy the initial hand
    let hand_start_round = hand.clone();

    // get the player choice
    loop {
        
        // clear the terminal
        print!("\x1b[2J\x1b[1;1H");
        
        println!("\x1b[1mPlayer {}'s turn", player+1);
        reset_style();
        
        print_situation(table, hand, deck);

        // print the options
        println!("\n1: Pick a card\n2: Play a sequence\n3: Take from the table\n4: Pass\n5, 6: Sort cards by rank or suit");
        
        match get_input().unwrap_or_else(|_| {"".to_string()})
              .trim().parse::<u16>() {
            Ok(1) => {
                if !hand_start_round.contains(hand) {
                    println!("You can't pick a card until you've played all the cards you've taken from the table!");
                } else if !hand.contains(&hand_start_round) {
                    println!("You can't pick a card after having played something");
                } else if custom_rule_jokers && hand.contains_joker() {
                    println!("Jokers need to be played!");
                } else {
                    match pick_a_card(hand, deck) {
                        Ok(card) => println!("You have picked a {}\x1b[38;2;0;0;0;1m", &card),
                        Err(_) => println!("No more card to draw!")
                    };
                    break
                }
            },
            Ok(2) => {
                play_sequence(hand, table);
                print_situation(table, hand, deck);
            },
            Ok(3) => {
                take_sequence(table, hand);
                print_situation(table, hand, deck);
            },
            Ok(4) => {
                if !hand_start_round.contains(hand) {
                    println!("{:?}", hand_start_round);
                    println!("You can't pass until you've played all the cards you've taken from the table!");
                } else if hand.contains(&hand_start_round) {
                    println!("You need to play something to pass");
                } else if custom_rule_jokers && hand.contains_joker() {
                    println!("Jokers need to be played!");
                } else {
                    break
                }
            }
            Ok(5) => {
                hand.sort_by_rank();
                print_situation(table, hand, deck);
            },
            Ok(6) => {
                hand.sort_by_suit();
                print_situation(table, hand, deck);
            },
            _ => ()
        };
    }
}


fn print_situation(table: &mut Table, hand: &mut Sequence, deck: &mut Sequence) {
    
    // print the table
    println!("\nTable:\n{}", table);

    // print the player hand
    println!("Your hand:\n{}", hand);
    reset_style();

    // print the number of remaining cards in the deck
    println!("\nRemaining cards in the deck: {}", deck.number_cards());

}


fn get_input() -> Result<String, InvalidInputError> {
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err(InvalidInputError {})
    }
    Ok(buffer)
}


fn pick_a_card(hand: &mut Sequence, deck: &mut Sequence) -> Result<Card, NoMoreCards> {
    let card = match deck.draw_card() {
        Some(c) => c,
        None => return Err(NoMoreCards {})
    };
    hand.add_card(card.clone());
    Ok(card)
}


fn play_sequence(hand: &mut Sequence, table: &mut Table) {
    println!("Please enter the sequence, in order, separated by spaces");
    let hand_and_indices = hand.show_indices();
    println!("{}", hand_and_indices.0);
    reset_style();
    println!("{}", hand_and_indices.1);
    let mut seq = Sequence::new();
    
    let mut s = get_input().unwrap_or_else(|_| {"".to_string()});
    s.pop();
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
    } else {
        println!("{} is not a valid sequence!", &seq);
        hand.merge(seq);
    }
}


fn take_sequence(table: &mut Table, hand: &mut Sequence) {
    println!("Which sequence would you like to take?");
    match get_input().unwrap_or_else(|_| {"".to_string()})
          .trim().parse::<usize>() {
        Ok(n) => match table.take(n) {
            Some(seq) => {
                hand.merge(seq);
            },
            None => println!("This sequence is not on the table")
        },
        Err(_) => ()
    };
}


pub struct InvalidInputError {}
pub struct NoMoreCards {}



//! # Machiavelli
//!
//! A simple machiavelli card game *(work in progress)*


use std::io::stdin;
pub mod sequence_cards;
pub mod table;
pub mod sort;
pub use sequence_cards::*;
pub use table::*;


pub struct Config {
    pub n_decks: u8,
    pub n_jokers_per_deck: u8,
    pub n_cards_to_start: u16,
    pub custom_rule_jokers: bool,
    pub n_players: u8
}


pub fn get_config() -> Result<Config,InvalidInputError> {
    
    println!("Number of decks (integer between 1 and 255): ");
    let n_decks = match get_input()?.trim().parse::<u8>() {
        Ok(0) => return Err(InvalidInputError {}),
        Ok(n) => n,
        Err(_) => return Err(InvalidInputError {})
    };
    
    println!("Number of jokers per deck (integer between 1 and 255): ");
    let n_jokers_per_deck = match get_input()?.trim().parse::<u8>() {
        Ok(n) => n,
        Err(_) => return Err(InvalidInputError {})
    };
    
    println!("Number of cards to start with (integer): ");
    let n_cards_to_start = match get_input()?.trim().parse::<u16>() {
        Ok(n) => {
            if n > (52+n_jokers_per_deck as u16) * (n_decks as u16) {
                println!("You can't draw more cards than there are in the deck");
                return Err(InvalidInputError {});
            }
            n
        },
        Err(_) => return Err(InvalidInputError {})
    };
    
    println!("Custom rule—jokers must be played immediately (y/n): ");
    let custom_rule_jokers = match get_input()?.trim() {
        "y" => true,
        _ => false
    };
    
    println!("Number of players: ");
    let n_players = match get_input()?.trim().parse::<u8>() {
        Ok(n) => {
            if n > 0 {
                n
            } else {
                return Err(InvalidInputError {});
            }
        },
        Err(_) => return Err(InvalidInputError {})
    };

    Ok(Config {
        n_decks, 
        n_jokers_per_deck,
        n_cards_to_start,
        custom_rule_jokers,
        n_players
    })
}


pub fn player_turn(table: &mut Table, hand: &mut Sequence, deck: &mut Sequence, 
                   custom_rule_jokers: bool) {

    print_situation(table, hand, deck);

    // print the options
    println!("\n1: Pick a card\n2: Pick a nose\n3: Play a sequence\n4: Take from the table\n5: Pass\n6, 7: Sort cards by rank or suit");

    // copy the initial hand
    let hand_start_round = hand.clone();

    // get the player choice
    loop {
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
                    pick_a_card(hand, deck).unwrap_or_else(|_| {println!("No more card to draw!");});
                    break
                }
            },
            Ok(2) => println!("Not yet implemented!"),
            Ok(3) => {
                play_sequence(hand, table);
                print_situation(table, hand, deck);
            },
            Ok(4) => {
                take_sequence(table, hand);
                print_situation(table, hand, deck);
            },
            Ok(5) => {
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
            Ok(6) => {
                hand.sort_by_rank();
                print_situation(table, hand, deck);
            },
            Ok(7) => {
                hand.sort_by_suit();
                print_situation(table, hand, deck);
            },
            _ => ()
        };
    }
}


fn print_situation(table: &mut Table, hand: &mut Sequence, deck: &mut Sequence) {
    
    // print a few empty lines
    let lines_between_rounds = 3;
    println!("{}", "\n".repeat(lines_between_rounds));
    
    // print the table
    println!("Table:\n{}", table);

    // print the player hand
    println!("Your hand:\n{}", hand);

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


fn pick_a_card(hand: &mut Sequence, deck: &mut Sequence) -> Result<(), NoMoreCards> {
    let card = match deck.draw_card() {
        Some(c) => c,
        None => return Err(NoMoreCards {})
    };
    hand.add_card(card);
    Ok(())
}


fn play_sequence(hand: &mut Sequence, table: &mut Table) {
    println!("Please enter the sequence, in order, separated by spaces");
    let mut seq = Sequence::new();
    
    let mut s = get_input().unwrap_or_else(|_| {"".to_string()});
    s.pop();
    let mut n_in_seq: usize = 0;
    for item in s.split(' ') {
        match item.parse::<usize>() {
            Ok(n) => {
                let card = match hand.take_card(n-n_in_seq) {
                    Some(c) => c,
                    None => continue
                };
                seq.add_card(card);
                n_in_seq += 1;
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



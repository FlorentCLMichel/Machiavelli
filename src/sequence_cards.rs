//! Define representations for cards and sequences of cards.

use std::fmt;
use std::collections::HashMap;
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
use crate::sort::sort;
pub use Card::*;
pub use Suit::*;

static MAX_VAL: u8 = 13;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Card {
    RegularCard(Suit, u8),
    Joker
}

fn suit_to_int(suit: Suit) -> u8 {
    match suit {
        Heart => 1,
        Club => 2,
        Diamond => 3,
        Spade => 4,
    }
}

fn int_to_suit(s: u8) -> Option<Suit> {
    match s {
        1 => Some(Heart),
        2 => Some(Club),
        3 => Some(Diamond),
        4 => Some(Spade),
        _ => None
    }
}

impl Card {

    fn from_byte(x: u8) -> Card {
        if x == 0 {
            return Joker;
        }
        let mut val = x % MAX_VAL;
        if val == 0 {
            val = MAX_VAL;
        }
        return RegularCard(int_to_suit((x-1) / MAX_VAL + 1).unwrap(), val)
    }

    fn to_byte(&self) -> u8 {
        match self {
            Joker => 0,
            RegularCard(suit, value) => (suit_to_int(*suit)-1) * MAX_VAL + value
        }
    }

}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegularCard(suit, val) => {
                let str_val = match val {
                    1 => "A".to_string(),
                    11 => "J".to_string(),
                    12 => "Q".to_string(),
                    13 => "K".to_string(),
                    10 => "10".to_string(),
                    _ => format!("{}", val)
                };
                let char_suit = match suit {
                    Heart => '♥',
                    Diamond => '♦',
                    Club => '♣',
                    Spade => '♠',
                };
                let color = match suit {
                    Heart => "1;31",
                    Diamond => "1;31",
                    Club => "1;30",
                    Spade => "1;30",
                };
                write!(f, "\x1b[{}m{}{}", color, str_val, char_suit)
            }
            Joker => write!(f, "\x1b[1;34m🃏")
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Sequence(Vec<Card>);

impl Sequence {

    /// Create an empty sequence of cards
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::Sequence;
    ///
    /// let sequence = Sequence::new();
    ///
    /// assert_eq!(0, sequence.number_cards());
    /// ```
    pub fn new() -> Sequence {
        Sequence(Vec::<Card>::new())
    }

    /// Create a sequence from an array of cards
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3)
    /// ];
    /// let sequence = Sequence::from_cards(&cards);
    ///
    /// assert_eq!(4, sequence.number_cards());
    /// ```
    pub fn from_cards(cards: &[Card]) -> Sequence {
        Sequence(cards.to_vec())
    }
    
    /// Create a sequence from an array of bytes
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let bytes: [u8; 5] = [0, 1, 20, 35, 51];
    /// let sequence = Sequence::from_bytes(&bytes);
    ///
    /// assert_eq!(Sequence::from_cards(&[
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Club, 7),
    ///     RegularCard(Diamond, 9),
    ///     RegularCard(Spade, 12),
    /// ]), sequence);
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> Sequence {
        let cards: Vec<Card> = bytes.into_iter().map(|x| Card::from_byte(*x)).collect();
        Sequence::from_cards(&cards)
    }
    
    /// Convert a sequence to an array of bytes
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Diamond, 7),
    ///     RegularCard(Club, 9),
    ///     RegularCard(Spade, 12),
    /// ];
    /// let sequence = Sequence::from_cards(&cards);
    ///
    /// let bytes = sequence.to_bytes();
    ///
    /// assert_eq!(vec![0, 1, 33, 22, 51], bytes);
    /// ```
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::<u8>::new();
        for card in &self.0 {
            res.push(card.to_byte());
        }
        res
    }

    /// Return the number of cards in the sequence
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11)
    /// ];
    /// let sequence = Sequence::from_cards(&cards);
    ///
    /// assert_eq!(5, sequence.number_cards());
    /// ```
    pub fn number_cards(&self) -> usize {
        self.0.len()
    }
    
    /// Return a string with the indices
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 10),
    ///     Joker,
    ///     RegularCard(Club, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Club, 3),
    ///     RegularCard(Club, 4),
    ///     RegularCard(Club, 10),
    ///     RegularCard(Club, 6),
    ///     RegularCard(Club, 10),
    /// ];
    /// let sequence = Sequence::from_cards(&cards);
    ///
    /// assert_eq!(sequence.show_indices(), 
    ///            ("\u{1b}[38;2;0;0;255m★ \u{1b}[38;2;255;0;0mA♥ \u{1b}[38;2;255;0;0m3♥ \u{1b}[38;2;0;0;0m10♣ \u{1b}[38;2;0;0;255m★ \u{1b}[38;2;0;0;0mA♣ \u{1b}[38;2;255;0;0m2♥ \u{1b}[38;2;0;0;0m3♣ \u{1b}[38;2;0;0;0m4♣ \u{1b}[38;2;0;0;0m10♣ \u{1b}[38;2;0;0;0m6♣ \u{1b}[38;2;0;0;0m10♣".to_string(),
    ///             "1 2  3  4   5 6  7  8  9  10  11 12".to_string()));
    /// ```
    pub fn show_indices(&self) -> (String,String) {

        let mut first_line = String::new();
        let mut second_line = String::new();
        let mut n_chars_1: usize;
        let mut n_chars_2: usize = 2;
        let mut power_of_ten: usize = 10;
        for i in 1..=self.0.len() {
            
            // if i is a power of 10, increase the number of characters for the second line by 1
            if i==power_of_ten {
                n_chars_2 += 1;
                power_of_ten *= 10;
            }
            
            // print the current card with a space
            let current_card = &self.0[i-1];
            first_line.push_str(&format!("{} ", current_card));
            
            // see hom many characters the current caerd take
            match current_card {
                Joker => n_chars_1 = 2,
                RegularCard(_,10) => n_chars_1 = 4,
                _ => n_chars_1 = 3
            };

            // print the index
            second_line.push_str(&format!("{} ", i));

            // pad the first line with spaces if necessary
            for _ in n_chars_1..n_chars_2 {
                first_line.push_str(&" ");
            }
            
            // pad the second line with spaces if necessary
            for _ in n_chars_2..n_chars_1 {
                second_line.push_str(&" ");
            }
        }
        
        first_line = first_line.trim().to_string();
        second_line = second_line.trim().to_string();
        return (first_line.to_string(), second_line.to_string());
    }
    
    /// Sort cards by suit
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11),
    ///     RegularCard(Club, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Club, 3)
    /// ];
    /// let mut sequence = Sequence::from_cards(&cards);
    /// sequence.sort_by_suit();
    ///
    /// assert_eq!(
    ///     Sequence::from_cards(&[
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 1),
    ///     RegularCard(Club, 3),
    ///     RegularCard(Club, 11),
    ///     Joker
    ///     ]),
    ///     sequence);
    /// ```
    pub fn sort_by_suit(&mut self) {
        self.0 = sort(&self.0, Box::new(value_card_by_suit));
    }
    

    /// Sort cards by rank
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11),
    ///     RegularCard(Club, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Club, 3)
    /// ];
    /// let mut sequence = Sequence::from_cards(&cards);
    /// sequence.sort_by_rank();
    ///
    /// assert_eq!(
    ///     Sequence::from_cards(&[
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Club, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 3),
    ///     RegularCard(Club, 11),
    ///     Joker
    ///     ]),
    ///     sequence);
    /// ```
    pub fn sort_by_rank(&mut self) {
        self.0 = sort(&self.0, Box::new(value_card_by_rank));
    }

    /// Merge the sequence with another one
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards_1 = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    /// ];
    /// let cards_2 = [
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11)
    /// ];
    /// let mut sequence_1 = Sequence::from_cards(&cards_1);
    /// let sequence_2 = Sequence::from_cards(&cards_2);
    ///
    /// sequence_1.merge(sequence_2);
    ///
    /// assert_eq!(5, sequence_1.number_cards());
    /// ```
    pub fn merge(&mut self, mut seq: Sequence) {
        while let Some(card) = seq.draw_card() {
            self.add_card(card);
        }
    }

    /// Build a randomly-shuffled deck of cards
    ///
    /// # Arguments
    ///
    /// * `n_decks`: the number of copies of a full deck of 52 cards
    /// * `n_jokers_per_deck`: the number of jokers per deck of 52 cards
    /// * `rng`: mutable reference to the random-number generator used foor shuffling
    ///
    /// # Example
    ///
    /// ```
    /// use rand::thread_rng;
    /// use machiavelli::sequence_cards::Sequence;
    ///
    /// let mut rng = thread_rng();
    /// let sequence = Sequence::multi_deck(3, 6, &mut rng);
    ///
    /// assert_eq!(162, sequence.number_cards());
    /// ```
    pub fn multi_deck(n_decks: u8, n_jokers: u8, rng: &mut ThreadRng) -> Sequence {
        
        let mut deck = Sequence::new();

        for _i in 0..n_decks {

            // add the regular cards
            for val in 1..=MAX_VAL {
                for suit in &[Heart, Diamond, Club, Spade] {
                    deck.add_card(RegularCard(*suit, val));
                }
            }
        }
            
        // add the jokers
        for _j in 0..n_jokers {
            deck.add_card(Joker);
        }

        // shuffle the deck
        deck.shuffle(rng);

        deck
    }
    
    /// Add a card to a sequence
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11)
    /// ];
    /// let mut sequence = Sequence::from_cards(&cards);
    /// sequence.add_card(Joker);
    ///
    /// assert_eq!(6, sequence.number_cards());
    /// ```
    pub fn add_card(&mut self, card: Card) {
        &self.0.push(card);
    }
    
    /// Draw the top card from a sequence
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11)
    /// ];
    /// let mut sequence = Sequence::from_cards(&cards);
    /// let card = sequence.draw_card().unwrap();
    ///
    /// assert_eq!(4, sequence.number_cards());
    /// assert_eq!(RegularCard(Club, 11), card);
    /// ```
    pub fn draw_card(&mut self) -> Option<Card> {
        self.0.pop()
    }
    
    /// Take a card from a sequence
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     Joker, 
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11)
    /// ];
    /// let mut sequence = Sequence::from_cards(&cards);
    /// let card = sequence.take_card(2).unwrap();
    ///
    /// assert_eq!(4, sequence.number_cards());
    /// assert_eq!(RegularCard(Heart, 1), card);
    /// ```
    pub fn take_card(&mut self, i: usize) -> Option<Card> {
        if (i>0) && (i<= self.0.len()) {
            let card = self.0[i-1].clone();
            self.0.remove(i-1);
            return Some(card);
        } 
        None
    }
    
    /// Check if a sequence has a joker
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards_1 = Sequence::from_cards(&[
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     Joker, 
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11)
    /// ]);
    /// let cards_2 = Sequence::from_cards(&[
    ///     RegularCard(Heart, 1),
    ///     RegularCard(Heart, 2),
    ///     RegularCard(Heart, 3),
    ///     RegularCard(Club, 11)
    /// ]);
    ///
    /// assert_eq!(true, cards_1.contains_joker());
    /// assert_eq!(false, cards_2.contains_joker());
    /// ```
    pub fn contains_joker(&self) -> bool {
        for card in &self.0 {
            if *card == Joker {
                return true;
            }
        }
        false
    }

    /// Check if a sequence if valid for the Machiavelli game
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::sequence_cards::{ Sequence, Card::* , Suit::*};
    ///
    /// let cards = [
    ///     RegularCard(Heart, 1),
    ///     Joker, 
    ///     RegularCard(Heart, 3),
    /// ];
    /// let mut sequence = Sequence::from_cards(&cards);
    ///
    /// assert!(sequence.is_valid());
    /// ```
    pub fn is_valid(&mut self) -> bool {
        
        if self.0.len() == 0 {
            return false;
        }
        
        if self.has_only_jokers() {
            return true;
        }

        if self.0.len() < 3 {
            return false;
        }

        // sort the equence
        self.sort_by_rank();

        // if the first card is an ace and the second one is not a two, put tha ace at the end
        if self.number_cards() > 1 {
            match self.0[0] {
                RegularCard(_, 1) => {
                    match self.0[1] {
                        RegularCard(_, n) => {
                            if n > 2 {
                                let ace = self.0[0].clone();
                                self.0 = self.0[1..].to_vec();
                                self.0.push(ace);
                            }
                        }
                        _ => ()
                    }
                }
                _ => ()
            };
        }
       
        if self.is_valid_sequence_same_suit() {
            return true;
        }

        if self.is_valid_sequence_same_val() {
            return true;
        }
        
        false
    }

    /// return the vector of cards
    pub fn to_vec(&self) -> Vec<Card> {
        self.0.clone()
    }

    /// determine if the sequence contains another one
    pub fn contains(&self, seq: &Sequence) -> bool {
        let count_rhs = seq.count_cards();
        let count_self = self.count_cards();
        for (card, count) in count_rhs {
            if !count_self.contains_key(&card) {
                return false;
            }
            if count_self[&card] < count {
                return false;
            }
        }
        true
    }
 
    fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.0.shuffle(rng);
    }
        
    fn has_only_jokers(&self) -> bool {
        for card in &self.0 {
            if *card != Joker {
                return false;
            }
        }
        true
    }

    fn n_jokers(&self) -> u8 {
        let mut res = 0;
        for card in &self.0 {
            match card {
                Joker => res += 1,
                _ => ()
            };
        }
        res
    }

    fn is_valid_sequence_same_val(&self) -> bool {
        let mut suits_in_seq = Vec::<Suit>::new();
        let mut common_value: u8 = 0;
        for card in &self.0 {
            match card {
                RegularCard(suit, value) => {
                    if common_value == 0 {
                        common_value = *value;
                    }
                    else if (suits_in_seq.contains(&*suit)) || (*value != common_value) {
                        return false
                    }
                    suits_in_seq.push(*suit);
                }
                Joker => ()
            }
        }
        true
    }

    fn is_valid_sequence_same_suit(&self) -> bool {
        let mut n_available_jokers = self.n_jokers();
        let mut common_suit = Club;
        let mut current_value: u8 = 0;
        for card in &self.0 {
            match card {
                RegularCard(suit, value) => {
                    if current_value == 0 {
                        common_suit = *suit;
                        current_value = *value;
                    } else {
                        if *suit != common_suit {
                            return false
                        }
                        if (*value != current_value + 1)
                              &&
                              ((current_value < MAX_VAL) || (*value != 1))
                        {
                            let next_val = match *value {
                                1 => MAX_VAL + 1,
                                n => n
                            };
                            if next_val < (current_value + 1) {
                                return false;
                            }
                            let diff = next_val - current_value - 1;
                            if diff > n_available_jokers {
                                return false;
                            }
                            n_available_jokers -= diff;
                        }
                        current_value += 1;
                    }
                }
                Joker => {
                    if current_value > 0 {
                        current_value += 1;
                    }
                }
            }
        }
        true
    }

    fn count_cards(&self) -> HashMap<Card, u16> {
        let mut res = HashMap::<Card, u16>::new();
        
        for card in &self.0 {
            if res.contains_key(card) {
                *res.get_mut(card).unwrap() += 1;
            } else {
                res.insert(card.clone(), 1);
            }
        }
        
        res
    }

}


impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.0 {
            card.fmt(f)?;
            write!(f, " ")?;
        }
        write!(f, "")
    }
}


fn value_card_by_suit(card: &Card) -> u8 {
    match *card {
        Joker => 255,
        RegularCard(suit, val) => (MAX_VAL + 1) * suit_to_int(suit) + val
    }
}


fn value_card_by_rank(card: &Card) -> u8 {
    match *card {
        Joker => 255,
        RegularCard(suit, val) => 4 * val + suit_to_int(suit)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use Card::{ RegularCard, Joker };
    use rand::thread_rng;
    
    #[test]
    fn sequence_two_jokers() {
        let seq = Sequence::from_cards(&[
            Joker, 
            Joker
        ]);
        assert_eq!(seq.is_valid(), true);
    }

    #[test]
    fn sequence_three_jokers() {
        let seq = Sequence::from_cards(&[
            Joker, 
            Joker, 
            Joker
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_only_jokers_1() {
        let seq = Sequence::from_cards(&[
            Joker, 
            Joker, 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_only_jokers_2() {
        let seq = Sequence::from_cards(&[
            Joker, 
            Joker, 
            Joker, 
            Joker, 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_1() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 1), 
            RegularCard(Heart, 2), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_2() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 1), 
            RegularCard(Club, 2), 
            RegularCard(Club, 3), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_3() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 2), 
            RegularCard(Club, 3), 
            RegularCard(Club, 4), 
            RegularCard(Club, 5), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_4() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 2), 
            RegularCard(Club, 3), 
            RegularCard(Club, 5), 
            RegularCard(Club, 6), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_5() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 2), 
            RegularCard(Club, 3), 
            RegularCard(Club, 3), 
            RegularCard(Club, 4), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_6() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 12), 
            RegularCard(Club, 13), 
            RegularCard(Club, 1), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_7() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 13), 
            RegularCard(Club, 1), 
            RegularCard(Club, 2), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_8() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 1), 
            RegularCard(Heart, 2), 
            RegularCard(Heart, 3), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_9() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            RegularCard(Heart, 3), 
            RegularCard(Heart, 4), 
            RegularCard(Heart, 5), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_10() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            RegularCard(Heart, 3), 
            RegularCard(Heart, 5), 
            RegularCard(Heart, 6), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_11() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            RegularCard(Heart, 3), 
            RegularCard(Heart, 3), 
            RegularCard(Heart, 4), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_12() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 12), 
            RegularCard(Heart, 13), 
            RegularCard(Heart, 1), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_13() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 13), 
            RegularCard(Heart, 1), 
            RegularCard(Heart, 2), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_one_j_1() {
        let seq = Sequence::from_cards(&[
            RegularCard(Diamond, 2), 
            RegularCard(Diamond, 3), 
            Joker, 
            RegularCard(Diamond, 5), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_suit_one_j_2() {
        let seq = Sequence::from_cards(&[
            RegularCard(Diamond, 2), 
            RegularCard(Diamond, 3), 
            Joker, 
            RegularCard(Diamond, 6), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_suit_two_j_1() {
        let seq = Sequence::from_cards(&[
            Joker, 
            RegularCard(Diamond, 3), 
            Joker, 
            RegularCard(Diamond, 5), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_val_1() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            RegularCard(Diamond, 2), 
            RegularCard(Spade, 2), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_val_2() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            RegularCard(Diamond, 2), 
            RegularCard(Spade, 2), 
            RegularCard(Club, 2), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_val_3() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            RegularCard(Spade, 2), 
            RegularCard(Spade, 2), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_val_one_j_1() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            Joker, 
            RegularCard(Spade, 2), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_val_one_j_2() {
        let seq = Sequence::from_cards(&[
            RegularCard(Heart, 2), 
            Joker, 
            RegularCard(Heart, 2), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn sequence_same_val_two_j_1() {
        let seq = Sequence::from_cards(&[
            Joker, 
            Joker, 
            RegularCard(Spade, 2), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn sequence_same_val_two_j_2() {
        let seq = Sequence::from_cards(&[
            Joker, 
            RegularCard(Club, 2), 
            Joker, 
            RegularCard(Spade, 2), 
        ]);
        assert_eq!(seq.is_valid(), true);
    }
    
    #[test]
    fn invalid_sequence_1() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 2), 
            RegularCard(Spade, 2), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }
    
    #[test]
    fn invalid_sequence_2() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 2), 
            RegularCard(Diamond, 3), 
            RegularCard(Heart, 2), 
        ]);
        assert_eq!(seq.is_valid(), false);
    }

    #[test]
    fn build_deck_1() {
        let mut rng = thread_rng();
        let deck = Sequence::multi_deck(2, 4, &mut rng);
        assert_eq!(108, deck.number_cards());
    }
    
    #[test]
    fn display_sequence_1() {
        let seq = Sequence::from_cards(&[
            RegularCard(Club, 2), 
            Joker,
            RegularCard(Diamond, 3), 
            RegularCard(Heart, 2), 
        ]);
        assert_eq!("\u{1b}[38;2;0;0;0m2♣ \u{1b}[38;2;0;0;255m★ \u{1b}[38;2;255;0;0m3♦ \u{1b}[38;2;255;0;0m2♥ ".to_string(), format!("{}", &seq));
    }

    #[test]
    fn contains_joker_1() {
        let cards = Sequence::from_cards(&[
            Joker, 
            RegularCard(Heart, 1),
            RegularCard(Heart, 2),
            RegularCard(Heart, 3),
            RegularCard(Club, 11)
        ]);
        
        assert_eq!(true, cards.contains_joker());
    }
    
    #[test]
    fn contains_joker_2() {
        let cards = Sequence::from_cards(&[
            RegularCard(Heart, 1),
            RegularCard(Heart, 2),
            RegularCard(Heart, 3),
            RegularCard(Club, 11)
        ]);
        
        assert_eq!(false, cards.contains_joker());
    }

    #[test]
    fn card_to_byte_1() {
        let card = Joker;
        let card_byte = card.to_byte();
        assert_eq!(0, card_byte);
    }
    
    #[test]
    fn card_to_byte_2() {
        let card = RegularCard(Heart, 1);
        let card_byte = card.to_byte();
        assert_eq!(1, card_byte);
    }
    
    #[test]
    fn card_to_byte_3() {
        let card = RegularCard(Diamond, 7);
        let card_byte = card.to_byte();
        assert_eq!(33, card_byte);
    }
    
    #[test]
    fn card_to_byte_4() {
        let card = RegularCard(Club, 9);
        let card_byte = card.to_byte();
        assert_eq!(22, card_byte);
    }
    
    #[test]
    fn card_to_byte_5() {
        let card = RegularCard(Spade, 12);
        let card_byte = card.to_byte();
        assert_eq!(51, card_byte);
    }
    
    #[test]
    fn card_to_byte_6() {
        let card = RegularCard(Spade, 13);
        let card_byte = card.to_byte();
        assert_eq!(52, card_byte);
    }
    
    #[test]
    fn card_to_byte_7() {
        let card = RegularCard(Spade, 1);
        let card_byte = card.to_byte();
        assert_eq!(40, card_byte);
    }
    
    #[test]
    fn byte_to_card_1() {
        let byte = 0;
        let card = Card::from_byte(byte);
        assert_eq!(Joker, card);
    }
    
    #[test]
    fn byte_to_card_2() {
        let byte = 1;
        let card = Card::from_byte(byte);
        assert_eq!(RegularCard(Heart, 1), card);
    }
    
    #[test]
    fn byte_to_card_3() {
        let byte = 20;
        let card = Card::from_byte(byte);
        assert_eq!(RegularCard(Club, 7), card);
    }
    
    #[test]
    fn byte_to_card_4() {
        let byte = 35;
        let card = Card::from_byte(byte);
        assert_eq!(RegularCard(Diamond, 9), card);
    }
    
    #[test]
    fn byte_to_card_5() {
        let byte = 51;
        let card = Card::from_byte(byte);
        assert_eq!(RegularCard(Spade, 12), card);
    }
    
    #[test]
    fn byte_to_card_6() {
        let byte = 52;
        let card = Card::from_byte(byte);
        assert_eq!(RegularCard(Spade, 13), card);
    }
    
    #[test]
    fn byte_to_card_7() {
        let byte = 40;
        let card = Card::from_byte(byte);
        assert_eq!(RegularCard(Spade, 1), card);
    }
}

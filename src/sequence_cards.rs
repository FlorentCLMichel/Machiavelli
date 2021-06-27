//! Define representations for cards and sequences of cards.

use std::fmt;
use rand::seq::SliceRandom;
use rand::rngs::ThreadRng;
pub use Card::*;
pub use Suit::*;

static MAX_VAL: u8 = 13;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade
}

#[derive(Clone, PartialEq, Debug)]
pub enum Card {
    RegularCard(Suit, u8),
    Joker
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RegularCard(suit, val) => {
                let char_suit = match suit {
                    Heart => '♥',
                    Diamond => '♦',
                    Club => '♣',
                    Spade => '♠',
                };
                let str_val = match val {
                    11 => "J ".to_string(),
                    12 => "Q ".to_string(),
                    13 => "K ".to_string(),
                    10 => "10".to_string(),
                    _ => format!("{} ", val)
                };
                write!(f, "{}{}", char_suit, str_val)
            }
            Joker => write!(f, " ★ ")
        }
    }
}

#[derive(Debug, PartialEq)]
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
    /// let sequence = Sequence::multi_deck(3, 2, &mut rng);
    ///
    /// assert_eq!(162, sequence.number_cards());
    /// ```
    pub fn multi_deck(n_decks: u8, n_jokers_per_deck: u8, rng: &mut ThreadRng) -> Sequence {
        
        let mut deck = Sequence::new();

        for _i in 0..n_decks {

            // add the regular cards
            for val in 1..=MAX_VAL {
                for suit in &[Heart, Diamond, Club, Spade] {
                    deck.add_card(RegularCard(*suit, val));
                }
            }

            // add the jokers
            for _j in 0..n_jokers_per_deck {
                deck.add_card(Joker);
            }
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
    
    /// Draw a card from a sequence
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
    pub fn is_valid(&self) -> bool {
        
        if self.0.len() < 3 {
            return false;
        }

        if self.is_valid_sequence_same_suit() {
            return true;
        }

        if self.is_valid_sequence_same_val() {
            return true;
        }
        
        false
    }
 
    fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.0.shuffle(rng);
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
        let mut common_suit = Club;
        let mut current_value: u8 = 0;
        for card in &self.0 {
            match card {
                RegularCard(suit, value) => {
                    if current_value == 0 {
                        common_suit = *suit;
                        current_value = *value;
                    } else {
                        if (*suit != common_suit) || (
                              (*value != current_value + 1)
                              &&
                              ((current_value < MAX_VAL) || (*value != 1))
                           ){
                            return false
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

}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for card in &self.0 {
            card.fmt(f)?;
        }
        write!(f, "")
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
        assert_eq!(seq.is_valid(), false);
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
        let deck = Sequence::multi_deck(2, 2, &mut rng);
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
        assert_eq!("♣2  ★ ♦3 ♥2 ".to_string(), format!("{}", &seq));
    }
}

use std::fmt;
use std::collections::HashMap;
use core::mem::swap;
use crate::sequence_cards::*;
use SequenceList::*;

pub struct Table {
    number_sequences: usize, 
    sequences: SequenceList
}

impl Table {
    
    /// Create a new table with no sequence
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::table::Table;
    ///
    /// let table = Table::new();
    /// ```
    pub fn new() -> Table {
        Table {
            number_sequences: 0,
            sequences: Nil
        }
    }
    
    /// Add a new sequence to a table
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::table::*;
    /// use machiavelli::sequence_cards::*;
    ///
    /// let mut table = Table::new();
    /// table.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 4), 
    ///     RegularCard(Club, 5), 
    ///     RegularCard(Club, 6), 
    /// ]));
    /// table.add(Sequence::from_cards(&[
    ///     RegularCard(Heart, 11), 
    ///     RegularCard(Heart, 12), 
    ///     RegularCard(Heart, 13), 
    /// ]));
    ///
    /// assert_eq!("1: J♥ Q♥ K♥ \n2: 4♣ 5♣ 6♣ \n".to_string(), format!("{}", &table));
    /// ```
    pub fn add(&mut self, sequence: Sequence) {
        let mut buffer = Box::new(Nil);
        swap(&mut self.sequences, &mut buffer);
        self.sequences = SequenceList::Cons(sequence, buffer);
        self.number_sequences += 1;
    }
    
    /// Take a sequence from a table
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::table::*;
    /// use machiavelli::sequence_cards::*;
    ///
    /// let mut table = Table::new();
    /// table.add(Sequence::from_cards(&[
    ///     RegularCard(Spade, 7), 
    ///     RegularCard(Heart, 7), 
    ///     RegularCard(Diamond, 7), 
    /// ]));
    /// table.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 4), 
    ///     RegularCard(Club, 5), 
    ///     RegularCard(Club, 6), 
    /// ]));
    /// table.add(Sequence::from_cards(&[
    ///     RegularCard(Heart, 11), 
    ///     RegularCard(Heart, 12), 
    ///     RegularCard(Heart, 13), 
    /// ]));
    /// 
    /// let mut seq = table.take(2).unwrap();
    ///
    /// assert_eq!(seq, Sequence::from_cards(&[
    ///     RegularCard(Club, 4), 
    ///     RegularCard(Club, 5), 
    ///     RegularCard(Club, 6), 
    /// ]));
    /// assert_eq!("1: J♥ Q♥ K♥ \n2: 7♠ 7♥ 7♦ \n".to_string(), format!("{}", &table));
    ///
    /// seq = table.take(1).unwrap();
    ///
    /// assert_eq!(seq, Sequence::from_cards(&[
    ///     RegularCard(Heart, 11), 
    ///     RegularCard(Heart, 12), 
    ///     RegularCard(Heart, 13), 
    /// ]));
    /// assert_eq!("1: 7♠ 7♥ 7♦ \n".to_string(), format!("{}", &table));
    ///
    /// seq = table.take(1).unwrap();
    ///
    /// assert_eq!(seq, Sequence::from_cards(&[
    ///     RegularCard(Spade, 7), 
    ///     RegularCard(Heart, 7), 
    ///     RegularCard(Diamond, 7), 
    /// ]));
    /// assert_eq!("".to_string(), format!("{}", &table));
    /// ```
    pub fn take(&mut self, n: usize) -> Option<Sequence> {
        
        if (n==0) || (n > self.number_sequences) {
            return None;
        }

        let mut buffer = Box::new(Nil);
        swap(&mut self.sequences, &mut buffer);
        let res: Sequence;

        if n==1 {
            res = match *buffer {
                Cons(seq, box_sl) => {
                    buffer = box_sl;
                    seq
                },
                Nil => Sequence::new()
            }
        } else {
            let mut current_item = &mut *buffer;
            for _i in 2..n {
                match current_item {
                    Cons(_, box_sl) => {
                        current_item = &mut *box_sl;
                    },
                    _ => ()
                }
            }

            let mut tail = Box::new(Nil);
            match &mut current_item {
                Cons(_, box_sl) => {
                    swap(box_sl, &mut tail);
                },
                _ => ()
            };

            res = match *tail {
                Cons(s, mut box_sl) => {
                    match &mut current_item {
                        Cons(_, box_sl_prev) => {
                            swap(&mut box_sl, box_sl_prev);
                        },
                        _ => ()
                    }
                    s
                },
                _ => Sequence::new()
            };
        }

        self.sequences = *buffer;
        self.number_sequences -= 1;

        return Some(res)
    }

    /// HashMap of the type and number of each card on the table
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::table::*;
    /// use machiavelli::sequence_cards::*;
    ///
    /// let mut table = Table::new();
    /// table.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 4), 
    ///     RegularCard(Club, 5), 
    ///     RegularCard(Club, 6), 
    /// ]));
    /// table.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 2), 
    ///     RegularCard(Club, 3), 
    ///     RegularCard(Club, 4), 
    /// ]));
    /// 
    /// let hm_cards = table.count_cards();
    ///
    /// assert_eq!(1, hm_cards[&RegularCard(Club, 5)]);
    /// assert_eq!(2, hm_cards[&RegularCard(Club, 4)]);
    /// assert_eq!(false, hm_cards.contains_key(&RegularCard(Club, 7)));
    /// ```
    pub fn count_cards(&self) -> HashMap<Card, u16> {

        let mut res = HashMap::<Card, u16>::new();

        let mut current_sequence = &self.sequences;
        while *current_sequence != Nil {
            match current_sequence {
                Cons(seq, box_sl) => {
                    for card in seq.to_vec() {
                        if res.contains_key(&card) {
                            *res.get_mut(&card).unwrap() += 1;
                        } else {
                            res.insert(card, 1);
                        }
                    }
                    current_sequence = &*box_sl;
                },
                _ => ()
            }
        }
        
        return res;
    }
    
    /// Determine whether a table contains all the cards in a hashmap
    ///
    /// # Example
    ///
    /// ```
    /// use machiavelli::table::*;
    /// use machiavelli::sequence_cards::*;
    ///
    /// let mut table_1 = Table::new();
    /// table_1.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 4), 
    ///     RegularCard(Club, 5), 
    ///     RegularCard(Club, 6), 
    /// ]));
    /// table_1.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 2), 
    ///     RegularCard(Club, 3), 
    ///     RegularCard(Club, 4), 
    /// ]));
    ///
    /// let mut table_2 = Table::new();
    /// table_2.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 4), 
    ///     RegularCard(Club, 5), 
    ///     RegularCard(Club, 6), 
    /// ]));
    /// table_2.add(Sequence::from_cards(&[
    ///     RegularCard(Club, 2), 
    ///     RegularCard(Club, 3), 
    ///     RegularCard(Club, 4), 
    ///     RegularCard(Club, 5), 
    /// ]));
    /// 
    /// let hm_cards_1 = table_1.count_cards();
    /// let hm_cards_2 = table_2.count_cards();
    ///
    /// assert_eq!(true, table_2.contains_hm(&hm_cards_1));
    /// assert_eq!(false, table_1.contains_hm(&hm_cards_2));
    /// ```
    pub fn contains_hm(&self, card_count: &HashMap<Card, u16>) -> bool {
        
        let card_count_self = self.count_cards();

        for (card, count) in card_count {
            if !card_count_self.contains_key(&card) {
                return false;
            }
            if card_count_self[&card] < *count {
                return false;
            }
        }
         
        true
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i_seq = 1;
        let mut sl = &self.sequences;
        while let Cons(seq, new_sl) = &*sl {
            write!(f, "{}: {}\n", i_seq, seq)?;
            i_seq += 1;
            sl = new_sl;
        }
        write!(f, "")
    }
}

#[derive(PartialEq)]
enum SequenceList {
    Cons(Sequence, Box<SequenceList>),
    Nil
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn display_table_1() {
        let seq_1 = Sequence::from_cards(&[
            RegularCard(Club, 2), 
            Joker,
            RegularCard(Diamond, 3), 
            RegularCard(Heart, 2), 
        ]);
        let seq_2 = Sequence::from_cards(&[
            RegularCard(Club, 4), 
            RegularCard(Diamond, 5), 
            RegularCard(Heart, 6), 
        ]);

        let table = Table {
            number_sequences: 2,
            sequences: Cons(seq_1, Box::new(Cons(seq_2, Box::new(Nil))))
        };

        assert_eq!("1: 2♣ ★ 3♦ 2♥ \n2: 4♣ 5♦ 6♥ \n".to_string(), format!("{}", &table));
    }

}

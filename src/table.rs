use std::fmt;
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
    /// assert_eq!("1: ♥J ♥Q ♥K \n2: ♣4 ♣5 ♣6 \n".to_string(), format!("{}", &table));
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
    /// assert_eq!("1: ♥J ♥Q ♥K \n2: ♠7 ♥7 ♦7 \n".to_string(), format!("{}", &table));
    ///
    /// seq = table.take(1).unwrap();
    ///
    /// assert_eq!(seq, Sequence::from_cards(&[
    ///     RegularCard(Heart, 11), 
    ///     RegularCard(Heart, 12), 
    ///     RegularCard(Heart, 13), 
    /// ]));
    /// assert_eq!("1: ♠7 ♥7 ♦7 \n".to_string(), format!("{}", &table));
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

        assert_eq!("1: ♣2  ★ ♦3 ♥2 \n2: ♣4 ♦5 ♥6 \n".to_string(), format!("{}", &table));
    }

}

use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::gamemaster::Guess;
pub enum Strategy {
    Random,
    VowelPrune,
}
pub struct Player {
    pub options: Vec<[char; 5]>,
    pub strategy: Strategy,
}

impl Player {
    pub fn new(data_set: Vec<[char; 5]>, strategy: Strategy) -> Self {
        Self {
            options: data_set,
            strategy,
        }
    }

    pub fn guess(&self) -> [char; 5] {
        match self.strategy {
            Strategy::Random => *self
                .options
                .choose(&mut rand::thread_rng())
                .expect("No possible anwers?"),
            Strategy::VowelPrune => {
                if self.options.contains(&['a', 'u', 'l', 'o', 'i']) {
                    return ['a', 'u', 'l', 'o', 'i'];
                };
                *self
                    .options
                    .choose(&mut rand::thread_rng())
                    .expect("No possible anwers?")
            }
        }
    }

    pub fn prune(&mut self, guesses: [Guess; 5]) {
        // These character should be in this posistion
        let mut correct_chars: Vec<(char, usize)> = Vec::new();

        // excluded characters do not only restrict their own posistion
        // but also other no correct ones. Else they would be "wrongprosition"
        // thats why we need the mask
        let mut excluded_chars: Vec<char> = Vec::new();
        let mut exclude_mask: Vec<usize> = Vec::new();

        // Correct and Wrong positiong count minimal amount of characters of the
        // same type in the answer
        let mut counts: HashMap<char, usize> = HashMap::with_capacity(5);

        for (i, guess) in guesses.iter().enumerate() {
            match guess {
                Guess::Correct(c) => {
                    correct_chars.push((*c, i));
                    let char_count = counts.get(c).unwrap_or(&0);
                    counts.insert(*c, char_count + 1);
                }
                Guess::WrongPosition(c) => {
                    exclude_mask.push(i);
                    let char_count = counts.get(c).unwrap_or(&0);
                    counts.insert(*c, char_count + 1);
                }
                Guess::Wrong(c) => {
                    excluded_chars.push(*c);
                    exclude_mask.push(i)
                }
            }
        }

        self.options = self
            .options
            .iter()
            .filter(|x| {
                for (c, i) in correct_chars.iter() {
                    if *c != x[*i] {
                        return false;
                    }
                }
                for c in excluded_chars.iter() {
                    for i in exclude_mask.iter() {
                        if *c == x[*i] {
                            return false;
                        }
                    }
                }
                for (c, count) in counts.iter() {
                    let x_count = x.iter().filter(|xc| *xc == c).count();
                    if x_count < *count {
                        return false;
                    }
                }
                true
            })
            .copied()
            .collect()
    }
}

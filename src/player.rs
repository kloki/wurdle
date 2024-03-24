use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::gamemaster::Guess;

const ASCII: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub enum Strategy {
    Random,
    VowelPrune,
    Deterministic,
    SplitStrategy,
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
            Strategy::Deterministic => self.options[0],
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
            Strategy::SplitStrategy => self.split_strategy(),
        }
    }

    fn split_strategy(&self) -> [char; 5] {
        if self.options.len() < 4 {
            return *self
                .options
                .choose(&mut rand::thread_rng())
                .expect("No possible anwers?");
        }

        let mut results: Vec<(char, isize)> = ASCII
            .iter()
            .map(|c| {
                let mut count: isize = 0;
                for o in &self.options {
                    if o.contains(c) {
                        count += 1;
                    }
                }
                let decisive_factor = (count - ((self.options.len() / 2) as isize)).abs();

                (*c, decisive_factor)
            })
            .collect();

        results.sort_unstable_by_key(|x| x.1);
        let most_decisive_char = results[0].0;
        let pruned_options: Vec<_> = self
            .options
            .iter()
            .filter(|x| x.contains(&most_decisive_char))
            .collect();
        dbg!(most_decisive_char);
        **pruned_options
            .choose(&mut rand::thread_rng())
            .expect("No possible anwers?")
    }

    pub fn prune(&mut self, guesses: [Guess; 5]) {
        // These character should be in this posistion or not
        let mut correct_chars: Vec<(char, usize)> = Vec::new();
        let mut excluded_chars: Vec<(char, usize)> = Vec::new();

        // Wrong guesses do not only restrict their own posistion
        // else they would be "wrongprosition"
        let mut excluded_chars_infered: Vec<char> = Vec::new();
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
                    excluded_chars.push((*c, i));
                    exclude_mask.push(i);
                    let char_count = counts.get(c).unwrap_or(&0);
                    counts.insert(*c, char_count + 1);
                }
                Guess::Wrong(c) => {
                    excluded_chars.push((*c, i));
                    excluded_chars_infered.push(*c);
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

                for (c, i) in excluded_chars.iter() {
                    if *c == x[*i] {
                        return false;
                    }
                }

                for c in excluded_chars_infered.iter() {
                    if *counts.get(c).unwrap_or(&0) == 0 {
                        for i in exclude_mask.iter() {
                            if *c == x[*i] {
                                return false;
                            }
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
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_non_correct() {
        let mut player = Player::new(
            vec![['a', 'a', 'a', 'a', 'b'], ['b', 'a', 'a', 'a', 'b']],
            Strategy::Deterministic,
        );
        player.prune([
            Guess::Correct('a'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
        ]);
        assert_eq!(player.options.len(), 1)
    }
    #[test]
    fn test_remove_non_excluded() {
        let mut player = Player::new(
            vec![['a', 'a', 'x', 'a', 'b'], ['a', 'a', 'a', 'a', 'b']],
            Strategy::Deterministic,
        );
        player.prune([
            Guess::Correct('a'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
        ]);
        assert_eq!(player.options.len(), 1)
    }
    #[test]
    fn test_remove_exclude() {
        let mut player = Player::new(
            vec![['a', 'a', 'x', 'a', 'b'], ['a', 'b', 'b', 'a', 'b']],
            Strategy::Deterministic,
        );
        player.prune([
            Guess::Correct('a'),
            Guess::WrongPosition('a'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
        ]);
        assert_eq!(player.options.len(), 1)
    }
    #[test]
    fn test_wrong_posistion_conflict() {
        let mut player = Player::new(
            vec![
                ['b', 'a', 'n', 'n', 'n'],
                ['b', 'n', 'n', 'a', 'n'],
                ['b', 'n', 'a', 'n', 'n'],
                ['b', 'n', 'a', 'n', 'x'],
            ],
            Strategy::Deterministic,
        );
        player.prune([
            Guess::Correct('b'),
            Guess::WrongPosition('a'),
            Guess::Wrong('a'),
            Guess::Wrong('x'),
            Guess::Wrong('x'),
        ]);
        assert_eq!(player.options.len(), 1)
    }
}

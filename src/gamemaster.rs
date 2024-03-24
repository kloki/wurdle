use rand::seq::SliceRandom;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Guess {
    Correct(char),
    WrongPosition(char),
    Wrong(char),
}

pub struct GameMaster {
    pub solution: [char; 5],
}

impl GameMaster {
    pub fn new(word_set: &Vec<[char; 5]>) -> Self {
        Self {
            solution: *word_set
                .choose(&mut rand::thread_rng())
                .expect("word set is empty"),
        }
    }

    pub fn guess(&self, guess: &[char; 5]) -> [Guess; 5] {
        let mut answer = [Guess::Wrong('a'); 5];
        let mut used_for_wrong_pos = [false; 5];
        'outer: for i in 0..5 {
            if self.solution[i] == guess[i] {
                answer[i] = Guess::Correct(guess[i]);
            } else {
                for ii in 0..5 {
                    if !used_for_wrong_pos[ii]
                        && guess[ii] != self.solution[ii]
                        && guess[i] == self.solution[ii]
                    {
                        answer[i] = Guess::WrongPosition(guess[i]);
                        used_for_wrong_pos[ii] = true;
                        continue 'outer;
                    }
                }
                answer[i] = Guess::Wrong(guess[i]);
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guessing_simple() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);

        assert_eq!(
            gm.guess(&['m', 'o', 'd', 'e', 'm',]),
            [
                Guess::Correct('m'),
                Guess::Wrong('o'),
                Guess::Wrong('d'),
                Guess::Wrong('e'),
                Guess::Wrong('m'),
            ]
        );
    }

    #[test]
    fn test_guessing_simple_wrong_position() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['q', 'o', 'd', 'e', 'm',]),
            [
                Guess::Wrong('q'),
                Guess::Wrong('o'),
                Guess::Wrong('d'),
                Guess::Wrong('e'),
                Guess::WrongPosition('m'),
            ]
        )
    }

    #[test]
    fn test_guessing_simple_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['q', 'o', 'd', 'm', 'm',]),
            [
                Guess::Wrong('q'),
                Guess::Wrong('o'),
                Guess::Wrong('d'),
                Guess::WrongPosition('m'),
                Guess::Wrong('m'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['a', 'x', 'a', 'x', 's',]),
            [
                Guess::WrongPosition('a'),
                Guess::Wrong('x'),
                Guess::WrongPosition('a'),
                Guess::Wrong('x'),
                Guess::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_some_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['m', 'a', 'a', 'x', 's',]),
            [
                Guess::Correct('m'),
                Guess::Correct('a'),
                Guess::WrongPosition('a'),
                Guess::Wrong('x'),
                Guess::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_switched() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['m', 'a', 'n', 's', 'a',]),
            [
                Guess::Correct('m'),
                Guess::Correct('a'),
                Guess::Correct('n'),
                Guess::WrongPosition('s'),
                Guess::WrongPosition('a'),
            ]
        )
    }
}

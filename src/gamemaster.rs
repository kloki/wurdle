use rand::seq::SliceRandom;

use crate::Word;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FeedbackType {
    Correct(char),
    WrongPosition(char),
    Wrong(char),
}

pub type Feedback = [FeedbackType; 5];

pub struct GameMaster {
    pub solution: Word,
}

impl GameMaster {
    pub fn new(word_set: &Vec<Word>) -> Self {
        Self {
            solution: *word_set
                .choose(&mut rand::thread_rng())
                .expect("word set is empty"),
        }
    }

    pub fn with_solution(solution: Word) -> Self {
        Self { solution }
    }

    pub fn guess(&self, guess: &Word) -> Feedback {
        let mut answer = [FeedbackType::Wrong('a'); 5];
        let mut used_for_wrong_pos = [false; 5];
        'outer: for i in 0..5 {
            if self.solution[i] == guess[i] {
                answer[i] = FeedbackType::Correct(guess[i]);
            } else {
                for ii in 0..5 {
                    if !used_for_wrong_pos[ii]
                        && guess[ii] != self.solution[ii]
                        && guess[i] == self.solution[ii]
                    {
                        answer[i] = FeedbackType::WrongPosition(guess[i]);
                        used_for_wrong_pos[ii] = true;
                        continue 'outer;
                    }
                }
                answer[i] = FeedbackType::Wrong(guess[i]);
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
                FeedbackType::Correct('m'),
                FeedbackType::Wrong('o'),
                FeedbackType::Wrong('d'),
                FeedbackType::Wrong('e'),
                FeedbackType::Wrong('m'),
            ]
        );
    }

    #[test]
    fn test_guessing_simple_wrong_position() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['q', 'o', 'd', 'e', 'm',]),
            [
                FeedbackType::Wrong('q'),
                FeedbackType::Wrong('o'),
                FeedbackType::Wrong('d'),
                FeedbackType::Wrong('e'),
                FeedbackType::WrongPosition('m'),
            ]
        )
    }

    #[test]
    fn test_guessing_simple_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['q', 'o', 'd', 'm', 'm',]),
            [
                FeedbackType::Wrong('q'),
                FeedbackType::Wrong('o'),
                FeedbackType::Wrong('d'),
                FeedbackType::WrongPosition('m'),
                FeedbackType::Wrong('m'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['a', 'x', 'a', 'x', 's',]),
            [
                FeedbackType::WrongPosition('a'),
                FeedbackType::Wrong('x'),
                FeedbackType::WrongPosition('a'),
                FeedbackType::Wrong('x'),
                FeedbackType::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_some_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['m', 'a', 'a', 'x', 's',]),
            [
                FeedbackType::Correct('m'),
                FeedbackType::Correct('a'),
                FeedbackType::WrongPosition('a'),
                FeedbackType::Wrong('x'),
                FeedbackType::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_switched() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['m', 'a', 'n', 's', 'a',]),
            [
                FeedbackType::Correct('m'),
                FeedbackType::Correct('a'),
                FeedbackType::Correct('n'),
                FeedbackType::WrongPosition('s'),
                FeedbackType::WrongPosition('a'),
            ]
        )
    }
}

use rand::seq::SliceRandom;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Feedback {
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

    pub fn with_solution(solution: [char; 5]) -> Self {
        Self { solution }
    }

    pub fn guess(&self, guess: &[char; 5]) -> [Feedback; 5] {
        let mut answer = [Feedback::Wrong('a'); 5];
        let mut used_for_wrong_pos = [false; 5];
        'outer: for i in 0..5 {
            if self.solution[i] == guess[i] {
                answer[i] = Feedback::Correct(guess[i]);
            } else {
                for ii in 0..5 {
                    if !used_for_wrong_pos[ii]
                        && guess[ii] != self.solution[ii]
                        && guess[i] == self.solution[ii]
                    {
                        answer[i] = Feedback::WrongPosition(guess[i]);
                        used_for_wrong_pos[ii] = true;
                        continue 'outer;
                    }
                }
                answer[i] = Feedback::Wrong(guess[i]);
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
                Feedback::Correct('m'),
                Feedback::Wrong('o'),
                Feedback::Wrong('d'),
                Feedback::Wrong('e'),
                Feedback::Wrong('m'),
            ]
        );
    }

    #[test]
    fn test_guessing_simple_wrong_position() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['q', 'o', 'd', 'e', 'm',]),
            [
                Feedback::Wrong('q'),
                Feedback::Wrong('o'),
                Feedback::Wrong('d'),
                Feedback::Wrong('e'),
                Feedback::WrongPosition('m'),
            ]
        )
    }

    #[test]
    fn test_guessing_simple_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['q', 'o', 'd', 'm', 'm',]),
            [
                Feedback::Wrong('q'),
                Feedback::Wrong('o'),
                Feedback::Wrong('d'),
                Feedback::WrongPosition('m'),
                Feedback::Wrong('m'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['a', 'x', 'a', 'x', 's',]),
            [
                Feedback::WrongPosition('a'),
                Feedback::Wrong('x'),
                Feedback::WrongPosition('a'),
                Feedback::Wrong('x'),
                Feedback::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_duplicate_some_wrong_position_duplicate() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['m', 'a', 'a', 'x', 's',]),
            [
                Feedback::Correct('m'),
                Feedback::Correct('a'),
                Feedback::WrongPosition('a'),
                Feedback::Wrong('x'),
                Feedback::Correct('s'),
            ]
        )
    }
    #[test]
    fn test_guessing_switched() {
        let gm = GameMaster::new(&vec![['m', 'a', 'n', 'a', 's']]);
        assert_eq!(
            gm.guess(&['m', 'a', 'n', 's', 'a',]),
            [
                Feedback::Correct('m'),
                Feedback::Correct('a'),
                Feedback::Correct('n'),
                Feedback::WrongPosition('s'),
                Feedback::WrongPosition('a'),
            ]
        )
    }
}

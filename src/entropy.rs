use std::collections::HashMap;

use rayon::prelude::*;

use crate::{
    gamemaster::{FeedbackType, GameMaster},
    Word,
};

#[derive(Hash, Eq, PartialEq)]
pub enum FB {
    C,
    WP,
    W,
}

impl From<FeedbackType> for FB {
    fn from(value: FeedbackType) -> Self {
        match value {
            FeedbackType::Correct(_) => FB::C,
            FeedbackType::WrongPosition(_) => FB::WP,
            FeedbackType::Wrong(_) => FB::W,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct FeedbackTypeMask([FB; 5]);

impl From<[FeedbackType; 5]> for FeedbackTypeMask {
    fn from(value: [FeedbackType; 5]) -> Self {
        FeedbackTypeMask([
            value[0].into(),
            value[1].into(),
            value[2].into(),
            value[3].into(),
            value[4].into(),
        ])
    }
}

pub fn best_guess(options: &Vec<Word>, valid: &Vec<Word>) -> Word {
    let results = find_entropies(options, valid);
    results.last().expect("results should not be empty").0
}

pub fn find_entropies(options: &Vec<Word>, valid: &Vec<Word>) -> Vec<(Word, f64)> {
    //Although we consider putting in words we are now ar wrong.
    //If they have a higher entropy that are worth considering
    let mut results: Vec<(Word, f64)> = valid
        .par_iter()
        .map(|x| (*x, find_entropy(*x, options)))
        .collect();
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Dont worry about Nan"));
    results
}

pub fn find_entropy(word: Word, options: &Vec<Word>) -> f64 {
    let mut feedback_count: HashMap<FeedbackTypeMask, usize> = HashMap::new();
    let size: f64 = options.len() as f64;
    for o in options {
        let gm = GameMaster::with_solution(*o);
        let fb: FeedbackTypeMask = gm.guess(&word).into();
        let count = feedback_count.get(&fb).unwrap_or(&0);
        feedback_count.insert(fb, count + 1);
    }
    feedback_count
        .values()
        .map(|x| {
            let p: f64 = *x as f64 / size;
            p * (1. / p).log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanity_check_1() {
        let result = find_entropies(
            &vec![['a', 'a', 'a', 'a', 'a'], ['a', 'a', 'b', 'a', 'a']],
            &vec![['a', 'a', 'a', 'a', 'a'], ['a', 'a', 'b', 'a', 'a']],
        );
        assert_eq!(result[0].1, 1.0);
        assert_eq!(result[1].1, 1.0);
    }

    #[test]
    fn test_sanity_check_2() {
        let result = find_entropies(
            &vec![
                ['a', 'a', 'a', 'a', 'a'],
                ['a', 'a', 'b', 'a', 'a'],
                ['z', 'z', 'z', 'z', 'z'],
            ],
            &vec![
                ['a', 'a', 'a', 'a', 'a'],
                ['a', 'a', 'b', 'a', 'a'],
                ['z', 'z', 'z', 'z', 'z'],
            ],
        );
        dbg![&result];
        assert!(result[0].1 < result[1].1);
        assert!(result[0].1 < result[2].1);
        assert_eq!(result[1].1, result[2].1);
    }
}

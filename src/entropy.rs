use std::collections::HashMap;

use rayon::prelude::*;

use crate::gamemaster::{Feedback, GameMaster};

#[derive(Hash, Eq, PartialEq)]
pub enum FB {
    C,
    WP,
    W,
}

impl From<Feedback> for FB {
    fn from(value: Feedback) -> Self {
        match value {
            Feedback::Correct(_) => FB::C,
            Feedback::WrongPosition(_) => FB::WP,
            Feedback::Wrong(_) => FB::W,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct FeedbackMask([FB; 5]);

impl From<[Feedback; 5]> for FeedbackMask {
    fn from(value: [Feedback; 5]) -> Self {
        FeedbackMask([
            value[0].into(),
            value[1].into(),
            value[2].into(),
            value[3].into(),
            value[4].into(),
        ])
    }
}

pub fn best_guess(options: &Vec<[char; 5]>) -> [char; 5] {
    if options.len() > 10000 {
        return ['t', 'a', 'r', 'e', 's'];
    }
    let results = find_entropies(options);
    results.last().expect("results should not be empty").0
}

pub fn find_entropies(options: &Vec<[char; 5]>) -> Vec<([char; 5], f64)> {
    let mut results: Vec<([char; 5], f64)> = options
        .par_iter()
        .map(|x| (*x, find_entropy(*x, &options)))
        .collect();
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).expect("Dont worry about Nan"));
    results
}

pub fn find_entropy(word: [char; 5], options: &Vec<[char; 5]>) -> f64 {
    let mut feedback_count: HashMap<FeedbackMask, usize> = HashMap::new();
    let size: f64 = options.len() as f64;
    for o in options {
        let gm = GameMaster::with_solution(*o);
        let fb: FeedbackMask = gm.guess(&word).into();
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

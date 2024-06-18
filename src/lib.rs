use crate::{
    gamemaster::{Feedback, GameMaster},
    player::{Player, Strategy},
    utils::validate_answer,
};
pub mod entropy;
pub mod gamemaster;
pub mod importer;
pub mod player;
pub mod utils;

#[derive(Debug)]
pub struct Game {
    guesses: Vec<Feedback>,
    won: bool,
}

pub type Word = [char; 5];

impl Game {
    pub fn won(&self) -> bool {
        self.won
    }
    pub fn number_of_guesses(&self) -> usize {
        self.guesses.len()
    }
}

pub fn run(data: Vec<Word>, solution: Word, strategy: Strategy, guesses: usize) -> Game {
    let gm = GameMaster::with_solution(solution);
    let mut p = Player::new(data, strategy);
    let mut results: Vec<Feedback> = vec![];
    for i in 0..guesses {
        let result = gm.guess(&p.guess(i == guesses - 1));
        results.push(result);
        if validate_answer(&result) {
            return Game {
                guesses: results,
                won: true,
            };
        }

        p.prune(result)
    }
    Game {
        guesses: results,
        won: false,
    }
}

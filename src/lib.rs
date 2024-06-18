use crate::{
    gamemaster::GameMaster,
    player::{Player, Strategy},
    utils::validate_answer,
};
pub mod entropy;
pub mod gamemaster;
pub mod importer;
pub mod player;
pub mod utils;

pub enum Score {
    Success(usize),
    Fail,
}

impl Score {
    pub fn won(&self) -> bool {
        match self {
            Score::Success(_) => true,
            Score::Fail => false,
        }
    }
}

pub fn run(data: Vec<[char; 5]>, solution: [char; 5], strategy: Strategy, guesses: usize) -> Score {
    let gm = GameMaster::with_solution(solution);
    let mut p = Player::new(data, strategy);
    for i in 0..guesses {
        let result = gm.guess(&p.guess(i == guesses - 1));
        if validate_answer(&result) {
            return Score::Success(i + 1);
        }

        p.prune(result)
    }
    Score::Fail
}

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

pub fn run(data: Vec<[char; 5]>, solution: [char; 5], strategy: Strategy) -> usize {
    let gm = GameMaster::with_solution(solution);
    let mut p = Player::new(data, strategy);
    let mut guesses = 0;
    loop {
        guesses += 1;
        let result = gm.guess(&p.guess());
        if validate_answer(&result) {
            break;
        }

        p.prune(result)
    }
    guesses
}

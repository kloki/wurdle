use colored::Colorize;

use crate::gamemaster::Guess;
pub fn validate_answer(result: &[Guess; 5]) -> bool {
    result.iter().all(|x| matches!(x, Guess::Correct(_)))
}

pub fn print_answer(result: &[Guess; 5]) {
    for i in result.iter() {
        match i {
            Guess::Correct(c) => print!("{}", c.to_string().green()),
            Guess::WrongPosition(c) => print!("{}", c.to_string().yellow()),
            Guess::Wrong(c) => print!("{}", c),
        }
    }
    println!(" ");
}

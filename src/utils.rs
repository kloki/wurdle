use colored::Colorize;

use crate::gamemaster::Feedback;
pub fn validate_answer(result: &[Feedback; 5]) -> bool {
    result.iter().all(|x| matches!(x, Feedback::Correct(_)))
}

pub fn print_answer(result: &[Feedback; 5]) {
    for i in result.iter() {
        match i {
            Feedback::Correct(c) => print!("{}", c.to_string().green()),
            Feedback::WrongPosition(c) => print!("{}", c.to_string().yellow()),
            Feedback::Wrong(c) => print!("{}", c),
        }
    }
    println!(" ");
}

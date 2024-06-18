use colored::Colorize;

use crate::gamemaster::FeedbackType;
pub fn validate_answer(result: &[FeedbackType; 5]) -> bool {
    result.iter().all(|x| matches!(x, FeedbackType::Correct(_)))
}

pub fn print_answer(result: &[FeedbackType; 5]) {
    for i in result.iter() {
        match i {
            FeedbackType::Correct(c) => print!("{}", c.to_string().green()),
            FeedbackType::WrongPosition(c) => print!("{}", c.to_string().yellow()),
            FeedbackType::Wrong(c) => print!("{}", c),
        }
    }
    println!(" ");
}

pub fn print_emoji(result: &[FeedbackType; 5]) {
    for i in result.iter() {
        match i {
            FeedbackType::Correct(_) => print!("🟩",),
            FeedbackType::WrongPosition(_) => print!("🟨"),
            FeedbackType::Wrong(_) => print!("⬜"),
        }
    }
    println!(" ");
}

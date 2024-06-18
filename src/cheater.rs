use colored::Colorize;
use wurdle::{
    gamemaster::{Feedback, FeedbackType},
    importer::import_file,
    player::Player,
    utils::print_emoji,
    Word,
};

pub fn get_use_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_feedback(guess: Word) -> Feedback {
    println!("Enter feedback('_/y/g'): ");
    let raw = get_use_input().chars().collect::<Vec<char>>();
    if raw.len() != 5 {
        println!("FeedbackType must be 5 characters long");
        return get_feedback(guess);
    }
    guess
        .iter()
        .enumerate()
        .map(|(i, c)| match raw[i] {
            'g' => FeedbackType::Correct(*c),
            'y' => FeedbackType::WrongPosition(*c),
            _ => FeedbackType::Wrong(*c),
        })
        .collect::<Vec<FeedbackType>>()
        .try_into()
        .expect("invalid feedback")
}

fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");
    let mut p = Player::new(
        data,
        wurdle::player::Strategy::CachedEntropy(['t', 'a', 'r', 'e', 's']),
    );

    println!("Wordle Helper\n");
    for i in 0..6 {
        let guess = p.guess(i == 5);
        if p.options.len() == 1 {
            println!(
                "Word found: {}",
                p.options[0].iter().collect::<String>().green()
            );
            break;
        } else if i == 5 {
            println!("Last guess: {}", guess.iter().collect::<String>().green());
        } else {
            println!("Suggestion: {}\n", guess.iter().collect::<String>().green());
        }

        if i != 5 {
            println!("\n");
            let feedback = get_feedback(guess);
            print_emoji(&feedback);
            p.prune(feedback);
        }
    }
}

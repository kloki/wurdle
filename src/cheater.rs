use colored::Colorize;
use wurdle::{gamemaster::Feedback, importer::import_file, player::Player, utils::print_emoji};

pub fn get_use_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_feedback(guess: [char; 5]) -> [Feedback; 5] {
    println!("Enter feedback('_/y/g'): ");
    let raw = get_use_input().chars().collect::<Vec<char>>();
    if raw.len() != 5 {
        println!("Feedback must be 5 characters long");
        return get_feedback(guess);
    }
    guess
        .iter()
        .enumerate()
        .map(|(i, c)| match raw[i] {
            'g' => Feedback::Correct(*c),
            'y' => Feedback::WrongPosition(*c),
            _ => Feedback::Wrong(*c),
        })
        .collect::<Vec<Feedback>>()
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
    loop {
        let guess = p.guess(false);
        println!("suggestion: {}\n", guess.iter().collect::<String>().green());
        let feedback = get_feedback(guess);
        println!("\n");
        print_emoji(&feedback);
        p.prune(feedback);
    }
}

use colored::Colorize;
use wurdle::{entropy::best_guess, importer::import_file};

fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");
    let opener = best_guess(&data, &data);

    println!(
        "Suggest opener for data set: {}",
        opener.iter().collect::<String>().green()
    );
}

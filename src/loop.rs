use colored::Colorize;
use rand::seq::SliceRandom;
use wurdle::{importer::import_file, player::Strategy, run, utils::print_emoji};
fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");
    let strategey = Strategy::CachedEntropy(['t', 'a', 'r', 'e', 's']);

    for _ in 0..1000 {
        let solution = data
            .choose(&mut rand::thread_rng())
            .expect("Cannot get solution");
        let game = run(data.clone(), *solution, strategey, 6);

        if game.won() {
            println!(
                "🎉 Found {} in {} guesses\n",
                solution.iter().collect::<String>().green(),
                game.number_of_guesses()
            );
        } else {
            println!(
                "👺 Couldn't find {} in {} guesses\n",
                solution.iter().collect::<String>().red(),
                game.number_of_guesses()
            );
        }
        for guess in &game.guesses {
            print_emoji(guess);
        }
        for _ in game.number_of_guesses()..7 {
            println!()
        }
    }
}

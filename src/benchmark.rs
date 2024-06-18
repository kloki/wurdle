use rand::seq::SliceRandom;
use rayon::prelude::*;
use wurdle::{importer::import_file, player::Strategy, run, Game};
const RUNS: usize = 100;

fn print_results(name: &str, results: Vec<Game>) {
    let win = results.iter().filter(|x| x.won()).count() as f64 / RUNS as f64;
    println!("{: >15}|{: >12}", name, win)
}

fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");

    println!("           Name|     Winrate");
    println!("----------------------------");
    let results: Vec<Game> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, Strategy::Random, 6)
        })
        .collect();
    print_results("Random", results);

    let results: Vec<Game> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, Strategy::VowelPrune, 6)
        })
        .collect();
    print_results("Vowel prune", results);

    let results: Vec<Game> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, Strategy::SplitStrategy, 6)
        })
        .collect();
    print_results("Split", results);

    let cached_entropy = Strategy::prepare_entropy(&data);
    let results: Vec<Game> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, cached_entropy, 6)
        })
        .collect();
    print_results("Entropy", results);
}

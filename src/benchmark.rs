use rand::seq::SliceRandom;
use rayon::prelude::*;
use wurdle::{importer::import_file, player::Strategy, run};
const RUNS: usize = 100;

fn print_results(name: &str, results: Vec<usize>) {
    let min = results.iter().min().unwrap();
    let max = results.iter().max().unwrap();
    let avg = results.iter().sum::<usize>() as f64 / RUNS as f64;
    let win = results.iter().filter(|x| *x < &7).count() as f64 / RUNS as f64;
    println!(
        "{: >15}|{: >8}|{: >8}|{: >8}|{: >8}",
        name, avg, max, min, win
    )
}

fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");

    println!("           Name|     Avg|     Max|     Min|   Winrate");
    println!("-----------------------------------------------------");
    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, Strategy::Random)
        })
        .collect();
    print_results("Random", results);

    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, Strategy::VowelPrune)
        })
        .collect();
    print_results("Vowel prune", results);

    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, Strategy::SplitStrategy)
        })
        .collect();
    print_results("Split", results);

    let cached_entropy = Strategy::prepare_entropy(&data);
    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| {
            let solution = data
                .choose(&mut rand::thread_rng())
                .expect("Cannot get solution");
            run(data.clone(), *solution, cached_entropy)
        })
        .collect();
    print_results("Entropy", results);
}

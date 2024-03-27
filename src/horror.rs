use rayon::prelude::*;
use wurdle::{importer::import_file, player::Strategy, run};
const RUNS: usize = 250;

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
    // For this one its probably impossible to get a meaningfull result:
    //  rg ites ./data/words.txt
    // 1122:bites
    // 2033:cites
    // 2891:dites
    // 4392:gites
    // 5864:kites
    // 6360:lites
    // 7000:mites
    // 7493:nites
    // 9228:rites
    // 10075:sites
    // 12563:wites
    // 12789:yites

    let data = import_file("./data/words.txt").expect("failed to import words");
    let solution = ['b', 'i', 't', 'e', 's'];

    println!("           Name|     Avg|     Max|     Min|   Winrate");
    println!("-----------------------------------------------------");
    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| run(data.clone(), solution, Strategy::Random))
        .collect();
    print_results("Random", results);

    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| run(data.clone(), solution, Strategy::VowelPrune))
        .collect();
    print_results("Vowel prune", results);

    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| run(data.clone(), solution, Strategy::SplitStrategy))
        .collect();
    print_results("Split", results);

    let cached_entropy = Strategy::prepare_entropy(&data);
    let results: Vec<usize> = (0..RUNS)
        .into_par_iter()
        .map(|_| run(data.clone(), solution, cached_entropy))
        .collect();
    print_results("Entropy", results);
}

use rayon::prelude::*;
use wurdle::{importer::import_file, player::Strategy, run};
#[test]
fn test_data_set() {
    // Run the solver on all possible inputs
    let data = import_file("./data/words.txt").expect("failed to import words");
    data.par_iter().for_each(|solution| {
        let steps = run(data.clone(), *solution, Strategy::Deterministic, 10);
        println!(
            "Solved {} with score {:?} ",
            solution.iter().collect::<String>(),
            steps
        )
    });
}

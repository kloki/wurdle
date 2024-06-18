use wurdle::{
    gamemaster::GameMaster,
    importer::import_file,
    player::Player,
    utils::{print_answer, validate_answer},
};
fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");
    let args: Vec<String> = std::env::args().collect();
    let solution: [char; 5] = args[1]
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("invalid word");
    let gm = GameMaster::with_solution(solution);
    let mut p = Player::new(
        data,
        wurdle::player::Strategy::CachedEntropy(['t', 'a', 'r', 'e', 's']),
    );
    let mut guesses = 0;

    println!("Starting");
    println!("Solution: {}\n", gm.solution.iter().collect::<String>());
    loop {
        guesses += 1;
        let result = gm.guess(&p.guess());
        print_answer(&result);
        if validate_answer(&result) {
            break;
        }

        p.prune(result)
    }
    if guesses > 6 {
        println!("\n😠 \nFound result in {} guesses", guesses);
    } else {
        println!("\n🎉 \nFound result in {} guesses", guesses);
    }
}

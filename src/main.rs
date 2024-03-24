use wurdle::{
    gamemaster::GameMaster,
    importer::import_file,
    player::Player,
    utils::{print_answer, validate_answer},
};
fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");
    let gm = GameMaster::new(&data);
    let mut p = Player::new(data, wurdle::player::Strategy::Deterministic);
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
    println!("\n🎉 \nFound result in {} guesses", guesses);
}

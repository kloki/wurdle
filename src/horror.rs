use wurdle::{
    gamemaster::GameMaster,
    importer::import_file,
    player::Player,
    utils::{print_answer, validate_answer},
};

fn main() {
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
    let gm = GameMaster::with_solution(['b', 'i', 't', 'e', 's']);
    let mut p = Player::new(
        data,
        wurdle::player::Strategy::CachedEntropy(['t', 'a', 'r', 'e', 's']),
    );

    println!("Starting");
    println!("Solution: {}\n", gm.solution.iter().collect::<String>());
    for i in 0..6 {
        let result = gm.guess(&p.guess(i == 5));
        print_answer(&result);
        if validate_answer(&result) {
            break;
        }

        p.prune(result)
    }
}

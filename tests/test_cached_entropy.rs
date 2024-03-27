use wurdle::{importer::import_file, player::Strategy};
#[test]
fn test_prepare_entropy() {
    // Run the solver on all possible inputs
    let data = import_file("./data/words.txt").expect("failed to import words");
    if let Strategy::CachedEntropy(start) = Strategy::prepare_entropy(&data) {
        assert_eq!(start, ['t', 'a', 'r', 'e', 's'])
    }
}

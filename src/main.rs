use wurdle::importer::import_file;
fn main() {
    let data = import_file("./data/words.txt").expect("failed to import words");
    println!("Loaded {} words!", data.len())
}

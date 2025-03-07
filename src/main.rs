use std::path::Path;
use trie::Trie;
use unscrambler::unscramble;

mod dictionary;
mod trie;
mod unscrambler;

fn main() {
    let mut trie = Trie::new();
    match dictionary::load_dictionary(&mut trie, Path::new("dict.txt")) {
        Ok(_) => {}
        Err(error) => panic!("Failed to load dictionary: {}", error),
    }

    let matches = unscramble(&trie, "testicular".to_owned());
    for word in matches {
        println!("{}", word);
    }
}

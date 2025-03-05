use std::path::Path;

use unscrambler::Unscrambler;

mod unscrambler;

fn main() {
    let mut unscrambler = Unscrambler::new();
    match unscrambler.load_dictionary(Path::new("dict.txt")) {
        Ok(_) => {}
        Err(error) => panic!("Failed to load dictionary: {}", error),
    }

    let found_words = unscrambler.unscramble(String::from("letters"));

    println!("{} words found", found_words.len());
    for word in found_words {
        println!("{}", word);
    }
}

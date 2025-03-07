use std::collections::{BTreeMap, BTreeSet, HashSet};

const MAX_WIDTH: usize = 80;

fn sort_words_by_length(words: &HashSet<String>) -> BTreeMap<usize, BTreeSet<String>> {
    let mut sorted = BTreeMap::new();

    for word in words {
        let length = word.len();
        sorted
            .entry(length)
            .or_insert_with(BTreeSet::new)
            .insert(word.clone());
    }

    sorted
}

pub fn print_words(words: &HashSet<String>) {
    println!();

    let sorted_words = sort_words_by_length(&words);

    let keys = sorted_words.keys();
    for key in keys.rev() {
        let words = match sorted_words.get(key) {
            Some(words) => words,
            None => continue,
        };

        let width = MAX_WIDTH / key;
        let mut count = 0;

        for word in words {
            if count > width {
                println!();
                count = 0;
            }
            print!("{} ", word);
            count += 1;
        }
        print!("\n\n");
    }
}

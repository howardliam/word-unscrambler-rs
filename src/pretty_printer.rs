use std::collections::{BTreeMap, BTreeSet, HashSet};

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
    let sorted_words = sort_words_by_length(&words);

    let keys = sorted_words.keys();
    for key in keys.rev() {
        let words = match sorted_words.get(key) {
            Some(words) => words,
            None => continue,
        };

        for word in words {
            print!("{} ", word);
        }
        print!("\n\n");
    }
}

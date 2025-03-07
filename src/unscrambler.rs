use std::collections::HashSet;

use crate::trie::{Trie, TrieNode};

pub fn unscramble(trie: &Trie, letters: String) -> HashSet<String> {
    let mut matches = HashSet::new();

    let current_node = &trie.root;
    recursive_unscramble(
        current_node,
        "".to_owned(),
        letters.chars().collect(),
        &mut matches,
    );

    matches
}

fn recursive_unscramble(
    current_node: &TrieNode,
    building: String,
    letters: Vec<char>,
    matches: &mut HashSet<String>,
) {
    if current_node.is_word {
        matches.insert(building.clone());
    }

    for (i, ch) in letters.iter().enumerate() {
        let next_node = match current_node.children.get(&ch) {
            Some(node) => node,
            None => continue,
        };

        let new_building = building.clone() + &ch.to_string();
        let mut new_letters = letters.clone();
        new_letters.remove(i);

        recursive_unscramble(next_node, new_building, new_letters, matches);
    }
}

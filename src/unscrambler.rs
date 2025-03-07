use std::collections::HashSet;

use crate::trie::{Trie, TrieNode};

pub fn unscramble(trie: &Trie, letters: String) -> Result<HashSet<String>, ()> {
    let mut matches = HashSet::new();

    let letters = letters.trim();

    if letters.len() > 10 {
        println!("10 max letters exceeded");
        return Err(());
    }

    let wildcard_count = letters.chars().filter(|&ch| ch == '?').count();
    if wildcard_count > 2 {
        println!("2 max wildcards exceeded");
        return Err(());
    }

    let current_node = &trie.root;
    recursive_unscramble(
        current_node,
        "".to_owned(),
        letters.chars().collect(),
        &mut matches,
    );

    Ok(matches)
}

fn recursive_unscramble(
    current_node: &TrieNode,
    building: String,
    letters: Vec<char>,
    matches: &mut HashSet<String>,
) {
    if let Some(pos) = letters.iter().position(|&ch| ch == '?') {
        for ch in "abcdefghijklmnopqrstuvwxyz".chars() {
            let mut new_letters = letters.clone();
            new_letters[pos] = ch;

            recursive_unscramble(current_node, building.clone(), new_letters, matches);
        }
    }

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

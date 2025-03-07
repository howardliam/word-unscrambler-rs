use std::{
    collections::{BTreeSet, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::{
    settings,
    trie::{Trie, TrieNode},
};

pub struct Unscrambler {
    trie: Trie,
    alphabet: Vec<char>,
}

impl Unscrambler {
    pub fn new() -> Self {
        Self {
            trie: Trie::new(),
            alphabet: settings::ALPHABET.chars().collect(),
        }
    }

    pub fn load_dictionary(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        let mut unique_letters = BTreeSet::new();

        while let Some(Ok(line)) = lines.next() {
            let words = line.split_whitespace();

            for word in words {
                for ch in word.chars() {
                    unique_letters.insert(ch);
                }

                match self.trie.insert(word.to_lowercase()) {
                    Ok(_) => {}
                    Err(error) => panic!("{}", error),
                }
            }
        }

        self.alphabet = unique_letters.iter().map(|&ch| ch).collect();

        Ok(())
    }

    pub fn unscramble(&self, letters: String) -> Option<HashSet<String>> {
        let mut matches = HashSet::new();

        let letters = letters.trim();

        if letters.len() > settings::MAX_LETTERS {
            println!("Letter limit of {} exceeded", settings::MAX_LETTERS);
            return None;
        }

        let wildcard_count = letters
            .chars()
            .filter(|&ch| ch == settings::BLANK_CHAR)
            .count();

        if wildcard_count > settings::MAX_BLANKS {
            println!("Blank limit of {} exceeded", settings::MAX_BLANKS);
            return None;
        }

        let current_node = &self.trie.root;
        self.recursive_unscramble(
            current_node,
            "".to_owned(),
            letters.chars().collect(),
            &mut matches,
        );

        Some(matches)
    }

    fn recursive_unscramble(
        &self,
        current_node: &TrieNode,
        building: String,
        letters: Vec<char>,
        matches: &mut HashSet<String>,
    ) {
        if let Some(pos) = letters.iter().position(|&ch| ch == settings::BLANK_CHAR) {
            for ch in &self.alphabet {
                let mut new_letters = letters.clone();
                new_letters[pos] = *ch;

                self.recursive_unscramble(current_node, building.clone(), new_letters, matches);
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

            self.recursive_unscramble(next_node, new_building, new_letters, matches);
        }
    }
}

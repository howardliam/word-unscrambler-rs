use std::collections::HashMap;

pub struct TrieNode {
    pub is_word: bool,
    pub children: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            is_word: false,
            children: HashMap::new(),
        }
    }
}

pub struct Trie {
    pub root: Box<TrieNode>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new()),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current = &mut self.root;

        for ch in word.chars() {
            if !current.children.contains_key(&ch) {
                current.children.insert(ch, Box::new(TrieNode::new()));
            }
            current = match current.children.get_mut(&ch) {
                Some(node) => node,
                None => panic!("Failed to get child"),
            }
        }

        current.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        for ch in word.chars() {
            if !current.children.contains_key(&ch) {
                return false;
            }
            current = match current.children.get(&ch) {
                Some(node) => node,
                None => panic!("Failed to get child"),
            }
        }

        current.is_word
    }
}

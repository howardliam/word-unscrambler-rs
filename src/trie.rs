use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeInsertError;

impl fmt::Display for TreeInsertError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to insert into trie")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeRetrievalError;

impl fmt::Display for TreeRetrievalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to retrieve from trie")
    }
}

pub struct TrieNode {
    pub is_word: bool,
    pub children: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    /// Creates a new [`TrieNode`].
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
    /// Creates a new [`Trie`].
    pub fn new() -> Self {
        Self {
            root: Box::new(TrieNode::new()),
        }
    }

    /// Inserts the given word into the trie.
    ///
    /// # Panics
    /// Panics if a child is failed to be acquired, this should never happen.
    pub fn insert(&mut self, word: String) -> Result<(), TreeInsertError> {
        let mut current = &mut self.root;

        for ch in word.chars() {
            if !current.children.contains_key(&ch) {
                current.children.insert(ch, Box::new(TrieNode::new()));
            }
            current = match current.children.get_mut(&ch) {
                Some(node) => node,
                None => return Err(TreeInsertError),
            }
        }

        current.is_word = true;

        Ok(())
    }

    /// Searches the trie to see if the given word exists.
    #[allow(dead_code)]
    pub fn search(&self, word: String) -> Result<bool, TreeRetrievalError> {
        let mut current = &self.root;
        for ch in word.chars() {
            if !current.children.contains_key(&ch) {
                return Ok(false);
            }
            current = match current.children.get(&ch) {
                Some(node) => node,
                None => return Err(TreeRetrievalError),
            }
        }

        Ok(current.is_word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        let result = trie.insert("foobar".to_owned());

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_search() {
        let trie = Trie::new();
        let result = trie.search("foobar".to_owned());

        assert_eq!(result, Ok(false));
    }
}

use crate::trie::Trie;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

pub fn load_dictionary(trie: &mut Trie, path: &Path) -> Result<(), Error> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let words = line.split_whitespace();

        for word in words {
            trie.insert(word.to_lowercase());
        }
    }

    Ok(())
}

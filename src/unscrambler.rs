use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

struct SearchData {
    pub building: String,
    pub letters: String,
}

pub struct Unscrambler {
    dictionary: HashSet<String>,
}

impl Unscrambler {
    /// Creates a new [`Unscrambler`].
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::new(),
        }
    }

    /// Loads the dictionary into the unscrambler.
    ///
    /// # Errors
    /// This function will return an error if the file does not exist.
    pub fn load_dictionary(&mut self, path: &Path) -> Result<(), Error> {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(Ok(line)) = lines.next() {
            let words = line.split(" ");

            words
                .map(|word| self.dictionary.insert(word.to_owned().to_lowercase()))
                .for_each(drop);
        }

        Ok(())
    }

    /// Generates a [`HashSet`] of words found.
    pub fn unscramble(&self, letters: String) -> HashSet<String> {
        let start_data = SearchData {
            building: String::new(),
            letters,
        };

        let mut search_queue = VecDeque::new();
        search_queue.push_back(start_data);

        let mut found_words = HashSet::new();

        while let Some(search_data) = search_queue.pop_front() {
            let SearchData { building, letters } = search_data;

            if self.dictionary.contains(&building) {
                found_words.insert(building.clone());
            }

            for i in 0..letters.len() {
                let ch = match letters.chars().nth(i) {
                    Some(character) => character,
                    None => continue,
                };

                let mut new_building = building.clone();
                new_building.push(ch);

                let mut new_letters = letters.clone();
                new_letters.remove(i);

                let new_data = SearchData {
                    building: new_building,
                    letters: new_letters,
                };

                search_queue.push_back(new_data);
            }
        }

        found_words
    }
}

use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

fn load_dictionary(path: &Path) -> Result<HashSet<String>, Error> {
    let mut dictionary = HashSet::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if line.is_err() {
            continue;
        }
        dictionary.insert(line.unwrap().to_lowercase());
    }

    Ok(dictionary)
}

struct SearchData {
    pub building: String,
    pub letters: String,
}

fn main() {
    let dictionary = match load_dictionary(Path::new("dict.txt")) {
        Ok(dictionary) => dictionary,
        Err(error) => panic!("Failed to load dictionary: {}", error),
    };

    let start_data = SearchData {
        building: String::new(),
        letters: String::from("testing"),
    };

    let mut search_queue = VecDeque::new();
    search_queue.push_back(start_data);

    let mut found_words = Vec::new();

    while let Some(search_data) = search_queue.pop_front() {
        let SearchData { building, letters } = search_data;

        if dictionary.contains(&building) {
            found_words.push(building.clone());
        }

        for i in 0..letters.len() {
            let ch = letters.chars().nth(i).unwrap();

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

    for word in found_words {
        println!("{}", word);
    }
}

use rustyline::{error::ReadlineError, DefaultEditor};
use std::path::Path;
use trie::Trie;
use unscrambler::unscramble;

mod dictionary;
mod pretty_printer;
mod trie;
mod unscrambler;

fn main() {
    let mut trie = Trie::new();
    match dictionary::load_dictionary(&mut trie, Path::new("dict.txt")) {
        Ok(_) => {}
        Err(error) => panic!("Failed to load dictionary: {}", error),
    }

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(error) => panic!("Failed to create editor: {}", error),
    };

    loop {
        let input = rl.readline(
            "Type some scrambled text ('?' for a wildcard, max 2) [Ctrl-C, Ctrl-D to exit]: ",
        );
        let line = match input {
            Ok(line) => line,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                println!("\nExiting...");
                break;
            }
            Err(error) => {
                println!("Error: {}", error);
                break;
            }
        };

        match rl.add_history_entry(line.as_str()) {
            Ok(_) => {}
            Err(error) => println!("Failed to add history entry: {}", error),
        }

        let words = unscramble(&trie, line);

        if words.is_empty() {
            println!("No results found \n");
        } else {
            pretty_printer::print_words(&words);
        }
    }
}

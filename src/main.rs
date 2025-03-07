use rustyline::{error::ReadlineError, DefaultEditor};
use std::path::Path;
use unscrambler::Unscrambler;

mod pretty_printer;
mod settings;
mod trie;
mod unscrambler;

fn main() {
    let mut unscrambler = Unscrambler::new();
    match unscrambler.load_dictionary(Path::new("dict.txt")) {
        Ok(_) => {}
        Err(error) => panic!("Failed to load dictionary: {}", error),
    }

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(error) => panic!("Failed to create editor: {}", error),
    };

    println!("Word unscrambler");
    println!(
        "Max letters: {}. Use ? for blank, max blanks: {}.\n",
        settings::MAX_LETTERS,
        settings::MAX_BLANKS
    );
    loop {
        let input = rl.readline("Type some scrambled text [Ctrl-C, Ctrl-D to exit]: ");
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

        let words = match unscrambler.unscramble(line) {
            Some(words) => words,
            None => continue,
        };

        match words.is_empty() {
            true => println!("No results found \n"),
            false => pretty_printer::print_words(&words),
        }
    }
}

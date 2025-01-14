use std::fs;
use rand::seq::SliceRandom;

fn main() {
    println!("Hangman in Rust!");
    let path = fs::read_to_string("./src/hangman.txt");
    match path {
        Ok(contents) => {
            let lines: Vec<&str> = contents.lines().collect();
            let mut rng = rand::thread_rng();
            let random_word = lines.choose(&mut rng);
            match random_word {
                Some(random_word) => {
                    println!("{random_word}");
                }
                None => panic!("failed to retrieve a random word. \
                    ensure the ./src/hangman.txt file is available")
            }
        }
        Err(e) => panic!("failed to read file: {e}"),
    }
}

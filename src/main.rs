use std::{fs, io};
use std::collections::HashSet;
use std::io::Write;
use rand::Rng;

fn main() {
    let path = fs::read_to_string("./src/hangman.txt");
    match path {
        Ok(contents) => {
            'game: loop {
                println!("Hangman in Rust!\n");
                let lines: Vec<&str> = contents.lines().collect();
                let mut rng = rand::thread_rng();

                let random_word = String::from(lines[rng.gen_range(0..lines.len())]);
                let mut guessed_word = String::new();
                let mut guessed_set: HashSet<char> = HashSet::new();
                let mut guesses = 7;

                for _ in random_word.chars() {
                    guessed_word.push('_');
                }

                while guesses > 0 && guessed_word != random_word {
                    println!("guess a letter for the word below! you have {guesses} guesses remaining!");
                    println!("{guessed_word}");
                    print!("Guessed Characters: ");
                    if guessed_set.len() == 0 {
                        print!("none")
                    } else {
                        for c in &guessed_set {
                            print!("{c} ");
                        }
                    }

                    println!();
                    io::stdout().flush().unwrap();

                    let mut guess = String::new();
                    match io::stdin().read_line(&mut guess) {
                        Ok(_) => {
                            let guessed_char = guess.trim().chars().next().unwrap();

                            if !guessed_set.insert(guessed_char) {
                                println!("that letter has already been guessed! try another letter!\n");
                                continue;
                            }

                            let mut temp = String::new();
                            let mut guessed = false;

                            for (i, c) in random_word.chars().enumerate() {
                                if c == guessed_char {
                                    temp.push(c);
                                    guessed = true;
                                } else {
                                    temp.push(guessed_word.chars().nth(i).unwrap());
                                }
                            }

                            if !guessed {
                                guesses -= 1;
                                println!("Incorrect guess! {guesses} guesses remain!\n");
                            } else {
                                println!();
                            }

                            guessed_word = temp;
                        }
                        Err(e) => eprintln!("Error reading line: {}", e),
                    }
                }

                if guesses == 0 {
                    println!("No guesses remain! The word was {random_word}");
                }
                else if guessed_word == random_word {
                    println!("You guessed the word {random_word}!");
                }
                println!("Thank you for playing hangman!");
                print!("Would you like to play again? (use y/n for yes or no):  ");
                io::stdout().flush().unwrap();

                let mut ans = String::new();
                match io::stdin().read_line(&mut ans) {
                    Ok(_) => {
                        ans = ans.trim().to_lowercase();
                        if ans == "y" {
                            continue 'game;
                        } else {
                            return;
                        }
                    }
                    Err(_) => panic!("Error reading input!"),
                }
            }
        }
        Err(e) => panic!("failed to read file: {e}"),
    }
}

extern crate rand;

use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::io::Write;
use std::{env, fs, io};

fn main() {
    // This code block is to find the path to the words.txt file.
    let root = env::current_dir().unwrap();
    let mut root_path = String::new();
    if let Some(x) = root.to_str() {
        root_path = String::from(x);
    }
    let words_path = root_path + "/words.txt";

    // Read and parse words and filter those which contain duplicate letters
    let text = fs::read_to_string(words_path).unwrap();
    let words: Vec<&str> = text
        .split('\n')
        .filter(|word| {
            let uniq: HashSet<char> = word.chars().collect();
            uniq.len() == 4
        })
        .collect();

    let mut rng = thread_rng();
    loop {
        let actual_word = words[rng.gen_range(0, words.len())];

        println!("------------COWS AND BULLS---------------");
        println!("{}", actual_word);
        loop {
            let mut guessed_word = String::new();

            // Asking input until a 4 letter word is entered
            loop {
                print!("Please type a four letter word: ");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut guessed_word)
                    .expect("Reading failed");
                guessed_word = guessed_word.trim().to_uppercase();
                if guessed_word.len() == 4 {
                    break;
                } else {
                    guessed_word = String::new();
                }
            }

            let matches = actual_word
                .chars()
                .zip(guessed_word.chars())
                .fold(0, |sum, (x, y)| sum + if x == y { 1 } else { 0 });

            let exists = guessed_word
                .chars()
                .map(|c| actual_word.contains(c) as u32)
                .sum::<u32>()
                - matches;

            if matches == 4 {
                println!("Yay! You guessed it!\n");
                break;
            } else {
                println!("Match: {}, Exist: {}\n", matches, exists);
            }
        }
    }
}

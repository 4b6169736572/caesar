use std::{env, process};

fn main() {
    let alphabet: &str = "abcdefghijklmnopqrstuvwxyz";
    let encrypted: Vec<String> = env::args().collect();
    let mut shifted_words: Vec<Vec<String>> = Vec::new();

    for word in &encrypted[1..] {
        let positions = get_character_indexes(alphabet, word.to_string().to_lowercase());

        shifted_words.push(shift(alphabet, positions));
    }

    for i in 0..26 {
        for word in &shifted_words {
            print!("{} ", word[i]);
        }
        println!("");
    }
}

fn get_character_indexes(alphabet: &str, encrypted: String) -> Vec<usize> {
    let mut positions: Vec<usize> = Vec::new();

    for car in encrypted.as_bytes() {
        positions.push(
            alphabet
                .as_bytes()
                .iter()
                .position(|carr| *carr == *car)
                .unwrap_or_else(|| {
                    println!("Invalid input, no numbers or special characters.");
                    process::exit(4);
                }),
        );
    }

    positions
}

fn shift(alphabet: &str, positions: Vec<usize>) -> Vec<String> {
    let mut shifted_words: Vec<String> = Vec::new();

    for shift in 0..26 {
        let shifted_word: String = [&alphabet[shift..], &alphabet[..shift]].concat();
        shifted_words.push(
            positions
                .iter()
                .map(|pos| shifted_word.as_bytes()[*pos] as char)
                .collect::<Vec<char>>()
                .into_iter()
                .collect::<String>(),
        );
    }
    shifted_words
}

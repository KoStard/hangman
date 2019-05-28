extern crate rand;

use std::io;
use std::fs;
use rand::seq::SliceRandom;
use std::io::Read;

fn load_words(s: &mut String) -> Vec<&str> {
    s.extend(fs::read_to_string("words.txt")
        .expect("Can't read the words.txt file").chars());
    s.split('\n').collect()
}

fn main() {
    let total_miss = 7u8;


    let mut buffer = String::new();
    let words = load_words(&mut buffer);
    let secret_word = get_secret_word(words);
    let mut guessing_word = secret_word.chars().map(|c| if c != ' ' { '_' } else { ' ' }).collect::<String>();

    let mut guessed_letters: Vec<char> = vec![];

    let mut count = 0;

    println!("{}", guessing_word);

    loop {
        println!("Type your guess.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Did not enter a correct string");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        if input.len() == 0 {println!("Nothing inputted! Try again..."); continue}
        let input_char: char = input.chars().next().unwrap();
        if guessed_letters.contains(&input_char) {
            println!("You have already guessed {}.", input_char);
        } else {
            let indexes = get_letter_indexes(input_char, secret_word);
            if indexes.is_empty() {
                count += 1;
                if total_miss - count != 0 {
                    println!("NAH NAH! Letter not found. You have {} more lives.", total_miss - count);
                }
            } else {
                println!("Letter was found.");
                replace_letters(indexes.clone(), &mut guessing_word);
            }
            println!("{}", guessing_word);
            if count == total_miss { break; }
            guessed_letters.push(input_char);
        }
        if &guessing_word[..] == secret_word {
            println!("You won!");
            return;
        }
    }
    println!("You lost... The word was {}", secret_word);
}


fn get_secret_word(words: Vec<&str>) -> &str {
    words.choose(&mut rand::thread_rng()).unwrap()
}


fn get_letter_indexes(letter: char, secret_word: &str) -> Vec<(usize, char)> {
    secret_word.chars().enumerate().filter(|&(_, l)| l == letter).collect::<Vec<_>>()
}

fn replace_letters(indexes: Vec<(usize, char)>, secret_word: &mut String) {
    for &(idx, letter) in indexes.iter() {
        secret_word.remove(idx);
        secret_word.insert(idx, letter);
    }
}


#[test]
fn test_get_letter_indexes() {
    let indexes = get_letter_indexes('p', "apple");
    assert_eq!((1, 'p'), indexes[0]);
    assert_eq!((2, 'p'), indexes[1]);
    assert_eq!(indexes.len(), 2);
}

#[test]
fn test_get_letter_indexes_not_found() {
    let indexes = get_letter_indexes('c', "apple");
    assert_eq!(indexes.len(), 0);
}

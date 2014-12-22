#![feature(tuple_indexing)]

use std::io;

#[cfg(not(test))]
fn main() {
    let total_miss = 5u;
    
    let secret_word = get_secret_word();

    println!("The secret word is: {}", secret_word);

    // TODO: use regular expression in case of whitespaces, dashes, etc.
    let mut guessing_word = String::from_char(secret_word.len(), '_');

    let mut guessed_letters: Vec<char> = vec![];

    let mut count = 1;
    
    loop {
        println!("Type your guess!");
        let input = io::stdin().read_char().ok().expect("Failed to read line");
        if guessed_letters.contains(&input) {
            println!("You already guessed it.");
        } else {
            let indexes = get_letter_indexes(input, secret_word);
            if indexes.is_empty() {
                count += 1u;
                println!("NAH NAH! Letter not found.");
            } else {
                println!("Letter was found.");
                replace_letters(indexes.clone(), &mut guessing_word);
            }
            println!("{}", guessing_word);
            if count == total_miss { break; }
            guessed_letters.push(input);
        }
        if guessing_word.as_slice() == secret_word { println!("You win!"); return; }
    }
    println!("You lose :-(");
}


fn get_secret_word() -> &'static str {
    "apple"
}


fn get_letter_indexes(letter: char, secret_word: &str) -> Vec<(uint, char)> {
    secret_word.chars().enumerate().filter(|&(_, l)| l == letter).collect::<Vec<_>>()
}

fn replace_letters(indexes: Vec<(uint, char)>, secret_word: &mut String) {
    for &(idx, letter) in indexes.iter() {
        secret_word.remove(idx);
        secret_word.insert(idx, letter);
    }
}


#[test]
fn test_get_secret_word() {
    let secret_word = get_secret_word();
    assert_eq!("apple", secret_word);
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

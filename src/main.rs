use std::io;

#[cfg(not(test))]
fn main() {
    let total_miss = 5u8;
    
    let secret_word = get_secret_word();

    let mut guessing_word = secret_word.chars().map(|c|if c != ' ' {'_'} else {' '}).collect::<String>();

    let mut guessed_letters: Vec<char> = vec![];

    let mut count = 0;
    
    loop {
        println!("Type your guess!");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Did not enter a correct string");
        if let Some('\n')=input.chars().next_back() {
            input.pop();
        }
        if let Some('\r')=input.chars().next_back() {
            input.pop();
        }
        let input_char: char = input.chars().next().unwrap();
        if guessed_letters.contains(&input_char) {
            println!("You already guessed it.");
        } else {
            let indexes = get_letter_indexes(input_char, secret_word);
            if indexes.is_empty() {
                count += 1;
                if total_miss-count != 0 {
                    println!("NAH NAH! Letter not found. You have {} more lives.", total_miss-count);
                }
            } else {
                println!("Letter was found.");
                replace_letters(indexes.clone(), &mut guessing_word);
            }
            println!("{}", guessing_word);
            if count == total_miss { break; }
            guessed_letters.push(input_char);
        }
        if &guessing_word[..] == secret_word { println!("You win!"); return; }
    }
    println!("You lose :-(");
}


fn get_secret_word() -> &'static str {
    "apple"
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
fn test_get_secret_word() {
    //- Invalid test - working only for one word!
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

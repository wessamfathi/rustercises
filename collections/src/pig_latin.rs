use std::{char, io};

pub fn pig_latinify() {
    let mut result = String::new();
    let mut input = String::new();

    println!("Enter the sentence you want to convert:");

    io::stdin()
        .read_line(&mut input)
        .expect("Faied to read line");

    for word in input.trim().split_whitespace() {
        let c = word.chars().next().unwrap();
        if is_vowel(c) {
            result += &(format!(" {}-hay", word));
        } else {
            let mut modified_word = word.to_string();
            let first_char = modified_word.remove(0);
            result += &(format!(" {}-{}ay", modified_word, first_char));
        }
    }

    println!("Modified sentence is:\n{}", result);
}

fn is_vowel(c: char) -> bool {
    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
        return true;
    }

    false
}

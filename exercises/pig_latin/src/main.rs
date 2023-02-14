/* Convert strings to pig latin. The ﬁrst consonant of each word is moved to the end of the word and “ay” is added, so “ﬁrst” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). */

use std::io;
use std::collections::HashMap;

fn main() {
    /* Create a hash map of Some(char) variants */
    let mut map: HashMap<Option<char>, bool> = HashMap::new();
    map.insert(Some('a'), true);
    map.insert(Some('e'), true);
    map.insert(Some('i'), true);
    map.insert(Some('o'), true);
    map.insert(Some('u'), true);

    /* Get user input */
    let mut input: String = String::new();
    println!("Provide input for pig_latining:");
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => println!("{e}"),
    }

    /* Initialize Vector to hold output pig-latin words */
    let mut words: Vec<String> = Vec::new();
    /* split the input on white space */
    for i in input.split_whitespace() {
        /* Turn the string slice into a String */
        let mut word = String::from(i);
        /* Lookup the first character in the hash map */
        /* Note: .chars() returns an iterator over unicode scalar values and not grapheme clusters */
        match map.get(&word.chars().nth(0)) {
            /* Word starts with a vowel */
            Some(c) => words.push(format!("{word}-hay")), 
            /* Word starts with a consonant */
            None => {
                let first_char = word.remove(0);
                words.push(format!("{word}-{first_char}ay"));
            }
        }
    }

    /* Initialize String to hold the combined pig-latin output */
    let mut output_string = String::new();
    for i in words {
        output_string = output_string + &i[..] + &" ";
    }
    println!("{output_string}");
}

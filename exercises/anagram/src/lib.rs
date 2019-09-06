use std::collections::HashSet;
use std::collections::HashMap;

// let letter_add =

fn letter_inventory(word: &str) -> HashMap<char, i32> {
    let mut letters = HashMap::new();

    let scrabble = word.to_lowercase().chars().fold(letters, |mut letters, c| {
        let occurences = letters.entry(c).or_insert(0);
        *occurences += 1;

        letters
        }
    );

    scrabble
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let scrabble = letter_inventory(word);

    // let anagrams = HashSet::new();

    possible_anagrams.into_iter().filter(|w| (*w != &word) && (letter_inventory(*w) == scrabble)).cloned().collect::<HashSet<&'a str>>()
}

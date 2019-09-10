use std::collections::HashMap;
use std::collections::HashSet;

fn letter_inventory(word: &str) -> HashMap<char, i32> {
    let letters = HashMap::new();

    word.to_lowercase().chars().fold(letters, |mut letters, c| {
        let occurences = letters.entry(c).or_insert(0);
        *occurences += 1;

        letters
    })
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let scrabble = letter_inventory(&word);

    // let anagrams = HashSet::new();

    possible_anagrams
        .iter()
        .filter(|w| (w.to_lowercase() != word) && (letter_inventory(*w) == scrabble))
        .cloned()
        .collect::<HashSet<&'a str>>()
}

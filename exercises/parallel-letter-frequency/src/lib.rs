use std::collections::HashMap;

fn str_freq(s: &str) -> HashMap<char, usize> {
    s.to_lowercase()
        .chars()
        .filter(|c| (*c).is_alphabetic())
        .fold(HashMap::new(), |mut h, c| {
            h.entry(c).and_modify(|v| *v += 1).or_insert(1usize);
            h
        })
}

fn chunk_freq(input: &[&str]) -> HashMap<char, usize> {
    str_freq(&input.concat())
}

pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    chunk_freq(input)
}

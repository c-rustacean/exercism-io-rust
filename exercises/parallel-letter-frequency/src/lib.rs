use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

fn str_freq(s: &str) -> HashMap<char, usize> {
    s.to_lowercase()
        .chars()
        .filter(|c| (*c).is_alphabetic())
        .fold(HashMap::new(), |mut h, c| {
            h.entry(c).and_modify(|v| *v += 1).or_insert(1usize);
            h
        })
}

#[cfg(single_thread)]
fn chunk_freq(input: &[&str]) -> HashMap<char, usize> {
    str_freq(&input.concat())
}

#[cfg(single_thread)]
pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
    chunk_freq(input)
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut handles = Vec::with_capacity(worker_count);
    let mut hashes: HashMap<char, usize> = HashMap::new();

    let input = input.concat();

    let len = input.chars().count();
    let chunk_size = (len + worker_count - 1) / worker_count;

    if chunk_size == 0 {
        return HashMap::new();
    } else if chunk_size < 1024usize {
        return str_freq(&input);
    }

    let mut chunks: Vec<String> = input
        .chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    let (tx, rx) = channel();
    let worker_count = chunks.len();
    for _ in 0..worker_count {
        let tx = tx.clone();
        let ch = chunks.pop().unwrap();
        handles.push(thread::spawn(move || {
            tx.send(str_freq(ch.as_str())).unwrap();
        }));
    }

    for _ in 0..worker_count {
        let hash_t = rx.recv().unwrap();
        for (k, v) in hash_t.iter() {
            let counter = hashes.entry(*k).or_insert(0);
            *counter += v;
        }
    }

    for h in handles {
        h.join().expect("Could not join thread");
    }

    hashes
}

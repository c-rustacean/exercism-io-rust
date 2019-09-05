fn alphabet() -> String {
    (b'a'..=b'z').map(char::from).collect::<String>()
}

fn atbash_letter(letter: char) -> char {
    let alphabet = alphabet();

    match alphabet.chars().position(|x| x == letter) {
        Some(pos) => alphabet
            .chars()
            .nth(25 - pos)
            .unwrap_or_else(|| panic!("Unexpected index {}", pos)),
        _ => letter,
    }
}

fn toggle(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.to_ascii_lowercase().is_ascii_alphanumeric())
        .map(atbash_letter)
        .collect()
}

fn char_as_string(t: (usize, char)) -> String {
    let mut r = String::new();

    if (t.0 > 0) && (t.0 % 5 == 0) {
        r.push(' ');
    };
    r.push(t.1);

    r
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    toggle(plain)
        .char_indices()
        .map(char_as_string)
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    toggle(cipher).split_whitespace().collect::<String>()
}

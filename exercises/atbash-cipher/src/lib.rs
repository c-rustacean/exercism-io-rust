// fn alphabet() -> String {
//     (b'a'..=b'z').map(char::from).collect::<String>()
// }

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn atbash_letter(letter: char) -> char {
    // let alphabet = alphabet();

    match ALPHABET.chars().position(|x| x == letter) {
        Some(pos) => ALPHABET

    // match alphabet.chars().position(|x| x == letter) {
    //     Some(pos) => alphabet
            .chars()
            .nth(25 - pos)
            .unwrap_or_else(|| panic!("Unexpected index {}", pos)),
        _ => letter,
    }
}

fn toggle(text: &str) -> impl Iterator<Item = char> + '_ {
    text.chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_ascii_alphanumeric())
        .map(atbash_letter)
}

fn char_as_string(t: (usize, char)) -> String {
    let mut r = String::with_capacity(2); // be ready for a char and a space

    if (t.0 > 0) && (t.0 % 5 == 0) {
        r.push(' ');
    };
    r.push(t.1);

    r
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    toggle(plain)
        .enumerate()
        .map(char_as_string)
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    toggle(cipher).collect::<String>()
}

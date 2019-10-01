fn atbash(letter: u8) -> char {
    if letter.is_ascii_digit() {
        letter
    } else {
        (b'z' - letter + b'a')
    }
    .into()
}

fn toggle(text: &str) -> impl Iterator<Item = char> + '_ {
    text.chars()
        .map(|c| (c as u8).to_ascii_lowercase())
        .filter(|c| c.is_ascii_alphanumeric())
        .map(atbash)
}

fn position_begins_chunk(position: usize) -> bool {
    position > 0 && position % 5 == 0
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    toggle(plain)
        .enumerate()
        .flat_map(|(p, c)| {
            if position_begins_chunk(p) {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect::<String>()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    toggle(cipher).collect::<String>()
}

// fn alphabet() -> String {
//     (b'a'..=b'z').map(char::from).collect::<String>()
// }

// fn atbash(letter: char) -> char {
//     let alphabet = alphabet();

//     let atbashed = alphabet
//         .chars()
//         .zip(alphabet.chars().rev())
//         .filter(|(direct, _reversed)| *direct == letter)
//         .map(|(_d, r)| r)
//         .collect::<Vec<char>>();

//     if atbashed.is_empty() {
//         letter
//     } else {
//         atbashed[0]
//     }
// }

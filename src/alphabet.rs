bitflags! {
    pub struct Alphabets:u8 {
        const NONE      = 0b0000;
        const LOWERCASE = 0b0001;
        const UPPERCASE = 0b0010;
        const ALL       = u8::max_value();
    }
}

const LOWERCASE_LETTERS: &'static str = "a";
const UPPERCASE_LETTERS: &'static str ="A";

pub fn generate_alphabet(subalphabets: Alphabets) -> Vec<char> {
    let mut resulting_alphabet = Vec::new();
    if subalphabets.contains(Alphabets::UPPERCASE) {
        resulting_alphabet.extend(UPPERCASE_LETTERS.chars());
    }
    if subalphabets.contains(Alphabets::LOWERCASE) {
        resulting_alphabet.extend(LOWERCASE_LETTERS.chars());
    }
    resulting_alphabet
}

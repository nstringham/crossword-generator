use std::{fmt::Display, str::FromStr};

use crate::letter::Letter;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Word {}

impl<const N: usize> From<[Letter; N]> for Word {
    fn from(value: [Letter; N]) -> Self {
        todo!()
    }
}

impl FromIterator<Letter> for Word {
    fn from_iter<T: IntoIterator<Item = Letter>>(iter: T) -> Self {
        todo!()
    }
}

impl FromStr for Word {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PartialEq<&str> for Word {
    fn eq(&self, other: &&str) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_word_from_array() {
        let word = Word::from([Letter::H, Letter::E, Letter::L, Letter::L, Letter::O]);
        assert_eq!(word, "Hello");
    }

    #[test]
    fn can_create_word_from_iterator() {
        let letters = [Letter::H, Letter::E, Letter::L, Letter::L, Letter::O];
        let word: Word = letters.into_iter().collect();
        assert_eq!(word, "Hello");
    }

    #[test]
    fn can_parse_word_from_str() {
        let word: Word = "Hello".parse().unwrap();
        assert_eq!(word, "Hello");
    }

    #[test]
    fn invalid_str_fails_to_parse() {
        assert_eq!("Invalid Word".parse::<Word>(), Err(()));
    }

    #[test]
    fn word_can_be_converted_to_string() {
        let word = Word::from([Letter::H, Letter::E, Letter::L, Letter::L, Letter::O]);
        assert_eq!(word.to_string(), "HELLO");
    }

    #[test]
    fn word_equals_str() {
        let word = Word::from([Letter::H, Letter::E, Letter::L, Letter::L, Letter::O]);
        assert_eq!(word, "Hello");
        assert_ne!(word, "World");
    }

    #[test]
    fn word_does_not_equal_invalid_str() {
        let word = Word::from([Letter::H, Letter::E, Letter::L, Letter::L, Letter::O]);
        assert_ne!(word, "Invalid Word");
    }
}

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Display for Letter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        char::from(*self).fmt(f)
    }
}

impl TryFrom<char> for Letter {
    type Error = ();

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            'A' | 'a' => Ok(Letter::A),
            'B' | 'b' => Ok(Letter::B),
            'C' | 'c' => Ok(Letter::C),
            'D' | 'd' => Ok(Letter::D),
            'E' | 'e' => Ok(Letter::E),
            'F' | 'f' => Ok(Letter::F),
            'G' | 'g' => Ok(Letter::G),
            'H' | 'h' => Ok(Letter::H),
            'I' | 'i' => Ok(Letter::I),
            'J' | 'j' => Ok(Letter::J),
            'K' | 'k' => Ok(Letter::K),
            'L' | 'l' => Ok(Letter::L),
            'M' | 'm' => Ok(Letter::M),
            'N' | 'n' => Ok(Letter::N),
            'O' | 'o' => Ok(Letter::O),
            'P' | 'p' => Ok(Letter::P),
            'Q' | 'q' => Ok(Letter::Q),
            'R' | 'r' => Ok(Letter::R),
            'S' | 's' => Ok(Letter::S),
            'T' | 't' => Ok(Letter::T),
            'U' | 'u' => Ok(Letter::U),
            'V' | 'v' => Ok(Letter::V),
            'W' | 'w' => Ok(Letter::W),
            'X' | 'x' => Ok(Letter::X),
            'Y' | 'y' => Ok(Letter::Y),
            'Z' | 'z' => Ok(Letter::Z),
            _ => Err(()),
        }
    }
}

impl From<Letter> for char {
    fn from(letter: Letter) -> Self {
        match letter {
            Letter::A => 'A',
            Letter::B => 'B',
            Letter::C => 'C',
            Letter::D => 'D',
            Letter::E => 'E',
            Letter::F => 'F',
            Letter::G => 'G',
            Letter::H => 'H',
            Letter::I => 'I',
            Letter::J => 'J',
            Letter::K => 'K',
            Letter::L => 'L',
            Letter::M => 'M',
            Letter::N => 'N',
            Letter::O => 'O',
            Letter::P => 'P',
            Letter::Q => 'Q',
            Letter::R => 'R',
            Letter::S => 'S',
            Letter::T => 'T',
            Letter::U => 'U',
            Letter::V => 'V',
            Letter::W => 'W',
            Letter::X => 'X',
            Letter::Y => 'Y',
            Letter::Z => 'Z',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letter_is_one_byte() {
        assert_eq!(std::mem::size_of::<Letter>(), 1);
    }

    #[test]
    fn letter_can_use_niche_optimization() {
        assert_eq!(
            std::mem::size_of::<Option<Letter>>(),
            std::mem::size_of::<Letter>()
        );
    }

    #[test]
    fn letter_can_be_parsed_from_char() {
        assert_eq!(Letter::try_from('A'), Ok(Letter::A));
        assert_eq!(Letter::try_from('Z'), Ok(Letter::Z));
        assert_eq!(Letter::try_from('a'), Ok(Letter::A));
        assert_eq!(Letter::try_from('z'), Ok(Letter::Z));
        assert_eq!(Letter::try_from('1'), Err(()));
        assert_eq!(Letter::try_from(' '), Err(()));
    }

    #[test]
    fn letter_can_be_converted_to_char() {
        assert_eq!(char::from(Letter::A), 'A');
        assert_eq!(char::from(Letter::Z), 'Z');
    }

    #[test]
    fn letter_can_be_converted_to_string() {
        assert_eq!(Letter::A.to_string(), "A");
        assert_eq!(Letter::Z.to_string(), "Z");
    }
}

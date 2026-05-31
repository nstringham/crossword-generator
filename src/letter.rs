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
        todo!()
    }
}

impl TryFrom<char> for Letter {
    type Error = ();

    fn try_from(character: char) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl From<Letter> for char {
    fn from(letter: Letter) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn letter_can_be_converted_to_string() {
        assert_eq!(Letter::A.to_string(), "A");
        assert_eq!(Letter::Z.to_string(), "Z");
    }
}

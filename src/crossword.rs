use crate::{grid::Grid, letter::Letter, word::Word};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Across,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct WordLocation {
    pub x: usize,
    pub y: usize,
    pub direction: Direction,
    pub length: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Crossword {
    words: Vec<WordLocation>,
    grid: Grid<Option<Letter>>,
}

impl Crossword {
    pub fn new() -> Self {
        Crossword {
            words: Vec::new(),
            grid: Grid::new(0, 0),
        }
    }

    pub fn try_place_word(
        &mut self,
        x: isize,
        y: isize,
        direction: Direction,
        word: &Word,
    ) -> Result<(), ()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_crossword_can_be_created() {
        let crossword = Crossword::new();
        assert!(crossword.words.is_empty());
        assert_eq!(crossword.grid.width(), 0);
        assert_eq!(crossword.grid.height(), 0);
    }

    #[test]
    fn word_can_be_placed_in_empty_crossword() {
        let mut crossword = Crossword::new();
        let word = Word::from("HELLO");
        assert_eq!(
            crossword.try_place_word(0, 0, Direction::Across, &word),
            Ok(())
        );
        assert_eq!(
            crossword.words,
            [WordLocation {
                x: 0,
                y: 0,
                direction: Direction::Across,
                length: 5
            }]
        );
        assert_eq!(crossword.grid, ["HELLO"]);
    }

    #[test]
    fn word_can_be_placed_overlapping_existing_word() {
        let mut crossword = Crossword::new();
        let first = Word::from("FIRST");
        let second = Word::from("SECOND");
        let third = Word::from("THIRD");
        assert_eq!(
            crossword.try_place_word(0, 0, Direction::Across, &first),
            Ok(())
        );
        assert_eq!(
            crossword.try_place_word(3, 0, Direction::Down, &second),
            Ok(())
        );
        assert_eq!(
            crossword.try_place_word(-1, 5, Direction::Across, &third),
            Ok(())
        );
        assert_eq!(
            crossword.words,
            [
                WordLocation {
                    x: 1,
                    y: 0,
                    direction: Direction::Across,
                    length: 5
                },
                WordLocation {
                    x: 4,
                    y: 0,
                    direction: Direction::Down,
                    length: 6
                },
                WordLocation {
                    x: 0,
                    y: 5,
                    direction: Direction::Across,
                    length: 5
                }
            ]
        );

        assert_eq!(
            crossword.grid,
            [
                " FIRST", //
                "    E ", //
                "    C ", //
                "    O ", //
                "    N ", //
                "THIRD "  //
            ]
        );
    }

    #[test]
    fn word_cannot_be_placed_overlapping_conflicting_letter() {
        let mut crossword = Crossword::new();
        let first = Word::from("FIRST");
        let second = Word::from("SECOND");
        assert_eq!(
            crossword.try_place_word(0, 0, Direction::Across, &first),
            Ok(())
        );
        assert_eq!(
            crossword.try_place_word(2, -2, Direction::Down, &second),
            Err(())
        );
    }
}

use std::cmp::{max, min};

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

impl WordLocation {
    pub fn positions(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.length).map(|i| match self.direction {
            Direction::Across => (self.x + i, self.y),
            Direction::Down => (self.x, self.y + i),
        })
    }
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

    fn expand_grid_to_fit_word(
        &mut self,
        x: isize,
        y: isize,
        direction: Direction,
        length: usize,
    ) -> WordLocation {
        let (end_x, end_y) = match direction {
            Direction::Across => (x + length as isize, y),
            Direction::Down => (x, y + length as isize),
        };

        let left = min(0 - x, 0) as usize;
        let top = min(0 - y, 0) as usize;
        let right = min(self.grid.width() as isize - end_x, 0) as usize;
        let bottom = min(self.grid.height() as isize - end_y, 0) as usize;

        self.grid.expand(left, top, right, bottom);

        for word in &mut self.words {
            word.x += left;
            word.y += top;
        }

        WordLocation {
            x: max(x, 0) as usize,
            y: max(y, 0) as usize,
            direction,
            length,
        }
    }

    pub fn try_place_word(
        &mut self,
        x: isize,
        y: isize,
        direction: Direction,
        word: &Word,
    ) -> Result<(), ()> {
        let location = self.expand_grid_to_fit_word(x, y, direction, word.len());

        for (position, &letter) in location.positions().zip(word.iter()) {
            if matches!(self.grid[position], Some(existing_letter) if existing_letter != letter) {
                return Err(());
            }
        }

        self.words.push(location);

        for (position, &letter) in location.positions().zip(word.iter()) {
            self.grid[position] = Some(letter);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_iterate_over_horizontal_word_location_positions() {
        let location = WordLocation {
            x: 1,
            y: 2,
            direction: Direction::Across,
            length: 3,
        };
        let positions: Vec<(usize, usize)> = location.positions().collect();
        assert_eq!(positions, [(1, 2), (2, 2), (3, 2)]);
    }

    #[test]
    fn can_iterate_over_vertical_word_location_positions() {
        let location = WordLocation {
            x: 1,
            y: 2,
            direction: Direction::Down,
            length: 3,
        };
        let positions: Vec<(usize, usize)> = location.positions().collect();
        assert_eq!(positions, [(1, 2), (1, 3), (1, 4)]);
    }

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

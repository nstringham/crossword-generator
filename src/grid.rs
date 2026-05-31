use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        todo!();
    }
}

impl<T> Grid<T> {
    pub fn width(&self) -> usize {
        todo!();
    }

    pub fn height(&self) -> usize {
        todo!();
    }
}

impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    fn index(&self, y: usize) -> &Self::Output {
        todo!()
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, y: usize) -> &mut Self::Output {
        todo!()
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        todo!()
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        todo!()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> From<[[T; WIDTH]; HEIGHT]> for Grid<T> {
    fn from(value: [[T; WIDTH]; HEIGHT]) -> Self {
        todo!()
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> PartialEq<[[T; WIDTH]; HEIGHT]> for Grid<T> {
    fn eq(&self, other: &[[T; WIDTH]; HEIGHT]) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn grid_can_be_created_with_width_and_height() {
        let grid: Grid<u8> = Grid::new(5, 3);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 3);
        assert_eq!(grid, [[0; 5]; 3]);
    }

    #[test]
    fn grid_can_be_indexed_by_y() {
        let mut grid: Grid<u8> = Grid::new(5, 3);
        grid[1][2] = 42;
        assert_eq!(grid[1], [0, 0, 42, 0, 0]);
    }

    #[test]
    fn grid_can_be_indexed_by_coordinates() {
        let mut grid: Grid<u8> = Grid::new(5, 3);
        grid[(2, 1)] = 42;
        assert_eq!(grid[(2, 1)], 42);
        assert_eq!(grid[1][2], 42);
    }

    #[test]
    fn grid_can_be_created_from_array() {
        let array = [[1, 2, 3], [4, 5, 6]];
        let grid: Grid<u8> = array.into();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid, array);
    }
}

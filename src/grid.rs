pub mod iter;

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

    pub fn iter(&self) -> iter::Iter<'_, T> {
        todo!();
    }

    pub fn iter_mut(&mut self) -> iter::IterMut<'_, T> {
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

    #[test]
    fn gird_can_be_iterated_over() {
        let grid: Grid<u8> = [[1, 2, 3], [4, 5, 6]].into();
        let mut grid_iter = grid.iter();

        let row = grid_iter.next().unwrap();
        let mut row_iter = row.iter();
        assert_eq!(row_iter.next(), Some(&1));
        assert_eq!(row_iter.next(), Some(&2));
        assert_eq!(row_iter.next(), Some(&3));
        assert_eq!(row_iter.next(), None);

        let row = grid_iter.next().unwrap();
        let mut row_iter = row.iter();
        assert_eq!(row_iter.next(), Some(&4));
        assert_eq!(row_iter.next(), Some(&5));
        assert_eq!(row_iter.next(), Some(&6));
        assert_eq!(row_iter.next(), None);

        assert_eq!(grid_iter.next(), None);
    }

    #[test]
    fn gird_can_be_mutably_iterated_over() {
        let mut grid: Grid<u8> = [[1, 2, 3], [4, 5, 6]].into();
        let mut grid_iter = grid.iter_mut();

        let row = grid_iter.next().unwrap();
        let mut row_iter = row.iter_mut();
        assert_eq!(row_iter.next(), Some(&mut 1));
        assert_eq!(row_iter.next(), Some(&mut 2));
        assert_eq!(row_iter.next(), Some(&mut 3));
        assert_eq!(row_iter.next(), None);

        let row = grid_iter.next().unwrap();
        let mut row_iter = row.iter_mut();
        assert_eq!(row_iter.next(), Some(&mut 4));
        assert_eq!(row_iter.next(), Some(&mut 5));
        assert_eq!(row_iter.next(), Some(&mut 6));
        assert_eq!(row_iter.next(), None);

        assert_eq!(grid_iter.next(), None);
    }
}

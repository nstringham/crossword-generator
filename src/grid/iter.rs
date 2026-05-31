use super::Grid;

pub struct Iter<'a, T> {
    phantom: std::marker::PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a [T];

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

pub struct IterMut<'a, T> {
    phantom: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut [T];

    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_loops_over_rows() {
        let grid = Grid::from([[1, 2], [3, 4]]);
        let mut iter = grid.into_iter();
        assert_eq!(iter.next(), Some(&[1, 2][..]));
        assert_eq!(iter.next(), Some(&[3, 4][..]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut_loops_over_rows() {
        let mut grid = Grid::from([[1, 2], [3, 4]]);
        let mut iter = grid.iter_mut();
        assert_eq!(iter.next(), Some(&mut [1, 2][..]));
        assert_eq!(iter.next(), Some(&mut [3, 4][..]));
        assert_eq!(iter.next(), None);
    }
}

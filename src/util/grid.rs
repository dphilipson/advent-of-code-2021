use crate::harness::input::{LineInput, RawInput};
use crate::util::coords::Coord2;
use ndarray::Array2;
use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grid<T>(pub Array2<T>);

impl Grid<u32> {
    pub fn parse_digits(s: &str) -> Self {
        parse_grid(s, |line| line.digits())
    }
}

impl Grid<char> {
    pub fn parse_chars(s: &str) -> Self {
        parse_grid(s, |line| line.chars())
    }
}

impl Grid<u8> {
    pub fn parse_bytes(s: &str) -> Self {
        parse_grid(s, |line| line.bytes())
    }
}

impl<T> Grid<T>
where
    T: Default + FromStr,
    <T as FromStr>::Err: Debug,
{
    pub fn parse_on_whitespace(s: &str) -> Self {
        parse_grid(s, |line| line.split_whitespace())
    }
}

impl<T> Index<[usize; 2]> for Grid<T> {
    type Output = T;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T> Grid<T> {
    pub fn nrows(&self) -> usize {
        self.0.nrows()
    }

    pub fn ncols(&self) -> usize {
        self.0.ncols()
    }

    pub fn neighbors(&self, [i, j]: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
        let nrows = self.nrows();
        let ncols = self.ncols();
        Coord2(i as i32, j as i32)
            .neighbors()
            .into_iter()
            .map(|Coord2(i, j)| [i as usize, j as usize])
            .filter(move |&[i, j]| i < nrows && j < ncols)
    }

    pub fn orthogonal_neighbors(&self, [i, j]: [usize; 2]) -> impl Iterator<Item = [usize; 2]> {
        let nrows = self.nrows();
        let ncols = self.ncols();
        Coord2(i as i32, j as i32)
            .orthogonal_neighbors()
            .into_iter()
            .map(|Coord2(i, j)| [i as usize, j as usize])
            .filter(move |&[i, j]| i < nrows && j < ncols)
    }

    pub fn indices(&self) -> impl Iterator<Item = [usize; 2]> {
        let nrows = self.nrows();
        let ncols = self.ncols();
        (0..nrows).flat_map(move |i| (0..ncols).map(move |j| [i, j]))
    }

    pub fn map<'a, U, F>(&'a self, f: F) -> Grid<U>
    where
        F: FnMut(&'a T) -> U,
        T: 'a,
    {
        Grid(self.0.map(f))
    }
}

fn parse_grid<T: Default>(input: &str, f: impl Fn(LineInput) -> Vec<T>) -> Grid<T> {
    let values = RawInput::new(input).per_line(f);
    let n_rows = values.len();
    let n_cols = values[0].len();
    let mut result = Array2::default((n_rows, n_cols));
    for (i, row) in values.into_iter().enumerate() {
        for (j, value) in row.into_iter().enumerate() {
            result[[i, j]] = value;
        }
    }
    Grid(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_parse_digits() {
        let input = RawInput::new("123\n456\n789");
        let grid = Grid::parse_digits(input.as_str());
        let expected = Grid(ndarray::arr2(&[[1, 2, 3], [4, 5, 6], [7, 8, 9]]));
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid(Array2::<i32>::zeros((4, 4)));
        let neighbors: HashSet<_> = grid.neighbors([1, 2]).into_iter().collect();
        let expected: HashSet<_> = [
            [0, 1],
            [1, 1],
            [2, 1],
            [0, 2],
            [2, 2],
            [0, 3],
            [1, 3],
            [2, 3],
        ]
        .into_iter()
        .collect();
        assert_eq!(neighbors, expected);

        let neighbors: HashSet<_> = grid.neighbors([0, 3]).collect();
        let expected: HashSet<_> = [[0, 2], [1, 2], [1, 3]].into_iter().collect();
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_orthogonal_neighbors() {
        let grid = Grid(Array2::<i32>::zeros((4, 4)));
        let neighbors: HashSet<_> = grid.orthogonal_neighbors([1, 2]).collect();
        let expected: HashSet<_> = [[0, 2], [2, 2], [1, 1], [1, 3]].into_iter().collect();
        assert_eq!(neighbors, expected);

        let neighbors: HashSet<_> = grid.orthogonal_neighbors([0, 3]).into_iter().collect();
        let expected: HashSet<_> = [[1, 3], [0, 2]].into_iter().collect();
        assert_eq!(neighbors, expected);
    }
}

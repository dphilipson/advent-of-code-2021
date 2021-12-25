use crate::harness::input::RawInput;
use crate::util::grid::Grid;
use ndarray::Array2;
use std::collections::HashSet;

pub fn solve_part1(input: RawInput) -> usize {
    let bytes = Grid::parse_bytes(input.as_str());
    let mut state = State(bytes.0.map(|&b| b.into()));
    let mut step_count = 1;
    while state.advance() {
        step_count += 1;
    }
    step_count
}

pub fn solve_part2(_: RawInput) -> usize {
    todo!()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Space {
    Empty,
    EastFacing,
    SouthFacing,
}

impl From<u8> for Space {
    fn from(byte: u8) -> Self {
        match byte {
            b'>' => Self::EastFacing,
            b'v' => Self::SouthFacing,
            b'.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct State(Array2<Space>);

impl State {
    fn advance(&mut self) -> bool {
        let moved_east = self.advance_east_facing();
        let moved_south = self.advance_south_facing();
        moved_east || moved_south
    }

    fn advance_east_facing(&mut self) -> bool {
        let mut locations_to_move = HashSet::<[usize; 2]>::new();
        for i in 0..self.0.nrows() {
            for j in 0..self.0.ncols() {
                let next_j = (j + 1) % self.0.ncols();
                if self.0[[i, j]] == Space::EastFacing && self.0[[i, next_j]] == Space::Empty {
                    locations_to_move.insert([i, j]);
                }
            }
        }
        for &[i, j] in &locations_to_move {
            let next_j = (j + 1) % self.0.ncols();
            self.0[[i, j]] = Space::Empty;
            self.0[[i, next_j]] = Space::EastFacing;
        }
        !locations_to_move.is_empty()
    }

    fn advance_south_facing(&mut self) -> bool {
        let mut locations_to_move = HashSet::<[usize; 2]>::new();
        for i in 0..self.0.nrows() {
            for j in 0..self.0.ncols() {
                let next_i = (i + 1) % self.0.nrows();
                if self.0[[i, j]] == Space::SouthFacing && self.0[[next_i, j]] == Space::Empty {
                    locations_to_move.insert([i, j]);
                }
            }
        }
        for &[i, j] in &locations_to_move {
            let next_i = (i + 1) % self.0.nrows();
            self.0[[i, j]] = Space::Empty;
            self.0[[next_i, j]] = Space::SouthFacing;
        }
        !locations_to_move.is_empty()
    }
}

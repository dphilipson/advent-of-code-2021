use crate::harness::input::RawInput;
use crate::util::grid::Grid;
use crate::util::search::dijkstra;
use ndarray::Array2;

pub fn solve_part1(input: RawInput) -> usize {
    let grid = Grid::parse_digits(input.as_str());
    shortest_path_length(&grid)
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = Grid::parse_digits(input.as_str());
    let grid = expand_grid(&grid);
    shortest_path_length(&grid)
}

fn shortest_path_length(grid: &Grid<u32>) -> usize {
    dijkstra::search(
        [0, 0],
        |&ij| {
            grid.orthogonal_neighbors(ij)
                .map(|neighbor| (neighbor, grid[neighbor] as usize))
                .collect()
        },
        |&[i, j]| i == grid.nrows() - 1 && j == grid.ncols() - 1,
    )
    .goal_state()
    .unwrap()
    .distance
}

fn expand_grid(grid: &Grid<u32>) -> Grid<u32> {
    let mut result: Array2<u32> = Array2::zeros((grid.nrows() * 5, grid.ncols() * 5));
    for i in 0..5 {
        for j in 0..5 {
            for ii in 0..grid.nrows() {
                for jj in 0..grid.ncols() {
                    result[[i * grid.nrows() + ii, j * grid.ncols() + jj]] =
                        (grid[[ii, jj]] + i as u32 + j as u32 - 1) % 9 + 1
                }
            }
        }
    }
    Grid(result)
}

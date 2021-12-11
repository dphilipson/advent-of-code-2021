use crate::harness::input::RawInput;
use crate::util::grid::Grid;
use crate::util::search::bfs;
use ndarray::Array2;

pub fn solve_part1(input: RawInput) -> u32 {
    let grid = Grid::parse_digits(&input);
    get_low_points(&grid)
        .into_iter()
        .map(|ij| grid[ij] + 1)
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = Grid::parse_digits(&input);
    let mut basin_sizes = get_low_points(&grid)
        .into_iter()
        .map(|low_point| get_basin_size(&grid, low_point))
        .collect::<Vec<_>>();
    basin_sizes.sort();
    basin_sizes.reverse();
    basin_sizes.into_iter().take(3).product()
}

fn parse_grid(input: RawInput) -> Array2<u32> {
    let lines = input.per_line(|line| line.chars());
    let n_rows = lines.len();
    let n_cols = lines[0].len();
    let mut result = Array2::default((n_rows, n_cols));
    for i in 0..n_rows {
        for j in 0..n_cols {
            result[[i, j]] = lines[i][j].to_digit(10).unwrap();
        }
    }
    result
}

fn get_low_points(grid: &Grid<u32>) -> Vec<[usize; 2]> {
    grid.indexed_iter()
        .filter(|&((i, j), &value)| grid.orthogonal_neighbors([i, j]).all(|ij| grid[ij] > value))
        .map(|((i, j), _)| [i, j])
        .collect()
}

fn get_basin_size(grid: &Grid<u32>, low_point: [usize; 2]) -> usize {
    let search_result = bfs::search(
        low_point,
        |&ij| {
            grid.orthogonal_neighbors(ij)
                .filter(|&ij| grid[ij] != 9)
                .collect()
        },
        |_| false,
    );
    search_result.seen_states.len()
}

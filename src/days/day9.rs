use crate::harness::input::RawInput;
use crate::util::coords::Coord2;
use crate::util::search::bfs;
use ndarray::Array2;

pub fn solve_part1(input: RawInput) -> u32 {
    let grid = parse_grid(input);
    get_low_points(&grid)
        .into_iter()
        .map(|ij| grid[ij] + 1)
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = parse_grid(input);
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

fn get_low_points(grid: &Array2<u32>) -> Vec<[usize; 2]> {
    grid.indexed_iter()
        .filter(|&((i, j), &value)| neighbors(&grid, [i, j]).iter().all(|&ij| grid[ij] > value))
        .map(|((i, j), _)| [i, j])
        .collect()
}

fn get_basin_size(grid: &Array2<u32>, low_point: [usize; 2]) -> usize {
    let search_result = bfs::search(
        low_point,
        |&ij| {
            neighbors(&grid, ij)
                .into_iter()
                .filter(|&ij| grid[ij] != 9)
                .collect()
        },
        |_| false,
    );
    search_result.seen_states.len()
}

fn neighbors(grid: &Array2<u32>, [i, j]: [usize; 2]) -> Vec<[usize; 2]> {
    Coord2(i as i32, j as i32)
        .orthogonal_neighbors()
        .into_iter()
        .map(|Coord2(i, j)| [i as usize, j as usize])
        .filter(|&[i, j]| i < grid.nrows() && j < grid.ncols())
        .collect()
}

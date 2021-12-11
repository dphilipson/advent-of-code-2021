use crate::harness::input::RawInput;
use crate::util::coords::Coord2;
use ndarray::Array2;

pub fn solve_part1(input: RawInput) -> usize {
    let mut grid = parse_grid(input);
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += apply_step(&mut grid);
    }
    flash_count
}

pub fn solve_part2(input: RawInput) -> i32 {
    let mut grid = parse_grid(input);
    for step in 1.. {
        let flash_count = apply_step(&mut grid);
        if flash_count == grid.len() {
            return step;
        }
    }
    unreachable!()
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

fn apply_step(grid: &mut Array2<u32>) -> usize {
    let mut will_flash = Vec::<[usize; 2]>::new();
    let mut did_flash = Vec::<[usize; 2]>::new();
    for i in 0..grid.nrows() {
        for j in 0..grid.ncols() {
            let ij = [i, j];
            grid[ij] += 1;
            if grid[ij] == 10 {
                will_flash.push(ij);
            }
        }
    }
    while let Some(ij) = will_flash.pop() {
        for neighbor in neighbors(&grid, ij) {
            grid[neighbor] += 1;
            if grid[neighbor] == 10 {
                will_flash.push(neighbor);
            }
        }
        did_flash.push(ij);
    }
    for &ij in &did_flash {
        grid[ij] = 0;
    }
    did_flash.len()
}

fn neighbors(grid: &Array2<u32>, [i, j]: [usize; 2]) -> Vec<[usize; 2]> {
    Coord2(i as i32, j as i32)
        .neighbors()
        .into_iter()
        .map(|Coord2(i, j)| [i as usize, j as usize])
        .filter(|&[i, j]| i < grid.nrows() && j < grid.ncols())
        .collect()
}

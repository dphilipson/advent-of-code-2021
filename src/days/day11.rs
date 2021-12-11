use crate::harness::input::RawInput;
use crate::util::grid::Grid;

pub fn solve_part1(input: RawInput) -> usize {
    let mut grid = Grid::parse_digits(&input);
    let mut flash_count = 0;
    for _ in 0..100 {
        flash_count += apply_step(&mut grid);
    }
    flash_count
}

pub fn solve_part2(input: RawInput) -> i32 {
    let mut grid = Grid::parse_digits(&input);
    for step in 1.. {
        let flash_count = apply_step(&mut grid);
        if flash_count == grid.len() {
            return step;
        }
    }
    unreachable!()
}

fn apply_step(grid: &mut Grid<u32>) -> usize {
    let mut will_flash = Vec::<[usize; 2]>::new();
    let mut did_flash = Vec::<[usize; 2]>::new();
    for ij in grid.indices() {
        up(grid, &mut will_flash, ij);
    }
    while let Some(ij) = will_flash.pop() {
        for neighbor in grid.neighbors(ij) {
            up(grid, &mut will_flash, neighbor);
        }
        did_flash.push(ij);
    }
    for &ij in &did_flash {
        grid[ij] = 0;
    }
    did_flash.len()
}

fn up(grid: &mut Grid<u32>, will_flash: &mut Vec<[usize; 2]>, ij: [usize; 2]) {
    grid[ij] += 1;
    if grid[ij] == 10 {
        will_flash.push(ij);
    }
}

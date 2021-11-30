#![allow(dead_code)]
mod coords;
mod days;
mod harness;
mod input;
mod nums;
mod panics;
mod re;

use days::day1 as day;
const DAY: usize = 1;

fn main() {
    harness::solve(DAY, day::solve_part1, day::solve_part2);
}

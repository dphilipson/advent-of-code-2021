use crate::harness::input::RawInput;
use std::cmp;

pub fn solve_part1(input: RawInput) -> i32 {
    let mut nums = input.single_line(|line| line.split::<i32>(","));
    nums.sort();
    let median = nums[nums.len() / 2];
    nums.iter().map(|n| (n - median).abs()).sum()
}

pub fn solve_part2(input: RawInput) -> i32 {
    let nums = input.single_line(|line| line.split::<i32>(","));
    let mean = nums.iter().sum::<i32>() / nums.len() as i32;
    cmp::min(
        triangle_diff_sum(&nums, mean),
        triangle_diff_sum(&nums, mean + 1),
    )
}

fn triangle_diff_sum(nums: &[i32], x: i32) -> i32 {
    nums.iter().map(|n| triangle(n - x)).sum()
}

fn triangle(n: i32) -> i32 {
    let n = n.abs();
    n * (n + 1) / 2
}

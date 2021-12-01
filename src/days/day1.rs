use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    let depths = input.per_line(|line| line.single::<usize>());
    count_increases(&depths)
}

pub fn solve_part2(input: RawInput) -> usize {
    let depths = input.per_line(|line| line.single::<usize>());
    let mut windows: Vec<usize> = vec![];
    for i in 2..depths.len() {
        windows.push(depths[i] + depths[i - 1] + depths[i - 2])
    }
    count_increases(&windows)
}

fn count_increases(ns: &[usize]) -> usize {
    (1..ns.len()).filter(|&i| ns[i] > ns[i - 1]).count()
}

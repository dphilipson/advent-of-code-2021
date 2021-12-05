use crate::harness::input::RawInput;
use crate::regex;
use std::cmp;
use std::collections::HashMap;

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, false)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, true)
}

fn solve(input: RawInput, count_diagonals: bool) -> usize {
    let re = regex!(r"^(\d+),(\d+) -> (\d+),(\d+)$");
    let lines = input.per_line(|line| line.parse_with_regex::<(i16, i16, i16, i16)>(re));
    let mut point_counts = HashMap::<(i16, i16), u8>::new();
    for (x1, y1, x2, y2) in lines {
        if x1 == x2 {
            let min_y = cmp::min(y1, y2);
            let max_y = cmp::max(y1, y2);
            for y in min_y..=max_y {
                *point_counts.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            let min_x = cmp::min(x1, x2);
            let max_x = cmp::max(x1, x2);
            for x in min_x..=max_x {
                *point_counts.entry((x, y1)).or_default() += 1;
            }
        } else if count_diagonals {
            let (x1, y1, x2, y2) = if x1 < x2 {
                (x1, y1, x2, y2)
            } else {
                (x2, y2, x1, y1)
            };
            let y_sign = if y1 < y2 { 1 } else { -1 };
            for i in 0..=x2 - x1 {
                *point_counts.entry((x1 + i, y1 + y_sign * i)).or_default() += 1;
            }
        }
    }
    point_counts.values().filter(|&n| *n > 1).count()
}

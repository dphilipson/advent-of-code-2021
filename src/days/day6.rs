use crate::harness::input::RawInput;
use ndarray::{Array1, Array2};

pub fn solve_part1(input: RawInput) -> i64 {
    solve(input, 80)
}

pub fn solve_part2(input: RawInput) -> i64 {
    solve(input, 256)
}

fn solve(input: RawInput, days: usize) -> i64 {
    let ages = input.single_line(|line| line.split::<usize>(","));
    let mut age_counts = Array1::<i64>::zeros((9,));
    for age in ages {
        age_counts[[age]] += 1;
    }
    let one_day_transform = ndarray::arr2(&[
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
    ]);
    let n_days_transform = matrix_pow(one_day_transform, days);
    n_days_transform.dot(&age_counts).sum()
}

fn matrix_pow(mut matrix: Array2<i64>, mut n: usize) -> Array2<i64> {
    let mut result = Array2::eye(matrix.nrows());
    loop {
        if n & 1 == 1 {
            result = result.dot(&matrix);
        }
        n >>= 1;
        if n == 0 {
            return result;
        }
        matrix = matrix.dot(&matrix);
    }
}

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> i32 {
    let lines = input.per_line(|line| line.chars());
    let max_digits = (0..lines[0].len())
        .map(|i| {
            if lines.iter().filter(|c| c[i] == '1').count() > lines.len() / 2 {
                1
            } else {
                0
            }
        })
        .collect::<Vec<_>>();
    let gamma = max_digits.iter().fold(0, |acc, &x| 2 * acc + x);
    let epsilon = max_digits.iter().fold(0, |acc, &x| 2 * acc + 1 - x);
    gamma * epsilon
}

pub fn solve_part2(input: RawInput) -> i32 {
    let lines = input.per_line(|line| line.chars());
    let oxy = get_rating(&lines, true);
    let co2 = get_rating(&lines, false);
    oxy * co2
}

fn get_rating(nums: &[Vec<char>], use_more_common: bool) -> i32 {
    let mut current_nums = nums.to_owned();
    for i in 0..current_nums[0].len() {
        if current_nums.len() == 1 {
            break;
        }
        apply_step(&mut current_nums, i, use_more_common);
    }
    current_nums[0]
        .iter()
        .fold(0, |acc, c| 2 * acc + if *c == '0' { 0 } else { 1 })
}

fn apply_step(nums: &mut Vec<Vec<char>>, bit_position: usize, use_more_common: bool) {
    let digit = if one_is_most_common(&nums, bit_position) == use_more_common {
        '1'
    } else {
        '0'
    };
    nums.retain(|num| num[bit_position] == digit);
}

fn one_is_most_common(nums: &[Vec<char>], bit_position: usize) -> bool {
    nums.iter().filter(|c| c[bit_position] == '1').count() >= (nums.len() + 1) / 2
}

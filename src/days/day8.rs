use crate::harness::input::RawInput;
use std::collections::HashMap;

pub fn solve_part1(input: RawInput) -> usize {
    let lines = input.per_line(|line| line.single::<String>());
    lines
        .iter()
        .flat_map(|line| parse_line(line).1)
        .filter(|&s| [2, 3, 4, 7].contains(&s.len()))
        .count()
}

pub fn solve_part2(input: RawInput) -> usize {
    let lines = input.per_line(|line| line.single::<String>());
    lines.iter().map(|line| solve_line(line)).sum()
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = line.split(" | ");
    let all_patterns = parts.next().unwrap().split(' ').collect();
    let output_pattern = parts.next().unwrap().split(' ').collect();
    (all_patterns, output_pattern)
}

fn solve_line(line: &str) -> usize {
    let (all_patterns, output_patterns) = parse_line(line);
    let decoder = Decoder::new(&all_patterns);
    decoder.decode_digits(&output_patterns)
}

#[derive(Copy, Clone, Debug)]
struct Decoder {
    bottom_left: char,
    top_left: char,
    bottom_and_center: [char; 2],
}

impl Decoder {
    fn new(patterns: &[&str]) -> Self {
        let char_counts = count_chars(patterns);
        let mut bottom_and_center = ('a'..='g').filter(|c| char_counts[c] == 7);
        Self {
            bottom_left: ('a'..='g').find(|c| char_counts[c] == 4).unwrap(),
            top_left: ('a'..='g').find(|c| char_counts[c] == 6).unwrap(),
            bottom_and_center: [
                bottom_and_center.next().unwrap(),
                bottom_and_center.next().unwrap(),
            ],
        }
    }

    fn decode_digits(&self, digits: &[&str]) -> usize {
        digits
            .iter()
            .fold(0, |acc, digit| 10 * acc + self.decode_digit(digit))
    }

    fn decode_digit(&self, s: &str) -> usize {
        let &Self {
            bottom_left,
            top_left,
            bottom_and_center,
        } = self;
        match s.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 if s.contains(bottom_left) => 2,
            5 if s.contains(top_left) => 5,
            5 => 3,
            6 if !s.contains(bottom_left) => 9,
            6 if bottom_and_center.iter().all(|&seg| s.contains(seg)) => 6,
            6 => 0,
            7 => 8,
            _ => unreachable!(),
        }
    }
}

fn count_chars(patterns: &[&str]) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    patterns
        .iter()
        .flat_map(|&s| s.chars())
        .for_each(|c| *result.entry(c).or_default() += 1);
    result
}

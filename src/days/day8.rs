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
    let legend = make_legend(&all_patterns);
    apply_legend(&legend, &output_patterns)
}

fn make_legend(patterns: &[&str]) -> HashMap<String, usize> {
    let patterns = patterns.iter().map(|s| sort_str(s)).collect::<Vec<_>>();
    let char_counts = count_chars(&patterns);
    let bottom_left = ('a'..='g').find(|c| char_counts[c] == 4).unwrap();
    let top_left = ('a'..='g').find(|c| char_counts[c] == 6).unwrap();
    let bottom_or_center1 = ('a'..='g').find(|c| char_counts[c] == 7).unwrap();
    let bottom_or_center2 = ('a'..='g').filter(|c| char_counts[c] == 7).nth(1).unwrap();
    let decode_pattern = |s: &str| match s.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 if s.contains(bottom_left) => 2,
        5 if s.contains(top_left) => 5,
        5 => 3,
        6 if !s.contains(bottom_left) => 9,
        6 if s.contains(bottom_or_center1) && s.contains(bottom_or_center2) => 6,
        6 => 0,
        7 => 8,
        _ => unreachable!(),
    };
    patterns
        .into_iter()
        .map(|pattern| {
            let value = decode_pattern(&pattern);
            (pattern, value)
        })
        .collect()
}

fn count_chars(patterns: &[String]) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    patterns
        .iter()
        .flat_map(|s| s.chars())
        .for_each(|c| *result.entry(c).or_default() += 1);
    result
}

fn apply_legend(legend: &HashMap<String, usize>, digits: &[&str]) -> usize {
    digits
        .iter()
        .map(|&s| legend[&sort_str(s)])
        .fold(0, |acc, digit| 10 * acc + digit)
}

fn sort_str(s: &str) -> String {
    let mut bytes = Vec::from_iter(s.bytes());
    bytes.sort();
    String::from_utf8(bytes).unwrap()
}

use crate::harness::input::RawInput;
use std::collections::HashMap;

pub fn solve_part1(input: RawInput) -> u64 {
    solve(input, 10)
}

pub fn solve_part2(input: RawInput) -> u64 {
    solve(input, 40)
}

type PairMap<T> = HashMap<(char, char), T>;

#[derive(Debug)]
struct Input {
    template: Vec<char>,
    rules: PairMap<char>,
}

fn solve(input: RawInput, n_steps: usize) -> u64 {
    let Input { template, rules } = parse_input(input);
    let mut pair_counts = count_pairs(&template);
    for _ in 0..n_steps {
        pair_counts = apply_step(&pair_counts, &rules);
    }
    score_final_counts(&pair_counts)
}

fn parse_input(input: RawInput) -> Input {
    let mut lines = input.lines();
    let template = lines.next().unwrap().chars().collect();
    lines.next().unwrap();
    let rules = lines
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            ((chars[0], chars[1]), chars[6])
        })
        .collect();
    Input { template, rules }
}

fn count_pairs(chars: &[char]) -> PairMap<u64> {
    let mut result = HashMap::new();
    for i in 0..chars.len() - 1 {
        *result.entry((chars[i], chars[i + 1])).or_default() += 1;
    }
    result
}

fn apply_step(pair_counts: &PairMap<u64>, rules: &PairMap<char>) -> PairMap<u64> {
    let mut result = HashMap::new();
    for (&pair, &count) in pair_counts {
        let (first, second) = pair;
        let inserted = rules[&pair];
        *result.entry((first, inserted)).or_default() += count;
        *result.entry((inserted, second)).or_default() += count;
    }
    result
}

fn score_final_counts(pair_counts: &PairMap<u64>) -> u64 {
    let mut char_counts = HashMap::<char, u64>::new();
    for (&(first, second), &pair_count) in pair_counts {
        *char_counts.entry(first).or_default() += pair_count;
        *char_counts.entry(second).or_default() += pair_count;
    }
    // Counting characters appearing in pairs double-counts all characters
    // except for the first and last, so divide by 2 and round up.
    let min = (*char_counts.values().min().unwrap() + 1) / 2;
    let max = (*char_counts.values().max().unwrap() + 1) / 2;
    max - min
}

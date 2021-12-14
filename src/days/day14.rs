use crate::harness::input::RawInput;
use ndarray::{Array1, Array2};
use std::collections::HashMap;

pub fn solve_part1(input: RawInput) -> u64 {
    solve(input, 10)
}

pub fn solve_part2(input: RawInput) -> u64 {
    solve(input, 40)
}

#[derive(Debug)]
struct Input {
    template: Vec<char>,
    rules: Vec<((char, char), char)>,
}

fn solve(input: RawInput, n_steps: usize) -> u64 {
    let Input { template, rules } = parse_input(input);
    let index_by_pair = make_index_by_pair_map(&rules);
    let transition_matrix = build_transition_matrix(&rules, &index_by_pair);
    let initial_state = build_initial_state(&template, &index_by_pair);
    let full_transform_matrix = matrix_pow(transition_matrix, n_steps);
    let final_state = full_transform_matrix.dot(&initial_state);
    score_final_state(&final_state, &template, &rules)
}

fn parse_input(input: RawInput) -> Input {
    let mut lines = input.lines();
    let template = lines.next().unwrap().chars().collect();
    lines.next().unwrap();
    let rules = lines
        .map(|line| {
            let bytes = line.chars().collect::<Vec<_>>();
            ((bytes[0], bytes[1]), bytes[6])
        })
        .collect();
    Input { template, rules }
}

fn make_index_by_pair_map(rules: &[((char, char), char)]) -> HashMap<(char, char), usize> {
    rules
        .iter()
        .enumerate()
        .map(|(i, &(pair, _))| (pair, i))
        .collect()
}

fn build_transition_matrix(
    rules: &[((char, char), char)],
    index_by_pair: &HashMap<(char, char), usize>,
) -> Array2<u64> {
    let mut result = Array2::zeros((rules.len(), rules.len()));
    for (i, &((first, second), inserted)) in rules.iter().enumerate() {
        let first_output_index = index_by_pair[&(first, inserted)];
        let second_output_index = index_by_pair[&(inserted, second)];
        result[[first_output_index, i]] = 1;
        result[[second_output_index, i]] += 1;
    }
    result
}

fn build_initial_state(
    template: &[char],
    index_by_pair: &HashMap<(char, char), usize>,
) -> Array1<u64> {
    let mut result = Array1::zeros((index_by_pair.len(),));
    for i in 0..template.len() - 1 {
        result[index_by_pair[&(template[i], template[i + 1])]] += 1
    }
    result
}

fn matrix_pow(mut matrix: Array2<u64>, mut n: usize) -> Array2<u64> {
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

fn score_final_state(
    state: &Array1<u64>,
    template: &[char],
    rules: &[((char, char), char)],
) -> u64 {
    let mut char_counts = HashMap::<char, u64>::new();
    for (i, &((first, second), _)) in rules.iter().enumerate() {
        let pair_count = state[[i]];
        if pair_count > 0 {
            *char_counts.entry(first).or_default() += pair_count;
            *char_counts.entry(second).or_default() += pair_count;
        }
    }
    // Counting characters appearing in pairs double-counts all characters
    // except for the first and last, which don't change when applying a step.
    *char_counts.entry(template[0]).or_default() += 1;
    *char_counts.entry(template[template.len() - 1]).or_default() += 1;
    let min = *char_counts.values().min().unwrap() / 2;
    let max = *char_counts.values().max().unwrap() / 2;
    max - min
}

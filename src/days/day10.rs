use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    let lines = input.per_line(|line| line.chars());
    lines
        .into_iter()
        .filter_map(|line| {
            if let Flaw::Corruption(c) = get_flaw(&line) {
                Some(c)
            } else {
                None
            }
        })
        .map(score_corruption)
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let lines = input.per_line(|line| line.chars());
    let mut scores = lines
        .into_iter()
        .filter_map(|line| {
            if let Flaw::Incomplete(chars) = get_flaw(&line) {
                Some(chars)
            } else {
                None
            }
        })
        .map(|chars| score_completion(&chars))
        .collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2]
}

#[derive(Debug)]
enum Flaw {
    Corruption(char),
    Incomplete(Vec<char>),
}

fn get_flaw(line: &[char]) -> Flaw {
    let mut closers = Vec::<char>::new();
    for &c in line {
        match c {
            '(' => closers.push(')'),
            '[' => closers.push(']'),
            '{' => closers.push('}'),
            '<' => closers.push('>'),
            _ => {
                if closers.pop() != Some(c) {
                    return Flaw::Corruption(c);
                }
            }
        }
    }
    closers.reverse();
    Flaw::Incomplete(closers)
}

fn score_corruption(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn score_completion(chars: &[char]) -> usize {
    chars
        .iter()
        .fold(0, |acc, &c| 5 * acc + ")]}>".find(c).unwrap() + 1)
}

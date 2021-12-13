use crate::harness::input::{LineInput, RawInput};
use crate::regex;
use crate::util::re;
use std::collections::HashSet;

pub fn solve_part1(input: RawInput) -> usize {
    let Input { mut dots, folds } = parse_input(input);
    apply_fold(&mut dots, folds[0]);
    dots.len()
}

pub fn solve_part2(input: RawInput) -> String {
    let Input { mut dots, folds } = parse_input(input);
    for fold in folds {
        apply_fold(&mut dots, fold);
    }
    print_dots(&dots);
    "".to_owned()
}

#[derive(Debug)]
struct Input {
    dots: HashSet<(u32, u32)>,
    folds: Vec<Fold>,
}

#[derive(Copy, Clone, Debug)]
struct Fold {
    value: u32,
    is_x: bool,
}

fn parse_input(input: RawInput) -> Input {
    let groups = input.grouped_lines(|line| line.single::<String>());
    let dots = groups[0]
        .iter()
        .map(|line| LineInput::new(line).split(","))
        .map(|parts| (parts[0], parts[1]))
        .collect();
    let folds = groups[1]
        .iter()
        .map(|line| {
            re::parse_with_regex::<(char, u32)>(regex!(r"^fold along (.)=(\d+)$"), line).unwrap()
        })
        .map(|(c, value)| Fold {
            value,
            is_x: c == 'x',
        })
        .collect();
    Input { dots, folds }
}

fn apply_fold(dots: &mut HashSet<(u32, u32)>, Fold { value, is_x }: Fold) {
    if is_x {
        fold_x(dots, value);
    } else {
        fold_y(dots, value);
    }
}

fn fold_x(dots: &mut HashSet<(u32, u32)>, x: u32) {
    let folded_dots = dots
        .iter()
        .filter(|&dot| dot.0 > x)
        .map(|&dot| dot)
        .collect::<Vec<_>>();
    for dot in folded_dots {
        dots.remove(&dot);
        dots.insert((2 * x - dot.0, dot.1));
    }
}

fn fold_y(dots: &mut HashSet<(u32, u32)>, y: u32) {
    let folded_dots = dots
        .iter()
        .filter(|&dot| dot.1 > y)
        .map(|&dot| dot)
        .collect::<Vec<_>>();
    for dot in folded_dots {
        dots.remove(&dot);
        dots.insert((dot.0, 2 * y - dot.1));
    }
}

fn print_dots(dots: &HashSet<(u32, u32)>) {
    let max_x = dots.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = dots.iter().map(|&(_, y)| y).max().unwrap();
    for y in 0..=max_y {
        let s = (0..=max_x)
            .map(|x| if dots.contains(&(x, y)) { '█' } else { ' ' })
            .collect::<String>();
        println!("{}", s);
    }
}

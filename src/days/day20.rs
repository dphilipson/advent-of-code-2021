use crate::harness::input::RawInput;
use std::cmp;
use std::collections::HashSet;

pub fn solve_part1(input: RawInput) -> usize {
    todo!();
    solve(input, 2)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 50)
}

fn solve(input: RawInput, steps: usize) -> usize {
    let (algo, light_pixels) = parse_input(input);
    let mut state = State::new(&algo, light_pixels);
    for _ in 0..steps {
        state = apply_step(&state);
    }
    state.light_pixels.len()
}

fn parse_input(input: RawInput) -> ([bool; 512], HashSet<(i32, i32)>) {
    let groups =
        input.grouped_lines(|line| line.as_str().bytes().map(|b| b == b'#').collect::<Vec<_>>());
    let mut algo = [false; 512];
    for i in 0..512 {
        algo[i] = groups[0][0][i];
    }
    let mut light_pixels = HashSet::new();
    for i in 0..groups[1].len() {
        for j in 0..groups[1][0].len() {
            if groups[1][i][j] {
                light_pixels.insert((i as i32, j as i32));
            }
        }
    }
    (algo, light_pixels)
}

#[derive(Debug)]
struct State<'a> {
    algo: &'a [bool; 512],
    light_pixels: HashSet<(i32, i32)>,
    is_outside_light: bool,
    i_bounds: (i32, i32),
    j_bounds: (i32, i32),
}

impl<'a> State<'a> {
    fn new(algo: &'a [bool; 512], light_pixels: HashSet<(i32, i32)>) -> Self {
        let (min_i, max_i, min_j, max_j) = get_bounds(&light_pixels);
        Self {
            algo,
            light_pixels,
            is_outside_light: false,
            i_bounds: (min_i, max_i),
            j_bounds: (min_j, max_j),
        }
    }
}

fn apply_step<'a>(state: &State<'a>) -> State<'a> {
    let State {
        algo,
        is_outside_light,
        i_bounds: (min_i, max_i),
        j_bounds: (min_j, max_j),
        ..
    } = state;
    let mut new_light_pixels = HashSet::new();
    for i in min_i - 1..=max_i + 1 {
        for j in min_j - 1..=max_j + 1 {
            if evaluate_pixel(state, i, j) {
                new_light_pixels.insert((i, j));
            }
        }
    }
    State {
        algo,
        light_pixels: new_light_pixels,
        is_outside_light: algo[0] && !*is_outside_light,
        i_bounds: (min_i - 1, max_i + 1),
        j_bounds: (min_j - 1, max_j + 1),
    }
}

fn get_bounds(pixels: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
    let mut min_i = 0;
    let mut max_i = 0;
    let mut min_j = 0;
    let mut max_j = 0;
    for &(i, j) in pixels {
        min_i = cmp::min(min_i, i);
        max_i = cmp::max(max_i, i);
        min_j = cmp::min(min_j, j);
        max_j = cmp::max(max_j, j);
    }
    (min_i, max_i, min_j, max_j)
}

fn evaluate_pixel(state: &State, i: i32, j: i32) -> bool {
    let State {
        algo,
        light_pixels,
        is_outside_light,
        i_bounds: (min_i, max_i),
        j_bounds: (min_j, max_j),
    } = state;
    let i_bounds = *min_i..=*max_i;
    let j_bounds = *min_j..=*max_j;
    let mut index = 0;
    for i2 in i - 1..=i + 1 {
        for j2 in j - 1..=j + 1 {
            let is_light = (*is_outside_light
                && !(i_bounds.contains(&i2) && j_bounds.contains(&j2)))
                || light_pixels.contains(&(i2, j2));
            index = 2 * index + is_light as usize;
        }
    }
    algo[index]
}

use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re::parse_with_regex;
use ndarray::Array3;
use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;
use std::{cmp, error};

pub fn solve_part1(input: RawInput) -> i64 {
    solve(input, true)
}

pub fn solve_part2(input: RawInput) -> i64 {
    solve(input, false)
}

pub fn solve(input: RawInput, trim_region: bool) -> i64 {
    let mut steps = input.per_line(|line| line.single::<Step>());
    if trim_region {
        steps = steps
            .into_iter()
            .map(|step| step.trim_to_initialization_region())
            .collect();
    }
    let [x_breakpoints, y_breakpoints, z_breakpoints] = get_sorted_breakpoints(&steps);
    let x_indices_by_breakpoint = get_indices_by_breakpoints(&x_breakpoints);
    let y_indices_by_breakpoint = get_indices_by_breakpoints(&y_breakpoints);
    let z_indices_by_breakpoint = get_indices_by_breakpoints(&z_breakpoints);
    let mut grid = Array3::default((
        x_breakpoints.len() - 1,
        y_breakpoints.len() - 1,
        z_breakpoints.len() - 1,
    ));
    for step in steps {
        let x_min_index = x_indices_by_breakpoint[&step.x_min];
        let x_max_index = x_indices_by_breakpoint[&(step.x_max + 1)];
        let y_min_index = y_indices_by_breakpoint[&step.y_min];
        let y_max_index = y_indices_by_breakpoint[&(step.y_max + 1)];
        let z_min_index = z_indices_by_breakpoint[&step.z_min];
        let z_max_index = z_indices_by_breakpoint[&(step.z_max + 1)];
        for x in x_min_index..x_max_index {
            for y in y_min_index..y_max_index {
                for z in z_min_index..z_max_index {
                    grid[[x, y, z]] = step.is_on;
                }
            }
        }
    }
    let mut total = 0;
    for x in 0..x_breakpoints.len() - 1 {
        for y in 0..y_breakpoints.len() - 1 {
            for z in 0..z_breakpoints.len() - 1 {
                if grid[[x, y, z]] {
                    total += (x_breakpoints[x + 1] - x_breakpoints[x])
                        * (y_breakpoints[y + 1] - y_breakpoints[y])
                        * (z_breakpoints[z + 1] - z_breakpoints[z]);
                }
            }
        }
    }
    total
}

#[derive(Copy, Clone, Debug)]
struct Step {
    is_on: bool,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl FromStr for Step {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"^(.+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$");
        let result: (String, i64, i64, i64, i64, i64, i64) = parse_with_regex(re, s)?;
        Ok(Step {
            is_on: &result.0 == "on",
            x_min: result.1,
            x_max: result.2,
            y_min: result.3,
            y_max: result.4,
            z_min: result.5,
            z_max: result.6,
        })
    }
}

impl Step {
    fn trim_to_initialization_region(self) -> Self {
        Self {
            is_on: self.is_on,
            x_min: cmp::max(-50, self.x_min),
            x_max: cmp::min(50, self.x_max),
            y_min: cmp::max(-50, self.y_min),
            y_max: cmp::min(50, self.y_max),
            z_min: cmp::max(-50, self.z_min),
            z_max: cmp::min(50, self.z_max),
        }
    }
}

fn get_sorted_breakpoints(steps: &[Step]) -> [Vec<i64>; 3] {
    let mut x_breakpoints = BTreeSet::new();
    let mut y_breakpoints = BTreeSet::new();
    let mut z_breakpoints = BTreeSet::new();
    for step in steps {
        x_breakpoints.extend([step.x_min, step.x_max + 1]);
        y_breakpoints.extend([step.y_min, step.y_max + 1]);
        z_breakpoints.extend([step.z_min, step.z_max + 1]);
    }
    [
        Vec::from_iter(x_breakpoints),
        Vec::from_iter(y_breakpoints),
        Vec::from_iter(z_breakpoints),
    ]
}

fn get_indices_by_breakpoints(breakpoints: &[i64]) -> HashMap<i64, usize> {
    breakpoints
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect()
}

use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re::parse_with_regex;
use ndarray::Array3;
use std::collections::HashMap;
use std::str::FromStr;
use std::{cmp, error};

pub fn solve_part1(input: RawInput) -> usize {
    let instructions = input.per_line(|line| line.single::<Instruction>());
    let mut cubes = Array3::default((101, 101, 101));
    for instruction in instructions {
        apply_instruction(&mut cubes, instruction);
    }
    cubes.iter().filter(|&&b| b).count()
}

pub fn solve_part2(input: RawInput) -> i64 {
    let instructions = input.per_line(|line| line.single::<Instruction>());
    let mut x_breakpoints = Vec::with_capacity(instructions.len() * 2);
    let mut y_breakpoints = Vec::with_capacity(instructions.len() * 2);
    let mut z_breakpoints = Vec::with_capacity(instructions.len() * 2);
    for instruction in &instructions {
        x_breakpoints.extend([instruction.x_min, instruction.x_max + 1]);
        y_breakpoints.extend([instruction.y_min, instruction.y_max + 1]);
        z_breakpoints.extend([instruction.z_min, instruction.z_max + 1]);
    }
    x_breakpoints.sort();
    y_breakpoints.sort();
    z_breakpoints.sort();
    let x_indices_by_breakpoint = x_breakpoints
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i))
        .collect::<HashMap<_, _>>();
    let y_indices_by_breakpoint = y_breakpoints
        .iter()
        .enumerate()
        .map(|(i, &y)| (y, i))
        .collect::<HashMap<_, _>>();
    let z_indices_by_breakpoint = z_breakpoints
        .iter()
        .enumerate()
        .map(|(i, &z)| (z, i))
        .collect::<HashMap<_, _>>();
    let mut grid = Array3::default((
        x_breakpoints.len() - 1,
        y_breakpoints.len() - 1,
        z_breakpoints.len() - 1,
    ));
    for instruction in instructions {
        let x_min_index = x_indices_by_breakpoint[&instruction.x_min];
        let x_max_index = x_indices_by_breakpoint[&(instruction.x_max + 1)];
        let y_min_index = y_indices_by_breakpoint[&instruction.y_min];
        let y_max_index = y_indices_by_breakpoint[&(instruction.y_max + 1)];
        let z_min_index = z_indices_by_breakpoint[&instruction.z_min];
        let z_max_index = z_indices_by_breakpoint[&(instruction.z_max + 1)];
        for x in x_min_index..x_max_index {
            for y in y_min_index..y_max_index {
                for z in z_min_index..z_max_index {
                    grid[[x, y, z]] = instruction.is_on;
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
struct Instruction {
    is_on: bool,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

impl FromStr for Instruction {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"^(.+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$");
        let result: (String, i64, i64, i64, i64, i64, i64) = parse_with_regex(re, s)?;
        Ok(Instruction {
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

fn apply_instruction(grid: &mut Array3<bool>, instruction: Instruction) {
    let x_min = cmp::max(instruction.x_min, -50) + 50;
    let x_max = cmp::min(instruction.x_max, 50) + 50;
    let y_min = cmp::max(instruction.y_min, -50) + 50;
    let y_max = cmp::min(instruction.y_max, 50) + 50;
    let z_min = cmp::max(instruction.z_min, -50) + 50;
    let z_max = cmp::min(instruction.z_max, 50) + 50;
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                grid[[x as usize, y as usize, z as usize]] = instruction.is_on;
            }
        }
    }
}

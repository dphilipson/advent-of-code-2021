use crate::harness::input::RawInput;
use crate::regex;
use crate::util::re;
use std::error;
use std::str::FromStr;

pub fn solve_part1(input: RawInput) -> i32 {
    let target = input.single_line(|line| line.single::<Target>());
    let vels = get_successful_vels(target);
    let max_vy = vels.into_iter().map(|(_, vy)| vy).max().unwrap();
    max_height(max_vy)
}

pub fn solve_part2(input: RawInput) -> usize {
    let target = input.single_line(|line| line.single::<Target>());
    let vels = get_successful_vels(target);
    vels.len()
}

#[derive(Copy, Clone, Debug)]
struct Target {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl FromStr for Target {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = regex!(r"^target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)$");
        let (x_min, x_max, y_min, y_max) = re::parse_with_regex::<(i32, i32, i32, i32)>(regex, s)?;
        Ok(Target {
            x_min,
            x_max,
            y_min,
            y_max,
        })
    }
}

impl Target {
    fn contains(self, x: i32, y: i32) -> bool {
        (self.x_min..=self.x_max).contains(&x) && (self.y_min..=self.y_max).contains(&y)
    }
}

fn get_successful_vels(target: Target) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for vx in 1..=target.x_max {
        for vy in target.y_min..=-target.y_min {
            if does_vel_hit_target(vx, vy, target) {
                result.push((vx, vy));
            }
        }
    }
    result
}

fn does_vel_hit_target(mut vx: i32, mut vy: i32, target: Target) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x <= target.x_max && y >= target.y_min {
        if target.contains(x, y) {
            return true;
        }
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
    }
    false
}

fn max_height(vy: i32) -> i32 {
    vy * (vy + 1) / 2
}

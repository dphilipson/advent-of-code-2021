use crate::harness::input::{LineInput, RawInput};
use crate::util::coords::Coord3;
use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: RawInput) -> usize {
    let scanners = parse_scanners(input);
    solve(&scanners).beacons.len()
}

pub fn solve_part2(input: RawInput) -> i32 {
    let scanners = parse_scanners(input);
    let solved_scanners = solve(&scanners).scanners;
    let mut best_distance = 0;
    for scanner1 in &solved_scanners {
        for scanner2 in &solved_scanners {
            best_distance = cmp::max(
                best_distance,
                (scanner1.location - scanner2.location).manhattan_norm(),
            );
        }
    }
    best_distance
}

type Point = Coord3<i32>;

fn parse_scanners(input: RawInput) -> Vec<Vec<Point>> {
    input
        .as_str()
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .skip(1)
                .map(|line| {
                    let parts = LineInput::new(line).split::<i32>(",");
                    Coord3(parts[0], parts[1], parts[2])
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
struct Solution {
    beacons: HashSet<Point>,
    scanners: Vec<SolvedScanner>,
}

#[derive(Debug)]
struct SolvedScanner {
    location: Point,
    orientation: Orientation,
    absolute_beacons: Vec<Point>,
}

fn solve(beacons_by_scanner: &[Vec<Point>]) -> Solution {
    let displacements_by_beacon_by_scanner = beacons_by_scanner
        .iter()
        .map(|beacons| get_beacon_displacements(beacons))
        .collect::<Vec<_>>();
    let mut solved_scanners_by_index = HashMap::<usize, SolvedScanner>::new();
    let mut unsolved_scanners_by_index = beacons_by_scanner
        .iter()
        .map(|beacons| beacons.clone())
        .enumerate()
        .collect::<HashMap<_, _>>();
    let mut seen_pairs = HashSet::new();
    solved_scanners_by_index.insert(
        0,
        SolvedScanner {
            location: Point::default(),
            orientation: Orientation::default(),
            absolute_beacons: beacons_by_scanner[0].clone(),
        },
    );
    while unsolved_scanners_by_index.len() > 0 {
        let (i, solved_scanner, new_seen_pairs) = solve_next_scanner(
            &displacements_by_beacon_by_scanner,
            &solved_scanners_by_index,
            &unsolved_scanners_by_index,
            &seen_pairs,
        );
        seen_pairs.extend(new_seen_pairs);
        unsolved_scanners_by_index.remove(&i);
        solved_scanners_by_index.insert(i, solved_scanner);
    }
    let beacons = solved_scanners_by_index
        .values()
        .flat_map(|scanner| scanner.absolute_beacons.iter().map(|&b| b))
        .collect::<HashSet<Point>>();
    let mut scanners = Vec::with_capacity(solved_scanners_by_index.len());
    for i in 0..solved_scanners_by_index.len() {
        scanners.push(solved_scanners_by_index.remove(&i).unwrap());
    }
    Solution { beacons, scanners }
}

fn solve_next_scanner(
    displacements_by_beacon_by_scanner: &[Vec<HashSet<Point>>],
    solved_scanners_by_index: &HashMap<usize, SolvedScanner>,
    unsolved_scanners_by_index: &HashMap<usize, Vec<Point>>,
    seen_pairs: &HashSet<(usize, usize)>,
) -> (usize, SolvedScanner, Vec<(usize, usize)>) {
    let mut new_seen_pairs = vec![];
    for (&i, solved_scanner) in solved_scanners_by_index {
        let solved_displacements = &displacements_by_beacon_by_scanner[i];
        for (&j, unsolved_scanner) in unsolved_scanners_by_index {
            if seen_pairs.contains(&(i, j)) {
                continue;
            }
            new_seen_pairs.push((i, j));
            let unsolved_displacements = &displacements_by_beacon_by_scanner[j];
            if let Some(new_solved_scanner) = try_overlapping_scanner_pair(
                solved_scanner,
                solved_displacements,
                unsolved_scanner,
                unsolved_displacements,
            ) {
                return (j, new_solved_scanner, new_seen_pairs);
            }
        }
    }
    panic!("No overlap found.");
}

fn try_overlapping_scanner_pair(
    solved_scanner: &SolvedScanner,
    solved_displacements_by_beacon: &[HashSet<Point>],
    new_beacons: &[Point],
    new_displacements_by_beacon: &[HashSet<Point>],
) -> Option<SolvedScanner> {
    let solved_beacons = &solved_scanner.absolute_beacons;
    let mut corresponding_beacons = Vec::<(Point, Point)>::new();
    for (i, solved_displacements) in solved_displacements_by_beacon.iter().enumerate() {
        for (j, new_displacements) in new_displacements_by_beacon.iter().enumerate() {
            if solved_displacements.intersection(new_displacements).count() > 11 {
                corresponding_beacons.push((solved_beacons[i], new_beacons[j]));
            }
        }
    }
    if corresponding_beacons.len() < 12 {
        return None;
    }
    for orientation in Orientation::all() {
        let get_displacement = |(solved, new): (Point, Point)| solved - orientation.apply(new);
        let target_displacement = get_displacement(corresponding_beacons[0]);
        if corresponding_beacons[1..]
            .iter()
            .all(|&pair| get_displacement(pair) == target_displacement)
        {
            let location = target_displacement;
            let absolute_beacons = new_beacons
                .iter()
                .map(|&beacon| orientation.apply(beacon) + location)
                .collect();
            return Some(SolvedScanner {
                location,
                orientation,
                absolute_beacons,
            });
        }
    }
    None
}

fn get_beacon_displacements(beacons: &[Point]) -> Vec<HashSet<Point>> {
    beacons
        .iter()
        .map(|&beacon| {
            beacons
                .iter()
                .map(|&other| {
                    let Coord3(x, y, z) = other - beacon;
                    let mut coords = [x.abs(), y.abs(), z.abs()];
                    coords.sort();
                    Coord3(coords[0], coords[1], coords[2])
                })
                .collect()
        })
        .collect()
}

#[derive(Copy, Clone, Debug)]
struct Orientation(i32, i32, i32);

impl Orientation {
    fn apply(self, Coord3(x, y, z): Point) -> Point {
        let Self(o0, o1, o2) = self;
        let parts = [0, x, y, z];
        Coord3(
            o0.signum() * parts[o0.abs() as usize],
            o1.signum() * parts[o1.abs() as usize],
            o2.signum() * parts[o2.abs() as usize],
        )
    }

    fn all() -> Vec<Self> {
        let mut result = Vec::with_capacity(24);
        for (x1, y1, z1) in [
            (1, 2, 3),
            (-1, 3, 2),
            (2, 3, 1),
            (-2, 1, 3),
            (3, 1, 2),
            (-3, 2, 1),
        ] {
            for (x2, y2, z2) in [(1, 1, 1), (1, -1, -1), (-1, 1, -1), (-1, -1, 1)] {
                result.push(Orientation(x1 * x2, y1 * y2, z1 * z2))
            }
        }
        result
    }
}

impl Default for Orientation {
    fn default() -> Self {
        Self(0, 1, 2)
    }
}

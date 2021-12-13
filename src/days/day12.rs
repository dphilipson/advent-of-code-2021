use crate::harness::input::RawInput;
use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: RawInput) -> usize {
    count_routes(input, false)
}

pub fn solve_part2(input: RawInput) -> usize {
    count_routes(input, true)
}

#[derive(Debug)]
enum PendingStep<'a> {
    Explore(&'a str),
    Backtrack {
        room: &'a str,
        is_double_visit: bool,
    },
}

fn count_routes(input: RawInput, can_double_visit: bool) -> usize {
    let paths = parse_paths(&input);
    let mut seen = HashSet::<&str>::new();
    let mut has_double_visited = false;
    let mut pending = vec![PendingStep::Explore("start")];
    let mut count = 0;
    while let Some(step) = pending.pop() {
        match step {
            PendingStep::Explore(room) => {
                if room == "end" {
                    count += 1;
                    continue;
                }
                if is_small(room) {
                    let is_double_visit = seen.contains(room);
                    if is_double_visit {
                        has_double_visited = true;
                    } else {
                        seen.insert(room);
                    }
                    pending.push(PendingStep::Backtrack {
                        room,
                        is_double_visit,
                    });
                }
                pending.extend(
                    paths
                        .get(room)
                        .unwrap_or(&vec![])
                        .iter()
                        .filter(|&next_room| {
                            (can_double_visit && !has_double_visited && *next_room != "start")
                                || !seen.contains(next_room)
                        })
                        .map(|&next_room| PendingStep::Explore(next_room)),
                );
            }
            PendingStep::Backtrack {
                room,
                is_double_visit,
            } => {
                if is_double_visit {
                    has_double_visited = false;
                } else {
                    seen.remove(room);
                }
            }
        }
    }
    count
}

fn parse_paths<'a>(input: &'a RawInput) -> HashMap<&'a str, Vec<&'a str>> {
    let paths = input
        .lines()
        .map(|line| line.split('-').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut result = HashMap::<&'a str, Vec<&'a str>>::new();
    let mut insert_path = |a: &'a str, b: &'a str| {
        if a != "end" {
            result.entry(a).or_default().push(b);
        }
    };
    for endpoints in paths {
        insert_path(&endpoints[0], &endpoints[1]);
        insert_path(&endpoints[1], &endpoints[0]);
    }
    result
}

fn is_small(room: &str) -> bool {
    room.bytes().all(|b| (b'a'..=b'z').contains(&b))
}

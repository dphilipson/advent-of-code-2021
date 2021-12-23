use crate::harness::input::RawInput;
use crate::util::search::{dijkstra, Distance};

pub fn solve_part1(input: RawInput) -> usize {
    let rooms = parse_rooms(input);
    solve(rooms)
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut rooms = parse_rooms(input);
    rooms[0].splice(1..1, [3, 3]);
    rooms[1].splice(1..1, [1, 2]);
    rooms[2].splice(1..1, [0, 1]);
    rooms[3].splice(1..1, [2, 0]);
    solve(rooms)
}

fn parse_rooms(input: RawInput) -> [Vec<Frog>; 4] {
    let bytes = input.per_line(|line| line.bytes());
    let read_room = |i: usize| {
        vec![
            (bytes[3][3 + 2 * i] - b'A') as usize,
            (bytes[2][3 + 2 * i] - b'A') as usize,
        ]
    };
    [read_room(0), read_room(1), read_room(2), read_room(3)]
}

fn solve(rooms: [Vec<usize>; 4]) -> usize {
    let initial_state = State::new(rooms);
    dijkstra::search(initial_state, State::get_next_states, State::is_goal_state)
        .goal_state()
        .unwrap()
        .distance
}

type Frog = usize;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    hallway: [Option<Frog>; 11],
    rooms: [Vec<Frog>; 4],
    room_size: usize,
}

impl State {
    fn new(rooms: [Vec<Frog>; 4]) -> Self {
        let room_size = rooms[0].len();
        Self {
            hallway: [None; 11],
            rooms,
            room_size,
        }
    }

    fn get_next_states(&self) -> Vec<(Self, Distance)> {
        let mut result = vec![];
        for position in 0..11 {
            if let Some(next_state) = self.get_move_from_hallway(position) {
                result.push(next_state);
            }
        }
        for room_index in 0..4 {
            result.extend(self.get_moves_from_room(room_index));
        }
        result
    }

    fn is_goal_state(&self) -> bool {
        (0..4).all(|frog| self.can_enter_room(frog))
            && self.rooms.iter().all(|room| room.len() == self.room_size)
    }

    fn get_move_from_hallway(&self, position: usize) -> Option<(State, Distance)> {
        let frog = self.hallway[position]?;
        if !self.can_enter_room(frog) {
            return None;
        }
        let target = 2 * frog + 2;
        if !self.is_hallway_clear(position, target) {
            return None;
        }
        let distance = abs_diff(position, target) + self.room_size - self.rooms[frog].len();
        let mut next_state = self.clone();
        next_state.hallway[position] = None;
        next_state.rooms[frog].push(frog);
        Some((next_state, energy_per_move(frog) * distance))
    }

    fn get_moves_from_room(&self, room_index: usize) -> Vec<(Self, Distance)> {
        if let Some(&frog) = self.rooms[room_index].last() {
            if frog == room_index && self.can_enter_room(frog) {
                return vec![];
            }
            let start = 2 * room_index + 2;
            let min_hallway = (0..=start)
                .rev()
                .take_while(|&i| self.hallway[i].is_none())
                .last()
                .unwrap();
            let max_hallway = (start..11)
                .take_while(|&i| self.hallway[i].is_none())
                .last()
                .unwrap();
            (min_hallway..=max_hallway)
                .filter(|&i| !is_banned_hallway(i))
                .map(|i| {
                    let distance =
                        abs_diff(start, i) + self.room_size - self.rooms[room_index].len() + 1;
                    let mut next_state = self.clone();
                    next_state.hallway[i] = Some(frog);
                    next_state.rooms[room_index].pop();
                    (next_state, energy_per_move(frog) * distance)
                })
                .collect()
        } else {
            vec![]
        }
    }

    fn is_hallway_clear(&self, start: usize, end: usize) -> bool {
        let mut range = if start < end {
            start + 1..end + 1
        } else {
            end..start
        };
        range.all(|i| self.hallway[i].is_none())
    }

    fn can_enter_room(&self, frog: Frog) -> bool {
        self.rooms[frog].iter().all(|&room_frog| room_frog == frog)
    }
}

fn is_banned_hallway(i: usize) -> bool {
    match i {
        2 | 4 | 6 | 8 => true,
        _ => false,
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}

fn energy_per_move(frog: Frog) -> usize {
    let mut result = 1;
    for _ in 0..frog {
        result *= 10;
    }
    result
}

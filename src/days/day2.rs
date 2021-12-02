use crate::harness::input::RawInput;
use crate::util::coords::Coord2;
use crate::{regex, string_enum};

pub fn solve_part1(input: RawInput) -> i32 {
    let moves = parse_moves(input);
    let Coord2(x, y) = moves
        .iter()
        .map(|&(direction, step)| direction.coord() * step)
        .sum();
    x * y
}

pub fn solve_part2(input: RawInput) -> i32 {
    let moves = parse_moves(input);
    let mut state = State::default();
    for (direction, step) in moves {
        state = state.update(direction, step);
    }
    state.position.0 * state.position.1
}

fn parse_moves(input: RawInput) -> Vec<(Direction, i32)> {
    let re = regex!(r"^(\S+) (\d+)$");
    input.per_line(|line| line.parse_with_regex::<(Direction, i32)>(re))
}

string_enum!(Direction {
    Forward = "forward",
    Down = "down",
    Up = "up",
});

impl Direction {
    fn coord(self) -> Coord2<i32> {
        match self {
            Direction::Forward => Coord2(1, 0),
            Direction::Down => Coord2(0, 1),
            Direction::Up => Coord2(0, -1),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct State {
    position: Coord2<i32>,
    aim: i32,
}

impl State {
    fn update(self, direction: Direction, step: i32) -> State {
        let State { position, aim } = self;
        match direction {
            Direction::Forward => State {
                position: position + Coord2(1, aim) * step,
                aim,
            },
            Direction::Down => State {
                position,
                aim: aim + step,
            },
            Direction::Up => State {
                position,
                aim: aim - step,
            },
        }
    }
}

use crate::harness::input::RawInput;
use ndarray::Array2;
use std::cmp;

pub fn solve_part1(input: RawInput) -> u32 {
    let (mut a_pos, mut b_pos) = parse_start_positions(input);
    let mut a_score = 0;
    let mut b_score = 0;
    let mut roll_count = 0;
    for i in (6..).step_by(18) {
        a_pos = ((a_pos - 1 + i) % 10) + 1;
        roll_count += 3;
        a_score += a_pos as u32;
        if a_score >= 1000 {
            return b_score * roll_count;
        }
        b_pos = ((b_pos - 1 + i + 9) % 10) + 1;
        roll_count += 3;
        b_score += b_pos as u32;
        if b_score >= 1000 {
            return a_score * roll_count;
        }
    }
    unreachable!()
}

pub fn solve_part2(input: RawInput) -> u64 {
    let (a_pos, b_pos) = parse_start_positions(input);
    let mut a_universes = get_intiial_universes(a_pos);
    let mut b_universes = get_intiial_universes(b_pos);
    let mut a_universe_count = 1;
    let mut b_universe_count = 1;
    let mut a_win_count = 0;
    let mut b_win_count = 0;
    let mut a_finish_count = 0;
    let mut b_finish_count = 0;
    while a_universes.iter().any(|&count| count > 0) || b_universes.iter().any(|&count| count > 0) {
        let (new_a_universes, new_a_finish_count) = update_universes(&a_universes);
        a_universes = new_a_universes;
        a_universe_count *= 27;
        a_finish_count = 27 * a_finish_count + new_a_finish_count;
        a_win_count += new_a_finish_count * (b_universe_count - b_finish_count);
        let (new_b_universes, new_b_finish_count) = update_universes(&b_universes);
        b_universes = new_b_universes;
        b_universe_count *= 27;
        b_finish_count = 27 * b_finish_count + new_b_finish_count;
        b_win_count += new_b_finish_count * (a_universe_count - a_finish_count);
    }
    cmp::max(a_win_count, b_win_count)
}

fn parse_start_positions(input: RawInput) -> (usize, usize) {
    let positions = input.per_line(|line| (line.as_str().as_bytes()[28] - b'0') as usize);
    (positions[0], positions[1])
}

fn get_intiial_universes(start_pos: usize) -> Array2<u64> {
    let mut result = Array2::zeros((10, 21));
    result[[start_pos - 1, 0]] = 1;
    result
}

/// universes[i][j] is the number of universes where this player is on spot i-1
/// and has j points. Returns the updated universes as well as the number of
/// universes where this player reached 21 or higher this round.
fn update_universes(universes: &Array2<u64>) -> (Array2<u64>, u64) {
    let mut new_universes = Array2::zeros((10, 21));
    let mut finish_count = 0;
    for roll in 3..=9 {
        let universe_count = get_universe_count_for_roll(roll);
        for spot in 0..10 {
            for score in 0..21 {
                let old_universes = universes[[spot, score]];
                let new_spot = (spot + roll) % 10;
                let new_score = score + new_spot + 1;
                let added_universes = old_universes * universe_count;
                if new_score >= 21 {
                    finish_count += added_universes;
                } else {
                    new_universes[[new_spot, new_score]] += added_universes;
                }
            }
        }
    }
    (new_universes, finish_count)
}

/// The number of ways of rolling each possible sum with three 3-sided dice.
fn get_universe_count_for_roll(roll: usize) -> u64 {
    match roll {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => unreachable!(),
    }
}

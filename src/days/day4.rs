use crate::harness::input::RawInput;
use ndarray::Array2;

pub fn solve_part1(input: RawInput) -> usize {
    let Input { calls, boards } = input.into();
    let mut called_nums = [false; 100];
    for call in calls {
        called_nums[call] = true;
        if let Some(board) = boards.iter().find(|board| has_line(board, &called_nums)) {
            return score_board(board, &called_nums, call);
        }
    }
    unreachable!()
}

pub fn solve_part2(input: RawInput) -> usize {
    let Input { calls, mut boards } = input.into();
    let mut called_nums = [false; 100];
    for call in calls {
        called_nums[call] = true;
        if boards.len() > 1 {
            boards.retain(|board| !has_line(board, &called_nums));
        } else if has_line(&boards[0], &called_nums) {
            return score_board(&boards[0], &called_nums, call);
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct Input {
    calls: Vec<usize>,
    boards: Vec<Array2<usize>>,
}

impl From<RawInput<'_>> for Input {
    fn from(input: RawInput<'_>) -> Self {
        let groups = input.grouped_lines(|line| line.single::<String>());
        let calls = groups[0][0]
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        let boards = groups
            .iter()
            .skip(1)
            .map(|group| parse_board(group))
            .collect();
        Input { calls, boards }
    }
}

fn parse_board(group: &[String]) -> Array2<usize> {
    let side = group.len();
    let mut board = Array2::<usize>::zeros([side, side]);
    for i in 0..side {
        group[i]
            .split_ascii_whitespace()
            .enumerate()
            .for_each(|(j, s)| board[[i, j]] = s.parse::<usize>().unwrap());
    }
    board
}

fn has_line(board: &Array2<usize>, called_nums: &[bool]) -> bool {
    let side = board.nrows();
    (0..side).any(|i| {
        (0..side).all(|j| called_nums[board[[i, j]]])
            || (0..side).all(|j| called_nums[board[[j, i]]])
    })
}

fn score_board(board: &Array2<usize>, called_nums: &[bool], last_called: usize) -> usize {
    board.iter().filter(|&&n| !called_nums[n]).sum::<usize>() * last_called
}

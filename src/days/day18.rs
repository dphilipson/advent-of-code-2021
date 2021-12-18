use crate::harness::input::RawInput;
use std::cmp;
use std::str::FromStr;

pub fn solve_part1(input: RawInput) -> u32 {
    let snailfish_nums = input.per_line(|line| line.single::<SnailfishNum>());
    let sum = snailfish_nums.into_iter().reduce(|a, b| a.add(&b)).unwrap();
    sum.get_magnitude()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let snailfish_nums = input.per_line(|line| line.single::<SnailfishNum>());
    let mut best_magnitude = 0;
    for i in 0..snailfish_nums.len() {
        for j in 0..snailfish_nums.len() {
            if i != j {
                let magnitude = snailfish_nums[i].add(&snailfish_nums[j]).get_magnitude();
                best_magnitude = cmp::max(best_magnitude, magnitude);
            }
        }
    }
    best_magnitude
}

#[derive(Copy, Clone, Debug)]
enum Token {
    PairStart,
    Num(u32),
}

impl Token {
    fn downcast_to_num(self) -> u32 {
        match self {
            Self::Num(n) => n,
            _ => panic!("Cannot downcast {:?} to number.", self),
        }
    }
}

#[derive(Debug)]
struct SnailfishNum(Vec<Token>);

impl FromStr for SnailfishNum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s
            .as_bytes()
            .iter()
            .filter_map(|&b| match b {
                b'[' => Some(Token::PairStart),
                b'0'..=b'9' => Some(Token::Num((b - b'0') as u32)),
                _ => None,
            })
            .collect();
        Ok(Self(tokens))
    }
}

impl SnailfishNum {
    fn add(&self, rhs: &Self) -> Self {
        let mut tokens = Vec::with_capacity(self.0.len() + rhs.0.len() + 1);
        tokens.push(Token::PairStart);
        tokens.extend(&self.0);
        tokens.extend(&rhs.0);
        let mut result = Self(tokens);
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        while self.reduce_once() {}
    }

    fn reduce_once(&mut self) -> bool {
        if let Some(i) = self.find_explode_index() {
            self.explode_at(i);
            true
        } else if let Some(i) = self.find_split_index() {
            self.split_at(i);
            true
        } else {
            false
        }
    }

    fn find_explode_index(&self) -> Option<usize> {
        let mut depth = 0;
        // true on stack represents a number, false a pairstart.
        let mut stack = vec![];
        for (i, &token) in self.0.iter().enumerate() {
            match token {
                Token::PairStart => {
                    depth += 1;
                    if depth > 4 {
                        return Some(i);
                    }
                    stack.push(false);
                }
                Token::Num(_) => {
                    stack.push(true);
                    while stack.len() > 2 && stack[stack.len() - 1] && stack[stack.len() - 2] {
                        for _ in 0..3 {
                            stack.pop();
                        }
                        stack.push(true);
                        depth -= 1;
                    }
                }
            }
        }
        None
    }

    fn find_split_index(&self) -> Option<usize> {
        self.0.iter().position(|&token| match token {
            Token::Num(n) => n >= 10,
            _ => false,
        })
    }

    fn explode_at(&mut self, i: usize) {
        let left = self.0[i + 1].downcast_to_num();
        let right = self.0[i + 2].downcast_to_num();
        self.0.splice(i..i + 3, [Token::Num(0)]);
        for j in (0..i).rev() {
            match self.0[j] {
                Token::Num(n) => {
                    self.0[j] = Token::Num(n + left);
                    break;
                }
                _ => (),
            }
        }
        for j in i + 1..self.0.len() {
            match self.0[j] {
                Token::Num(n) => {
                    self.0[j] = Token::Num(n + right);
                    break;
                }
                _ => (),
            }
        }
    }

    fn split_at(&mut self, i: usize) {
        let n = self.0[i].downcast_to_num();
        self.0.splice(
            i..i + 1,
            [Token::PairStart, Token::Num(n / 2), Token::Num((n + 1) / 2)],
        );
    }

    fn get_magnitude(&self) -> u32 {
        let mut stack = vec![];
        for &token in &self.0 {
            stack.push(token);
            while stack.len() > 2 {
                if let Token::Num(top) = stack[stack.len() - 1] {
                    if let Token::Num(second_top) = stack[stack.len() - 2] {
                        for _ in 0..3 {
                            stack.pop();
                        }
                        stack.push(Token::Num(2 * top + 3 * second_top));
                        continue;
                    }
                }
                break;
            }
        }
        stack[0].downcast_to_num()
    }
}

use crate::harness::input::RawInput;
use std::cmp;

pub fn solve_part1(input: RawInput) -> u32 {
    let lines = input.per_line(|line| line.single::<String>());
    let mut acc = parse_tokens(&lines[0]);
    acc = reduce_snailfish_number(acc);
    for line in &lines[1..] {
        let tokens = parse_tokens(line);
        acc = add_snailfish_numbers(&acc, &tokens);
        acc = reduce_snailfish_number(acc);
    }
    let (tree, _) = parse_tree(&acc);
    tree.get_value()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let lines = input.per_line(|line| line.single::<String>());
    let mut best = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i != j {
                let sum = add_snailfish_numbers(&parse_tokens(&lines[i]), &parse_tokens(&lines[j]));
                let reduced_sum = reduce_snailfish_number(sum);
                let (tree, _) = parse_tree(&reduced_sum);
                best = cmp::max(best, tree.get_value());
            }
        }
    }
    best
}

#[derive(Copy, Clone, Debug)]
enum Token {
    Open,
    Close,
    Comma,
    Num(u32),
}

impl Token {
    fn downcast_to_num(self) -> u32 {
        if let Token::Num(n) = self {
            n
        } else {
            panic!("Token was not a number. Was: {:?}", self)
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match &self {
            Token::Open => "[".to_owned(),
            Token::Close => "]".to_owned(),
            Token::Comma => ",".to_owned(),
            Token::Num(n) => n.to_string(),
        }
    }
}

fn parse_tokens(s: &str) -> Vec<Token> {
    let mut bytes: &[u8] = &s.bytes().collect::<Vec<_>>();
    let mut result = vec![];
    while let Some((token, remaining_bytes)) = parse_token(bytes) {
        result.push(token);
        bytes = remaining_bytes;
    }
    result
}

fn tokens_to_string(tokens: &[Token]) -> String {
    tokens.iter().map(|token| token.to_string()).collect()
}

fn parse_token(mut bytes: &[u8]) -> Option<(Token, &[u8])> {
    if let Some(&byte) = bytes.first() {
        match byte {
            b'[' => Some((Token::Open, &bytes[1..])),
            b']' => Some((Token::Close, &bytes[1..])),
            b',' => Some((Token::Comma, &bytes[1..])),
            _ => {
                let mut n = 0;
                while (b'0'..=b'9').contains(&bytes[0]) {
                    n = n * 10 + (bytes[0] - b'0') as u32;
                    bytes = &bytes[1..];
                }
                Some((Token::Num(n), bytes))
            }
        }
    } else {
        None
    }
}

fn add_snailfish_numbers(tokens1: &[Token], tokens2: &[Token]) -> Vec<Token> {
    let mut result = Vec::with_capacity(tokens1.len() + tokens2.len() + 3);
    result.push(Token::Open);
    result.extend(tokens1);
    result.push(Token::Comma);
    result.extend(tokens2);
    result.push(Token::Close);
    result
}

fn reduce_snailfish_number(mut tokens: Vec<Token>) -> Vec<Token> {
    while let Some(next_tokens) = apply_step(&tokens) {
        tokens = next_tokens;
    }
    tokens
}

/// Returns None if the tokens are settled.
fn apply_step(tokens: &[Token]) -> Option<Vec<Token>> {
    if let Some(i) = find_exploding_start(tokens) {
        Some(explode_at(tokens, i))
    } else if let Some(i) = find_splitting_start(tokens) {
        Some(split_at(tokens, i))
    } else {
        None
    }
}

fn find_exploding_start(tokens: &[Token]) -> Option<usize> {
    let mut depth_count = 0;
    for (i, &token) in tokens.iter().enumerate() {
        match token {
            Token::Open => {
                depth_count += 1;
                if depth_count > 4 {
                    return Some(i);
                }
            }
            Token::Close => depth_count -= 1,
            _ => (),
        }
    }
    None
}

fn find_splitting_start(tokens: &[Token]) -> Option<usize> {
    tokens.iter().position(|&token| match token {
        Token::Num(n) => n >= 10,
        _ => false,
    })
}

fn explode_at(tokens: &[Token], i: usize) -> Vec<Token> {
    let mut result = Vec::with_capacity(tokens.len() - 4);
    let left = tokens[i + 1].downcast_to_num();
    let right = tokens[i + 3].downcast_to_num();
    result.extend(&tokens[..i]);
    result.push(Token::Num(0));
    result.extend(&tokens[(i + 5)..]);
    for i in (0..i).rev() {
        if let Token::Num(n) = result[i] {
            result[i] = Token::Num(n + left);
            break;
        }
    }
    for i in i + 1..result.len() {
        if let Token::Num(n) = result[i] {
            result[i] = Token::Num(n + right);
            break;
        }
    }
    result
}

fn split_at(tokens: &[Token], i: usize) -> Vec<Token> {
    let mut result = Vec::with_capacity(tokens.len() + 4);
    let n = tokens[i].downcast_to_num();
    result.extend(&tokens[..i]);
    result.extend([
        Token::Open,
        Token::Num(n / 2),
        Token::Comma,
        Token::Num((n + 1) / 2),
        Token::Close,
    ]);
    result.extend(&tokens[i + 1..]);
    result
}

#[derive(Debug)]
enum Node {
    Pair(Box<Node>, Box<Node>),
    Num(u32),
}

impl Node {
    fn get_value(&self) -> u32 {
        match self {
            Node::Pair(left, right) => 3 * left.get_value() + 2 * right.get_value(),
            Node::Num(n) => *n,
        }
    }
}

fn parse_tree(tokens: &[Token]) -> (Node, &[Token]) {
    match tokens[0] {
        Token::Num(n) => (Node::Num(n), &tokens[1..]),
        Token::Open => {
            let (left, tokens) = parse_tree(&tokens[1..]);
            let (right, tokens) = parse_tree(&tokens[1..]);
            (Node::Pair(Box::new(left), Box::new(right)), &tokens[1..])
        }
        _ => unreachable!(),
    }
}

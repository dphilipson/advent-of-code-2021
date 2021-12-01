use crate::regex;
use input::RawInput;
use std::fmt::{Debug, Display};
use std::panic::RefUnwindSafe;
use std::path::Path;
use std::str::FromStr;
use std::time::Instant;
use std::{error, fs};

pub mod input;
mod panics;

pub fn solve<F1, F2, O1, O2>(day: usize, solve_part1: F1, solve_part2: F2)
where
    F1: RefUnwindSafe + Fn(RawInput) -> O1,
    F2: RefUnwindSafe + Fn(RawInput) -> O2,
    O1: Display + Eq + FromStr,
    O2: Display + Eq + FromStr,
    <O1 as FromStr>::Err: error::Error + 'static,
    <O2 as FromStr>::Err: error::Error + 'static,
{
    let input_filename = format!("input/day{}-input.txt", day);
    let test_input_filename = format!("input/day{}-test-input.txt", day);
    let input_text = fs::read_to_string(Path::new(&input_filename)).unwrap();
    let raw_test_input = fs::read_to_string(Path::new(&test_input_filename)).unwrap();
    let test_input = TestInput::try_from(raw_test_input.as_str()).unwrap();
    solve_part(SolvePartArgs {
        part: 1,
        solve: solve_part1,
        input: &input_text,
        test_input: &test_input.text,
        test_expected_output: test_input.part1_expected,
    });
    println!();
    solve_part(SolvePartArgs {
        part: 2,
        solve: solve_part2,
        input: &input_text,
        test_input: &test_input.text,
        test_expected_output: test_input.part2_expected,
    });
}

#[derive(Copy, Clone, Debug)]
struct SolvePartArgs<'a, F, O> {
    part: usize,
    solve: F,
    input: &'a str,
    test_input: &'a str,
    test_expected_output: Option<O>,
}

fn solve_part<F, O>(
    SolvePartArgs {
        part,
        solve,
        input,
        test_input,
        test_expected_output,
    }: SolvePartArgs<F, O>,
) where
    F: RefUnwindSafe + Fn(RawInput) -> O,
    O: Display + Eq,
{
    if let Some(expected) = test_expected_output {
        if let Some(test_output) = panics::catching_todo(|| solve(RawInput::new(test_input))) {
            if test_output == expected {
                println!("Part {} test output: {} ✅", part, test_output);
            } else {
                println!("Part {} test output: {} ❌", part, test_output);
                println!("          Expected: {}", expected);
                return;
            }
        } else {
            println!("Part {} not implemented.", part);
            return;
        }
    }
    let result = panics::catching_todo(|| {
        let start_time = Instant::now();
        let output = solve(RawInput::new(input));
        let duration = start_time.elapsed();
        (output, duration)
    });
    if let Some((output, duration)) = result {
        println!("Part {} output: {}", part, output);
        println!("   ↑ Duration: {:.2?}", duration);
    } else {
        println!("Part {} not implemented.", part);
    }
}

#[derive(Copy, Clone, Debug)]
struct TestInput<'a, O1, O2> {
    part1_expected: Option<O1>,
    part2_expected: Option<O2>,
    text: &'a str,
}

// Not implementing `FromStr` because of the lifetime bound.
impl<'a, O1, O2> TryFrom<&'a str> for TestInput<'a, O1, O2>
where
    O1: FromStr,
    O2: FromStr,
    <O1 as FromStr>::Err: error::Error + 'static,
    <O2 as FromStr>::Err: error::Error + 'static,
{
    type Error = Box<dyn error::Error>;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let re = regex!(
            r"(?s)^Part 1 expected: *([^\n]+)?
Part 2 expected: *([^\n]+)?
 *
(.*)$"
        );
        let caps = re.captures(s).ok_or("Invalid test input format.")?;
        Ok(TestInput {
            part1_expected: caps.get(1).map(|m| m.as_str().parse()).transpose()?,
            part2_expected: caps.get(2).map(|m| m.as_str().parse()).transpose()?,
            text: caps.get(3).unwrap().as_str(),
        })
    }
}

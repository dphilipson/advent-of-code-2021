use crate::util::re;
use crate::util::re::MatchTuple;
use regex::Regex;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub struct LineInput<'a>(&'a str);

impl<'a> LineInput<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn single<T>(&self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.0.parse().unwrap()
    }

    pub fn chars(&self) -> Vec<char> {
        self.0.chars().collect()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.0.bytes().collect()
    }

    pub fn digits(&self) -> Vec<u32> {
        self.0.bytes().map(|b| (b - b'0') as u32).collect()
    }

    pub fn split<T>(&self, pattern: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.0.split(pattern).map(|s| s.parse().unwrap()).collect()
    }

    pub fn split_whitespace<T>(&self) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.0
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    pub fn parse_with_regex<T>(&self, re: &Regex) -> T
    where
        T: MatchTuple,
    {
        re::parse_with_regex(re, &self.0).unwrap()
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RawInput<'a>(&'a str);

impl<'a> RawInput<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &'a str {
        self.0
    }

    pub fn single_line<F, T>(&self, f: F) -> T
    where
        F: Fn(LineInput) -> T,
    {
        let line = self.0.lines().next().unwrap();
        f(LineInput(line))
    }

    pub fn per_line<F, T>(&self, f: F) -> Vec<T>
    where
        F: Fn(LineInput) -> T,
    {
        self.0.lines().map(|line| f(LineInput(line))).collect()
    }

    pub fn grouped_lines<F, T>(&self, f: F) -> Vec<Vec<T>>
    where
        F: Fn(LineInput) -> T,
    {
        self.0
            .split("\n\n")
            .map(|group| group.lines().map(|line| f(LineInput(line))).collect())
            .collect()
    }

    pub fn raw_str(&self) -> &str {
        self.0
    }
}

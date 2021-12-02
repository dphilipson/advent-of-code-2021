use regex::{Captures, Regex};
use std::error;

/// Converts a string literal into a Regex, caching the value in a static variable for reuse.
#[macro_export]
macro_rules! regex {
    ($str:literal) => {{
        lazy_static::lazy_static! {
            static ref RE: regex::Regex = regex::Regex::new($str).unwrap();
        }
        let result: &regex::Regex = &RE;
        result
    }};
}

/// Matches a regex to a string, then parses each capture group as necessary to
/// produce a tuple of the desired return type.
pub fn parse_with_regex<T: MatchTuple>(re: &Regex, s: &str) -> Result<T, Box<dyn error::Error>> {
    let caps = re.captures(s).ok_or("Regex did not match string.")?;
    if caps.len() != T::len() + 1 {
        Err(format!(
            "Expected {} (non-global) capture groups, found {}.",
            T::len(),
            caps.len() - 1,
        ))?
    }
    Ok(T::parse_captures(&caps)?)
}

pub trait MatchTuple: Sized {
    fn len() -> usize;
    fn parse_captures(caps: &Captures) -> Result<Self, Box<dyn error::Error>>;
}

macro_rules! impl_match_tuple {
    ($($T:ident),*) => {
        impl <$($T, )*> MatchTuple for ($($T, )*)
        where
            $(
                $T: std::str::FromStr,
                <$T as std::str::FromStr>::Err: std::error::Error + 'static,
            )*
        {
            fn len() -> usize {
                count_args!($($T )*)
            }

            fn parse_captures(_caps: &Captures) -> Result<Self, Box<dyn std::error::Error>> {
                Ok(parse_to_tuple!(_caps $($T )*))
            }
        }
    }
}

macro_rules! count_args {
    () => {
        0
    };
    ($head:ident $($tail:tt)*) => {
        count_args!($($tail)*) + 1
    };
}

macro_rules! parse_to_tuple {
    (@accum [$caps:ident][][$($n:tt)*] -> [$($body:tt)*]) => {
        as_expr!(($($body)*))
    };
    (@accum [$caps:ident][$head:ident $($tail:tt)*][$($n:tt)*] -> [$($body:tt)*]) => {
        parse_to_tuple!(@accum [$caps][$($tail)*][$($n)* + 1] -> [$($body)* $caps[$($n)*].parse()?,])
    };
    (@as_expr $e:expr) => {
        $e
    };
    ($caps:ident $($all:tt)*) => {
        parse_to_tuple!(@accum [$caps][$($all)*][1] -> [])
    }
}

macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}

impl_match_tuple!();
impl_match_tuple!(A);
impl_match_tuple!(A, B);
impl_match_tuple!(A, B, C);
impl_match_tuple!(A, B, C, D);
impl_match_tuple!(A, B, C, D, E);
impl_match_tuple!(A, B, C, D, E, F);
impl_match_tuple!(A, B, C, D, E, F, G);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_match() {
        let re = regex!(r"^(.+) stole (\d+) cakes.$");
        let (name, count): (String, usize) =
            parse_with_regex(re, "Lex Luthor stole 40 cakes.").unwrap();
        assert_eq!(name, "Lex Luthor".to_owned());
        assert_eq!(count, 40);
    }
}

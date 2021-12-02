use std::error;
use std::fmt::{Display, Formatter};

/// Creates an enum where each variant parses from and formats to a fixed
/// string. The syntax is made to resemble TypeScript's.
#[macro_export]
macro_rules! string_enum {
    ($visibility:vis $name:ident {
        $($value:ident = $s:literal),* $(,)?
    }) => {
        #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
        $visibility enum $name {
            $(
                $value,
            )*
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(match self {
                    $(
                        Self::$value => $s,
                    )*
                })
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::util::string_enum::ParseEnumError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(
                        $s => Ok(Self::$value),
                    )*
                    _ => Err(Self::Err::new(stringify!($name), s.to_owned())),
                }
            }
        }
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseEnumError {
    enum_name: &'static str,
    invalid_variant: String,
}

impl ParseEnumError {
    pub fn new(enum_name: &'static str, invalid_variant: String) -> Self {
        Self {
            enum_name,
            invalid_variant,
        }
    }
}

impl Display for ParseEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid variant for {}: {}",
            self.enum_name, self.invalid_variant
        )
    }
}

impl error::Error for ParseEnumError {}

#[cfg(test)]
mod tests {
    use super::*;

    string_enum!(Color {
        Red = "red",
        Green = "green",
        Blue = "blue",
    });

    #[test]
    fn test_string_enum() {
        assert_eq!(Color::Red.to_string(), "red");
        assert_eq!("green".parse::<Color>(), Ok(Color::Green));
        assert_eq!(
            "person".parse::<Color>().unwrap_err(),
            ParseEnumError {
                enum_name: "Color",
                invalid_variant: "person".to_owned()
            }
        );
    }
}

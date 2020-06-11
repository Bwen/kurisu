pub const VALUE_SEPARATOR: &str = "(_(__)===D";

#[cfg(feature = "parser_extras")]
mod extras;
#[cfg(feature = "parser_extras")]
pub use extras::*;

use crate::Kurisu;
use std::path::PathBuf;

pub trait Parser {
    fn parse(value: &str) -> Self;
}

impl Parser for String {
    fn parse(value: &str) -> Self {
        value.to_string()
    }
}

impl Parser for Vec<String> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value.split(VALUE_SEPARATOR).map(|v| v.to_string()).collect()
    }
}

impl Parser for Option<String> {
    fn parse(value: &str) -> Self {
        if value.is_empty() || value.eq("None") {
            return None;
        }

        Some(value.to_string())
    }
}

impl Parser for bool {
    fn parse(value: &str) -> Self {
        value.parse::<bool>().unwrap_or_default()
    }
}

impl Parser for u8 {
    fn parse(value: &str) -> Self {
        value.parse::<u8>().unwrap_or_default()
    }
}

impl Parser for usize {
    fn parse(value: &str) -> Self {
        value.parse::<usize>().unwrap_or_default()
    }
}

impl Parser for Vec<usize> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value.split(VALUE_SEPARATOR).map(|v| v.parse::<usize>().unwrap_or_default()).collect()
    }
}

impl Parser for Option<usize> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return None;
        }

        if let Ok(result) = value.parse::<usize>() {
            return Some(result);
        }

        None
    }
}

impl Parser for isize {
    fn parse(value: &str) -> Self {
        value.parse::<isize>().unwrap_or_default()
    }
}

impl Parser for Vec<isize> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value.split(VALUE_SEPARATOR).map(|v| v.parse::<isize>().unwrap_or_default()).collect()
    }
}

impl Parser for Option<isize> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return None;
        }

        if let Ok(result) = value.parse::<isize>() {
            return Some(result);
        }

        None
    }
}

impl Parser for f64 {
    fn parse(value: &str) -> Self {
        value.parse::<f64>().unwrap_or_default()
    }
}

impl Parser for Vec<f64> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value.split(VALUE_SEPARATOR).map(|v| v.parse::<f64>().unwrap_or_default()).collect()
    }
}

impl Parser for Option<f64> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return None;
        }

        if let Ok(result) = value.parse::<f64>() {
            return Some(result);
        }

        None
    }
}

impl Parser for PathBuf {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return PathBuf::default();
        }

        PathBuf::from(value)
    }
}

impl Parser for Vec<PathBuf> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value
            .split(VALUE_SEPARATOR)
            .map(|v| {
                if v.is_empty() {
                    return PathBuf::default();
                }

                PathBuf::from(v)
            })
            .collect()
    }
}

impl Parser for Option<PathBuf> {
    fn parse(value: &str) -> Self {
        if value.is_empty() || value.eq("None") {
            return None;
        }

        Some(PathBuf::from(value))
    }
}

impl<T> Parser for Option<T>
where
    T: Kurisu,
{
    fn parse(value: &str) -> Self {
        None
    }
}

pub(crate) const VALUE_SEPARATOR: &str = "(_(__)===D";

use std::path::PathBuf;
pub trait ArgParser {
    fn parse(value: &str) -> Self;
}

impl ArgParser for String {
    fn parse(value: &str) -> Self {
        value.to_string()
    }
}

impl ArgParser for Vec<String> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value.split(VALUE_SEPARATOR).map(|v| v.to_string()).collect()
    }
}

impl ArgParser for Option<String> {
    fn parse(value: &str) -> Self {
        if value.is_empty() || value.eq("None") {
            return None;
        }

        Some(value.to_string())
    }
}

impl ArgParser for bool {
    fn parse(value: &str) -> Self {
        value.parse::<bool>().unwrap_or_default()
    }
}

impl ArgParser for usize {
    fn parse(value: &str) -> Self {
        value.parse::<usize>().unwrap_or_default()
    }
}

impl ArgParser for Vec<usize> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value.split(VALUE_SEPARATOR).map(|v| v.parse::<usize>().unwrap_or_default()).collect()
    }
}

impl ArgParser for isize {
    fn parse(value: &str) -> Self {
        value.parse::<isize>().unwrap_or_default()
    }
}

impl ArgParser for Vec<isize> {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return Vec::new();
        }

        value.split(VALUE_SEPARATOR).map(|v| v.parse::<isize>().unwrap_or_default()).collect()
    }
}

impl ArgParser for PathBuf {
    fn parse(value: &str) -> Self {
        if value.is_empty() {
            return PathBuf::default();
        }

        PathBuf::from(value)
    }
}

impl ArgParser for Vec<PathBuf> {
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

impl ArgParser for Option<PathBuf> {
    fn parse(value: &str) -> Self {
        if value.is_empty() || value.eq("None") {
            return None;
        }

        Some(PathBuf::from(value))
    }
}

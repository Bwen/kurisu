use std::path::PathBuf;

pub trait ArgParser {
    fn parse(value: &str) -> Self;
}

impl ArgParser for String {
    fn parse(value: &str) -> String {
        value.to_string()
    }
}

impl ArgParser for Option<String> {
    fn parse(value: &str) -> Option<String> {
        if value.is_empty() || value.eq("None") {
            return None;
        }

        Some(value.to_string())
    }
}

impl ArgParser for bool {
    fn parse(value: &str) -> bool {
        value.parse::<bool>().unwrap_or_default()
    }
}

impl ArgParser for usize {
    fn parse(value: &str) -> usize {
        value.parse::<usize>().unwrap_or_default()
    }
}

impl ArgParser for isize {
    fn parse(value: &str) -> isize {
        value.parse::<isize>().unwrap_or_default()
    }
}

impl ArgParser for PathBuf {
    fn parse(value: &str) -> PathBuf {
        if value.is_empty() {
            return PathBuf::default();
        }

        PathBuf::from(value)
    }
}

impl ArgParser for Option<PathBuf> {
    fn parse(value: &str) -> Option<PathBuf> {
        if value.is_empty() || value.eq("None") {
            return None;
        }

        Some(PathBuf::from(value))
    }
}

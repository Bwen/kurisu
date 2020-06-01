mod error;
mod parser;

pub use error::Error;
pub use parser::{Parser, VALUE_SEPARATOR};

const TYPES_NO_VALUE: &[&str] = &["bool"];

#[derive(Debug, Clone, PartialEq)]
pub struct Arg<'a> {
    pub name: &'a str,
    pub value_type: &'a str,
    pub position: Option<i8>,
    pub doc: Option<&'a str>,
    pub short: Option<&'a str>,
    pub long: Option<&'a str>,
    pub exit: Option<fn() -> i32>,
    pub required_if: Option<&'a str>,
    pub default: &'a str,
    pub value: Vec<String>,
    pub occurrences: usize,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Arg<'a> {
        Arg {
            name: "",
            value_type: "",
            position: None,
            doc: None,
            short: None,
            long: None,
            exit: None,
            required_if: None,
            default: "",
            value: Vec::new(),
            occurrences: 0,
        }
    }
}

impl<'a> PartialEq<String> for &Arg<'a> {
    fn eq(&self, other: &String) -> bool {
        self.partial_eq_string(other)
    }
}

impl<'a> PartialEq<String> for &mut Arg<'a> {
    fn eq(&self, other: &String) -> bool {
        self.partial_eq_string(other)
    }
}

impl<'a> Arg<'a> {
    fn partial_eq_string<S: AsRef<str>>(&self, value: S) -> bool {
        let value = value.as_ref();
        if !value.starts_with('-') {
            return false;
        }

        if value.starts_with("--") && self.long.is_some() {
            return value.starts_with(&format!("--{}", self.long.expect("Infallible")));
        }

        if value.starts_with('-') && self.short.is_some() {
            return value.starts_with(&format!("-{}", self.short.expect("Infallible")));
        }

        false
    }

    pub fn is_value_required(&self) -> bool {
        !TYPES_NO_VALUE.contains(&self.value_type) && !self.value_type.starts_with("Option")
    }

    pub fn is_value_multiple(&self) -> bool {
        self.value_type.starts_with("Vec")
    }

    pub fn is_value_none(&self) -> bool {
        TYPES_NO_VALUE.contains(&self.value_type)
    }

    pub fn set_value(&'_ mut self, args: &[String], positions: &[i8]) {
        // TODO: What to do with Optional values?

        let mut pos = 1;
        let mut options_ended = false;
        for (i, arg) in args.iter().enumerate() {
            // We only drop the first --
            if !options_ended && arg.len() == 2 && arg == "--" {
                options_ended = true;
                continue;
            }

            if self.eq(arg) {
                if arg.contains('=') {
                    let value: Vec<&str> = arg.split('=').collect();
                    self.occurrences += 1;
                    self.value.push(value[1].to_owned());
                    continue;
                }

                self.occurrences += 1;

                // If we have a short, nolong u8 we put the number of occurrences in its value
                if self.value_type == "u8" && self.long.is_none() && self.short.is_some() {
                    self.value = vec![format!("{}", self.occurrences)];
                } else if self.is_value_none() {
                    self.value.push(String::from("true"));
                }
            } else if options_ended || !arg.starts_with('-') || (arg.starts_with('-') && arg.len() == 1) {
                if let Some(position) = self.position {
                    if position != pos && position != 0 && position != -1 {
                        pos += 1;
                        continue;
                    }

                    // If the arg is infinite but another arg has this position
                    if position == 0 && (positions.contains(&pos) || (positions.contains(&-1) && (i + 1) == args.len())) {
                        pos += 1;
                        continue;
                    }

                    // If we seek the last argument position
                    if position == -1 && (i + 1) != args.len() {
                        pos += 1;
                        continue;
                    }

                    self.occurrences += 1;
                    self.value.push(arg.clone());
                    pos += 1;
                }
            }
        }

        if !self.value.is_empty() {
            return;
        }

        // TODO: Fall back to check environment variables with the same name capitalized,
        // Maybe it should be mentioned in the annotation as to WHICH env var is attached to which field, to allow branding MYPROGRAM_PATH_TO_SOMETHING
        // Or... Should it? field mysql_host for example would allow a quick access to environment variables that dont require branding...
        // Optional branding annotation?
    }
}

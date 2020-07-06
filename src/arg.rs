mod error;
mod parser;

use core::fmt;
pub use error::Error;
pub use parser::{Parser, VALUE_SEPARATOR};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;

const TYPES_NO_VALUE: &[&str] = &["bool"];

// TODO: Arg type that can take stdin? < file redirected into?
// use std::io::{self, stdin, Read};
//
// fn main() -> io::Result<()> {
//     let mut buf = String::new();
//     stdin().lock().read_to_string(&mut buf)?;
//     println!("{}", buf);
//     Ok(())
// }
#[derive(Debug, Clone)]
pub struct Arg<'a> {
    pub name: &'a str,
    pub vname: Option<&'a str>,
    pub value_type: &'a str,
    pub position: Option<i8>,
    pub doc: Option<&'a str>,
    pub short: Option<&'a str>,
    pub long: Option<&'a str>,
    pub exit: Option<fn() -> i32>,
    pub env: Option<&'a str>,
    pub env_prefix: Option<&'a str>,
    pub required_if: Option<&'a str>,
    pub default: &'a str,
    pub value: Vec<String>,
    pub occurrences: usize,
}

impl<'a> Default for Arg<'a> {
    fn default() -> Arg<'a> {
        Arg {
            name: "",
            vname: None,
            value_type: "",
            position: None,
            doc: None,
            short: None,
            long: None,
            exit: None,
            env: None,
            env_prefix: None,
            required_if: None,
            default: "",
            value: Vec::new(),
            occurrences: 0,
        }
    }
}

impl<'a> Display for Arg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut short = String::from("  ");
        if let Some(s) = self.short {
            short = format!("-{}", s);
        }

        let mut long = String::from("");
        if let Some(l) = self.long {
            long = format!(" --{}", l);
        }

        let multiple = if self.is_value_multiple() { "..." } else { "" };
        let value = if !self.is_value_none() {
            let value_name = if self.vname.is_some() { self.vname.unwrap() } else { self.name };
            format!(" <{}>{}", value_name.to_uppercase(), multiple)
        } else {
            String::from("")
        };

        write!(f, "{}{}{}", short, long, value)
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

impl<'a> PartialEq<Arg<'a>> for Arg<'a> {
    fn eq(&self, other: &Self) -> bool {
        if (self.long.is_some() && self.long.is_some() && self.long.unwrap() == other.long.unwrap())
            || (self.short.is_some() && self.short.is_some() && self.short.unwrap() == other.short.unwrap())
        {
            true
        } else {
            self.name == other.name
        }
    }
}

impl<'a> PartialOrd for Arg<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.get_first_letter().cmp(&other.get_first_letter()))
    }
}

impl<'a> Arg<'a> {
    fn get_first_letter(&self) -> &'a str {
        if let Some(long) = self.long {
            &long[0..1]
        } else if let Some(short) = self.short {
            short
        } else {
            "0"
        }
    }

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
        self.default.is_empty() && !TYPES_NO_VALUE.contains(&self.value_type)
    }

    pub fn is_value_multiple(&self) -> bool {
        self.value_type.starts_with("Vec")
    }

    pub fn is_value_none(&self) -> bool {
        TYPES_NO_VALUE.contains(&self.value_type)
    }

    pub fn set_value(&'_ mut self, args: &[String], positions: &[i8]) {
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
                    let value: Vec<&str> = arg.splitn(2, '=').collect();
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

        if !self.default.is_empty() {
            self.value.push(self.default.to_string());
        } else {
            let vars: HashMap<String, String> = std::env::vars().collect();
            for (key, value) in vars {
                let mut env_var = self.name.to_string();
                if self.env.is_some() {
                    env_var = self.env.expect("Infallible").to_string();
                } else if self.env_prefix.is_some() {
                    env_var = format!("{}{}", self.env_prefix.expect("Infallible"), self.name)
                }

                if key.to_lowercase() == env_var.to_lowercase() {
                    self.value.push(value);
                    break;
                }
            }
        }
    }
}

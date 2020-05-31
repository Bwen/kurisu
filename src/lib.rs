pub mod arg;
mod exit_code;
pub mod mayuri;

pub use arg::Arg;
use arg::{Parser, VALUE_SEPARATOR};
pub use exit_code::*;
pub use once_cell::sync::OnceCell;
use std::sync::Mutex;

// TODO: Simplify namespaces where we can (outside macro output)

use crate::arg::Error;
#[doc(hidden)]
pub use kurisu_derive::*;

/// Hello there... Tuturuuuu ♫
pub trait Kurisu<'a> {
    fn from_args(env_args: Vec<String>) -> Self;
    fn get_info_instance(env_args: Vec<String>) -> &'static Mutex<Info<'static>>;
}

#[derive(Debug)]
pub struct Info<'a> {
    pub name: Option<&'a str>,
    pub version: Option<&'a str>,
    pub doc: Option<&'a str>,
    pub allow_noargs: bool,
    pub env_args: Vec<String>,
    pub args: Vec<Arg<'a>>,
}

pub fn exit_args(info: &Info<'static>) {
    let exit_args: Vec<&Arg<'_>> = info
        .args
        .iter()
        .filter(|a| a.exit.is_some() || ["usage", "version"].contains(&a.name))
        .collect();

    for arg in exit_args {
        if arg.occurrences == 0 {
            continue;
        }

        if arg.name == "version" {
            std::process::exit(mayuri::print_version(&info));
        } else if arg.name == "usage" {
            std::process::exit(mayuri::print_usage(&info));
        }

        if arg.exit.is_some() {
            std::process::exit((arg.exit.unwrap())());
        }
    }
}

pub fn normalize_env_args<'a>(args: &[String], kurisu_args: &[Arg<'a>]) -> Vec<String> {
    let mut env_vars: Vec<String> = vec![];
    let mut previous_flag: String = String::from("");
    let mut options_ended = false;
    for arg in args {
        if arg.len() == 2 && arg == "--" {
            options_ended = true;
        }

        let mut arguments: Vec<String> = vec![arg.clone()];
        // Stacking short flags, check if this is a negative number
        if !options_ended && arg.starts_with('-') && arg.parse::<isize>().is_err() && !arg.contains('=') && !arg.starts_with("--") && arg.len() > 2 {
            arguments = arg.chars().skip(1).map(|a| format!("-{}", a)).collect()
        }

        for arg in arguments {
            let karg = &kurisu_args.iter().find(|a| a == &arg);
            if previous_flag.is_empty() && karg.is_none() {
                env_vars.push(arg.clone());
                continue;
            }

            // Check if this is a negative number
            if arg.parse::<isize>().is_err() && arg.len() > 1 && (arg.starts_with('-') || arg.starts_with("--")) {
                // Two flags following each other
                if !previous_flag.is_empty() {
                    env_vars.push(previous_flag.clone());
                    previous_flag = String::from("");
                }

                if karg.is_some() && karg.unwrap().is_value_none() || arg.contains('=') {
                    env_vars.push(arg.clone());
                    continue;
                }

                previous_flag = arg.clone();
                continue;
            }

            env_vars.push(format!("{}={}", previous_flag, arg));
            previous_flag = String::from("");
        }
    }

    if !previous_flag.is_empty() {
        env_vars.push(previous_flag);
    }

    env_vars
}

pub fn parse_value<P: Parser>(name: &str, info: &Info) -> P {
    // TODO: user parsing if arg type is `fn()` how to call its function, kurisu doc should specify which function to call...
    let arg = info.args.iter().find(|a| name == a.name).unwrap();
    let value = arg.value.join(VALUE_SEPARATOR);
    P::parse(value.as_str())
}

pub fn valid_exit<'a, T: Kurisu<'a>>(_kurisu_struct: &T) {
    let arg_error = validate_usage(_kurisu_struct);
    mayuri::print_usage_error(_kurisu_struct, arg_error);
}

pub fn validate_usage<'a, T: Kurisu<'a>>(_kurisu_struct: &T) -> Option<Error> {
    let info = T::get_info_instance(std::env::args().skip(1).collect()).lock().unwrap();

    if info.env_args.is_empty() && !info.allow_noargs {
        return Some(Error::NoArgs);
    }

    let positions: Vec<i8> = info.args.iter().filter(|a| a.position.is_some()).map(|a| a.position.unwrap()).collect();

    // Always validate invalid options & args first
    let mut pos: i8 = 0;
    for arg in info.env_args.as_slice() {
        if arg.starts_with('-') {
            if !info.args.iter().any(|a| &a == arg) {
                return Some(Error::Invalid(arg.clone()));
            }
        } else {
            pos += 1;
            // If we have an infinite positional args we can never get an invalid pos
            if !positions.contains(&0) && !positions.contains(&pos) {
                return Some(Error::Invalid(arg.clone()));
            }
        }
    }

    // TODO: Validate arg type range, such as usize that cant be negative, etc...
    // TODO: Add a "required_if" annotation to add relationship between args...
    for arg in info.args.iter().filter(|a| a.is_value_required()) {
        if arg.occurrences > 0 && arg.value.is_empty() {
            return Some(Error::RequiresValue(arg.clone()));
        }
    }

    // TODO: Check for conflicting flags, add annotation to denote relationship? --debug & --no-debug
    // TODO: Positional arguments?

    None
}

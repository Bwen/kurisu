mod arg;
mod arg_parser;
mod exit_code;
pub mod mayuri;

pub use arg::Arg;
pub use arg_parser::ArgParser;
pub use exit_code::*;
pub use once_cell::sync::OnceCell;
use std::sync::Mutex;

use crate::arg_parser::VALUE_SEPARATOR;
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

pub fn parse_value<P: ArgParser>(name: &str, info: &Info) -> P {
    // TODO: user parsing if arg type is `fn()` how to call its function, kurisu doc should specify which function to call...
    let arg = info.args.iter().find(|a| name == a.name).unwrap();
    let value = arg.value.join(VALUE_SEPARATOR);
    P::parse(value.as_str())
}

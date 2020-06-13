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
pub trait Kurisu {
    fn from_args(env_args: Vec<String>) -> Self;
    fn get_info_instance(env_args: Vec<String>) -> &'static Mutex<Info<'static>>;
}

#[derive(Debug)]
pub struct Info<'a> {
    pub name: Option<&'a str>,
    pub version: Option<&'a str>,
    pub desc: Option<&'a str>,
    pub doc: Option<&'a str>,
    pub allow_noargs: bool,
    pub env_args: Vec<String>,
    pub args: Vec<Arg<'a>>,
}
impl<'a> Info<'a> {
    pub fn get_positional_args(&'a self) -> Vec<&'a Arg<'a>> {
        self.args.iter().filter(|a| a.position.is_some()).collect()
    }

    pub fn get_flags(&'a self) -> Vec<&'a Arg<'a>> {
        self.args
            .iter()
            .filter(|a| a.is_value_none() && (a.long.is_some() || a.short.is_some()))
            .collect()
    }

    pub fn get_options(&'a self) -> Vec<&'a Arg<'a>> {
        self.args
            .iter()
            .filter(|a| !a.is_value_none() && (a.long.is_some() || a.short.is_some()))
            .collect()
    }
}

pub fn exit_args<E>(info: &Info<'static>, exit: E) -> Option<i32>
where
    E: FnOnce(i32) -> Option<i32>,
{
    let exit_args: Vec<&Arg<'_>> = info
        .args
        .iter()
        .filter(|a| a.exit.is_some() || ["usage", "version"].contains(&a.name))
        .collect();

    for arg in exit_args {
        if arg.occurrences == 0 {
            continue;
        }

        return if arg.name == "version" {
            exit(mayuri::print_version(&info))
        } else if arg.name == "usage" {
            exit(mayuri::print_help(&info))
        } else {
            exit((arg.exit.expect("Infallible"))())
        };
    }

    None
}

pub fn normalize_env_args<'a>(args: &[String], kurisu_args: &[Arg<'a>]) -> Vec<String> {
    let known_short_flags: Vec<&str> = kurisu_args.iter().filter_map(|a| a.short).collect();
    let mut env_vars: Vec<String> = Vec::new();
    let mut previous_flag: String = String::from("");
    let mut options_ended = false;
    for arg in args {
        if arg.len() == 2 && arg == "--" {
            options_ended = true;
        }

        let mut arguments: Vec<String> = vec![arg.clone()];

        // check if this is a negative number
        if !options_ended
            && arg.starts_with('-')
            && arg.len() > 2
            && arg.parse::<isize>().is_err()
            && !arg.contains(',')
            && !arg.contains('=')
            && !arg.starts_with("--")
        {
            arguments = Vec::new();
            let mut unknown_flag = false;
            let mut value = String::from("");
            // Normalize short flag value with no spaces `-iVALUE` as well as stacking short flags
            for short in arg.chars().skip(1) {
                if !unknown_flag && known_short_flags.contains(&short.to_string().as_str()) {
                    arguments.push(format!("-{}", short));
                } else {
                    unknown_flag = true;
                    value.push(short);
                }
            }

            if !value.is_empty() {
                arguments.push(value);
            }
            //arguments = arg.chars().skip(1).map(|a| format!("-{}", a)).collect()
        }

        for arg in arguments {
            let karg = &kurisu_args.iter().find(|a| a == &arg);
            let previous_karg = &kurisu_args.iter().find(|a| a == &previous_flag);
            if previous_flag.is_empty() && karg.is_none() {
                env_vars.push(arg.clone());
                continue;
            }

            // Check for negative numbers
            // FIXME: Known issue if the env_ars contains `"-m", "-4,-5" for a multiple values it is not normalized correctly. See test: normalize_env_args.multiple_comma_values
            if arg.parse::<isize>().is_err() && arg.len() > 1 && (arg.starts_with('-') || arg.starts_with("--")) {
                // Two flags following each other
                if !previous_flag.is_empty() {
                    env_vars.push(previous_flag.clone());
                    previous_flag = String::from("");
                }

                if karg.is_some() && karg.expect("Infallible").is_value_none() || arg.contains('=') {
                    // If we have a comma delimited value and karg support multiple values we split it into multiple args
                    if karg.expect("Infallible").is_value_multiple() && arg.contains(',') {
                        let name: Vec<&str> = arg.split('=').collect();
                        for value in name[1].split(',') {
                            env_vars.push(format!("{}={}", name[0], value));
                        }
                    } else {
                        env_vars.push(arg.clone());
                    }
                    continue;
                }

                previous_flag = arg.clone();
                continue;
            }

            // If we have a comma delimited value and karg support multiple values we split it into multiple args
            if previous_karg.is_some() && previous_karg.expect("Infallible").is_value_multiple() && arg.contains(',') {
                for value in arg.split(',') {
                    env_vars.push(format!("{}={}", previous_flag, value));
                }
            } else {
                env_vars.push(format!("{}={}", previous_flag, arg));
            }

            previous_flag = String::from("");
        }
    }

    if !previous_flag.is_empty() {
        env_vars.push(previous_flag);
    }

    env_vars
}

pub fn parse_value<P: Parser>(name: &str, info: &'_ Info) -> P {
    let arg = info.args.iter().find(|a| name == a.name).expect("Infallible");
    let value = arg.value.join(VALUE_SEPARATOR);
    P::parse(value.as_str())
}

pub fn valid_exit<T: Kurisu>(_kurisu_struct: &T) {
    let arg_error = validate_usage(_kurisu_struct);
    mayuri::print_usage_error(_kurisu_struct, arg_error);
}

pub fn validate_usage<T: Kurisu>(_kurisu_struct: &T) -> Option<Error> {
    // The info instance should always be initialized before validate is called, thus we pass an empty vec
    let info = T::get_info_instance(Vec::new()).lock().unwrap();

    if info.env_args.is_empty() && !info.allow_noargs {
        return Some(Error::NoArgs);
    }

    let positions: Vec<i8> = info
        .args
        .iter()
        .filter(|a| a.position.is_some())
        .map(|a| a.position.expect("Infallible"))
        .collect();

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

    // Validate Position arguments
    for arg in info.args.iter().filter(|a| a.position.is_some()) {
        if arg.value.is_empty() {
            return Some(Error::RequiresPositional(arg.clone()));
        }
    }

    // Validate Options that requires value
    for arg in info.args.iter().filter(|a| a.occurrences > 0) {
        if arg.value.is_empty() {
            return Some(Error::RequiresValue(arg.clone()));
        }
    }

    for arg in info.args.iter().filter(|a| a.required_if.is_some()) {
        let counter_part = info.args.iter().find(|a| a.name == arg.required_if.expect("Infallible"));
        if let Some(counter_part) = counter_part {
            if counter_part.occurrences > 0 && arg.value.is_empty() {
                return Some(Error::RequiresValueIf(counter_part.clone(), arg.clone()));
            }
        }
    }

    // TODO: Validate arg type range, such as usize that cant be negative, etc...

    None
}

//! Command line arguments parser through custom derive macro
//!
//! For full documentation on `derive(Kurisu)`, please see [kurisu_derive](../kurisu_derive/index.html).
//!
//! ## Parsing, Validating and Usage display
//! Kurisu separate these three concepts:
//! ```
//! use kurisu::*;
//!
//! #[derive(Debug, Kurisu)]
//! struct Yargs {
//!     knots: usize,
//! }
//!
//! fn main() {
//!     let env_vars: Vec<String> = std::env::args().skip(1).collect();
//!     # let env_vars: Vec<String> = vec!["--knots=8".to_string()];
//!
//!     // Will take the string values from the command line and try to parse them and assign
//!     // them to the struct's field. If the flag or option is not present then
//!     // its default type value will be assigned to the struct's field.
//!     // In this case: usize::default()  
//!     let args = Yargs::from_args(env_vars);
//!     
//!     // Returns an Option<kurisu::arg::Error> or None
//!     let arg_error = kurisu::validate_usage(&args);
//!
//!     // If an error is present `print_usage_error` will std::process::exit()
//!     // with kurisu::ExitCode::USAGE(64) as exit code
//!     mayuri::print_usage_error(&args, arg_error);
//!     
//!     // Assuming the application was called like so: `mycli --knots 8`
//!     assert_eq!(args.knots, 8);
//! }
//! ```
//! You can shorten this to **_kurisu::[valid_exit](fn.valid_exit.html)(&args)_**
//! which combines **_kurisu::[validate_usage](fn.validate_usage.html)(&args)_** and
//! **_[mayuri](mayuri/index.html)::[print_usage_error](mayuri/fn.print_usage_error.html)(&args)_**.
//!
//! Kurisu tries to have sane defaults for the struct, if we take the following struct as example:
//! ```
//! struct Yargs {
//!     sinking: bool,
//!     knots: usize,
//!     pirate_ship_name: String
//! }
//! ```
//! The field `pirate_ship_name` will have no short option `-p` and only a long
//! option `--pirate-ship-name`. The characters `_` of the field's name will be matched
//! and displayed as `-`. The field `sinking` will be a flag because of its type `bool`.
//! The defaults can be altered through annotation,
//! please see [kurisu_derive](../kurisu_derive/index.html) for more information.
//!
//! Kurisu as specific definitions for Argument, Flag and Option. They are by no mean an official
//! definition, but this is how they are handled within this library.
//!
//! ## Arguments
//! A single word in a command line, example: `mycli myargument`. Also refered to as a positional
//! argument where you can define a specific struct field to a specific argument position.
//!
//! They are never prefixed by either `-` or `--`. Supported struct field types:
//! - `String`,
//! - `PathBuf`,
//! - `usize`,
//! - `isize`,
//! - `f64`,
//! - `bool`,
//!
//! It is possible to define an infinite positional argument where that struct field's value will
//! include all positional arguments (_excluding other defined arguments with specific positions_).
//! The infinite positional argument struct field type is defined by `Vec<T>` and one of the
//! supported types.
//!
//! Arguments are always required. There is no way to make them optional.
//!
//! ## Flags
//! Prefixed by either `-` or `--`, examples: `mycli --my-flag`, `mycli -f`. Their struct field type
//! is always a `bool`. They never have a value associated to them, example: `mycli -f value`,
//! is considered a flag followed by an argument unless `-f` is defined as an option.
//!
//! It is possible to stack short flags, example: `mycli -fBc`.
//!
//! It is also possible to have repeating flags with their occurrences counted,
//! example `mycli -vvv`. In this case the struct field type is a `u8`.
//!
//! Flags are always optional. There is no way to make them required.
//!
//! ## Options
//! Prefixed by either `-` or `--` followed by a value, examples: `mycli --my-option=myvalue`,
//! `mycli -f myvalue`. An option value assignment operator can either be `=` or ` `.
//! They support the same types as arguments.  
//!
//! It is possible to have repeating options, example: `mycli -f one -f=two -f three`,
//! in this case their struct field type is `Vec<T>` with a valid type.
//!
//! Options are always optional by default, but if present their value is always required.
//! It is possible to have an option be required through the annotation `required_if`,
//! for more details see [kurisu_derive](../kurisu_derive/index.html).
//!

#![forbid(unsafe_code)]

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

pub fn valid_exit<T: Kurisu>(kurisu_struct: &T) {
    let arg_error = validate_usage(kurisu_struct);
    mayuri::print_usage_error(kurisu_struct, arg_error);
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

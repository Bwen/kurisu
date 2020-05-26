use crate::{Arg, ExitCode, Info, Kurisu};

#[derive(Debug)]
pub enum ArgError {
    NoArgs,
    Invalid(String),
    RequiresValue(Arg<'static>),
}

pub fn usage_error<'a, T: Kurisu<'a>>(_kurisu_struct: &T, arg_error: Option<ArgError>) {
    if let Some(error) = arg_error {
        let info = T::get_info_instance(std::env::args().skip(1).collect()).lock().unwrap();
        let exit_code = match error {
            ArgError::NoArgs => print_usage(&info),
            ArgError::Invalid(arg) => print_invalid_arg(arg),
            ArgError::RequiresValue(arg) => print_missing_value(arg),
        };

        std::process::exit(exit_code);
    }
}

pub fn print_invalid_arg(arg: String) -> i32 {
    println!("Invalid argument {}", arg);
    ExitCode::USAGE.into()
}

pub fn print_missing_value(arg: Arg) -> i32 {
    // TODO: Interestingly the arg has both short & long but only one was mention in the command line... which to refer to?
    println!("Missing value for {:?}", arg.name);

    ExitCode::USAGE.into()
}

pub fn print_version(info: &Info) -> i32 {
    println!("{} {}", info.name.unwrap_or("Unknown"), info.version.unwrap_or("0"));
    ExitCode::OK.into()
}

pub fn print_usage(info: &Info) -> i32 {
    println!("{} {}", info.name.unwrap_or("Unknown"), info.version.unwrap_or("0"));
    println!("{}", info.doc.unwrap_or(""));
    ExitCode::USAGE.into()
}

pub fn validate_usage<'a, T: Kurisu<'a>>(_kurisu_struct: &T) -> Option<ArgError> {
    let info = T::get_info_instance(std::env::args().skip(1).collect()).lock().unwrap();

    if info.env_args.is_empty() && !info.allow_noargs {
        return Some(ArgError::NoArgs);
    }

    // TODO: Positional arguments?
    // Always validate invalid flags first
    for arg in info.env_args.as_slice() {
        if !arg.starts_with('-') {
            continue;
        }

        if !info.args.iter().any(|a| &a == arg) {
            return Some(ArgError::Invalid(arg.clone()));
        }
    }

    // TODO: Add a "required_if" annotation to add relationship between args...
    for arg in info.args.iter().filter(|a| a.value_required()) {
        if arg.occurrences > 0 {
            return Some(ArgError::RequiresValue(arg.clone()));
        }
    }

    None
}

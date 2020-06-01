use crate::arg::Error;
use crate::{Arg, ExitCode, Info, Kurisu};

pub fn print_usage_error<'a, T: Kurisu<'a>>(_kurisu_struct: &T, arg_error: Option<Error>) {
    if let Some(error) = arg_error {
        let info = T::get_info_instance(std::env::args().skip(1).collect()).lock().unwrap();
        let exit_code = match error {
            Error::NoArgs => print_usage(&info),
            Error::Invalid(arg) => print_invalid_arg(arg),
            Error::RequiresValue(arg) => print_missing_value(arg),
            Error::RequiresValueIf(a, b) => print_missing_value(a),
            Error::Custom(text) => print_custom_error(text),
            Error::CustomArg(arg, text) => print_custom_arg_error(arg, text),
        };

        std::process::exit(exit_code);
    }
}

pub fn print_custom_error(text: String) -> i32 {
    println!("{}", text);
    ExitCode::USAGE.into()
}

pub fn print_custom_arg_error(arg: Arg, text: String) -> i32 {
    println!("Error {}: {}", arg.long.unwrap(), text);
    ExitCode::USAGE.into()
}

pub fn print_invalid_arg(arg: String) -> i32 {
    println!("Invalid argument {}", arg);
    ExitCode::USAGE.into()
}

pub fn print_missing_value(arg: Arg) -> i32 {
    // TODO: Interestingly the arg has both short & long but only one was mentioned in the command line... which to refer to?
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
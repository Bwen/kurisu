use crate::arg::Error;
use crate::{Arg, ExitCode, Info, Kurisu};

pub fn print_usage_error<'a, T: Kurisu<'a>>(_kurisu_struct: &T, arg_error: Option<Error>) {
    if let Some(error) = arg_error {
        let info = T::get_info_instance(std::env::args().skip(1).collect()).lock().unwrap();
        let exit_code = match error {
            Error::NoArgs => print_help(&info),
            Error::Invalid(arg) => print_invalid_arg(arg),
            Error::RequiresValue(arg) => print_missing_value(arg),
            Error::RequiresValueIf(a, _b) => print_missing_value(a),
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

pub fn print_help(info: &Info) -> i32 {
    const TERM_WIDTH: usize = 70;
    let bin_name = info.name.unwrap_or("unknown");
    println!("{} {}", bin_name, info.version.unwrap_or("0"));
    println!("{}", textwrap::fill(info.doc.unwrap_or(""), TERM_WIDTH));

    println!();
    println!("USAGE:");
    println!("{} [OPTIONS] [ARGS]", bin_name);

    let flags: Vec<&Arg> = info
        .args
        .iter()
        .filter(|a| a.is_value_none() && (a.long.is_some() || a.short.is_some()))
        .collect();
    if !flags.is_empty() {
        println!();
        println!("FLAGS:");
        let lines = get_arg_lines(flags, TERM_WIDTH);
        for line in lines {
            println!("{}", line);
        }
    }

    let options: Vec<&Arg> = info
        .args
        .iter()
        .filter(|a| !a.is_value_none() && (a.long.is_some() || a.short.is_some()))
        .collect();

    if !options.is_empty() {
        println!();
        println!("OPTIONS:");
        let lines = get_arg_lines(options, TERM_WIDTH);
        for line in lines {
            println!("{}", line);
        }
    }

    let args: Vec<&Arg> = info.args.iter().filter(|a| a.position.is_some()).collect();
    if !args.is_empty() {
        println!();
        println!("ARGS:");
        let lines = get_arg_lines(args, TERM_WIDTH);
        for line in lines {
            println!("{}", line);
        }
    }

    // println!();
    // println!("SUBCOMMANDS:");

    println!();
    println!("DISCUSSION:");

    ExitCode::USAGE.into()
}

// fn get_args_lines(args: &[Arg], term_width: usize) -> Vec<String> {}

fn get_arg_lines(args: Vec<&Arg>, term_width: usize) -> Vec<String> {
    const DESC_SPACER: &str = "  ";
    const ARG_INDENT: &str = "    ";
    let mut lines: Vec<String> = Vec::new();

    let column1_width = args.iter().map(|a| format!("{}{}", ARG_INDENT, a).len()).max_by(|a, b| a.cmp(b)).unwrap();
    for arg in args {
        let doc = if let Some(doc) = arg.doc { doc } else { "" };
        let column2_width = term_width - column1_width;

        let default = if !arg.default.is_empty() && !arg.is_value_none() {
            format!(" [default: {}]", arg.default)
        } else {
            String::from("")
        };

        // Column width minus 1 because we add a leading space for the other lines of the description
        let desc_wrap = textwrap::fill(format!("{}{}", doc, default).as_str(), column2_width - 1);
        let mut desc = desc_wrap.clone();
        let mut desc_parts: Vec<&str> = vec![];
        if desc_wrap.contains('\n') {
            desc_parts = desc_wrap.split('\n').collect();
            desc = desc_parts.drain(0..1).collect();
        }

        let arg_string = format!("{}{}", ARG_INDENT, arg);
        let mut line = format!("{:width$}{}{}\n", arg_string, DESC_SPACER, desc, width = column1_width);
        for part in desc_parts {
            line = format!("{}{:width$}{} {}\n", line, "", DESC_SPACER, part.trim_start(), width = column1_width);
        }

        lines.push(line.trim_matches('\n').to_string());
    }

    lines
}

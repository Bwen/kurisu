use crate::arg::Error;
use crate::{Arg, ExitCode, Info, Kurisu};
use textwrap::Wrapper;

const DESC_SPACER: &str = "  ";
const ARG_INDENT: &str = "    ";

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
    if let Some(desc) = info.desc {
        println!("{}", textwrap::fill(desc, TERM_WIDTH));
    }

    let args: Vec<&Arg> = info.args.iter().filter(|a| a.position.is_some()).collect();
    let flags: Vec<&Arg> = info
        .args
        .iter()
        .filter(|a| a.is_value_none() && (a.long.is_some() || a.short.is_some()))
        .collect();
    let options: Vec<&Arg> = info
        .args
        .iter()
        .filter(|a| !a.is_value_none() && (a.long.is_some() || a.short.is_some()))
        .collect();

    let usage_args = if args.len() == 1 {
        let first_arg = format!("{}", args[0]);
        format!(" {}", first_arg.trim())
    } else if !args.is_empty() {
        String::from(" [ARGS]")
    } else {
        String::from("")
    };

    let usage_options = if !flags.is_empty() && !options.is_empty() {
        " [OPTIONS]"
    } else {
        " [FLAGS]"
    };

    println!();
    println!("USAGE:");
    println!("{}{}{}{}", ARG_INDENT, bin_name, usage_options, usage_args);

    if !flags.is_empty() {
        println!();
        println!("FLAGS:");
        for line in get_arg_lines(flags, TERM_WIDTH) {
            println!("{}", line);
        }
    }

    if !options.is_empty() {
        println!();
        println!("OPTIONS:");
        for line in get_arg_lines(options, TERM_WIDTH) {
            println!("{}", line);
        }
    }

    if !args.is_empty() {
        println!();
        println!("ARGS:");
        for line in get_arg_lines(args, TERM_WIDTH) {
            println!("{}", line);
        }
    }

    // println!();
    // println!("SUBCOMMANDS:");

    if let Some(doc) = info.doc {
        println!();
        println!("DISCUSSION:");
        let wrapper = Wrapper::new(TERM_WIDTH).initial_indent(ARG_INDENT).subsequent_indent(ARG_INDENT);
        println!("{}", wrapper.wrap(doc).join("\n"));
    }

    ExitCode::USAGE.into()
}

fn get_arg_lines(args: Vec<&Arg>, term_width: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    let column1_width = args.iter().map(|a| format!("{}{}", ARG_INDENT, a).len()).max_by(|a, b| a.cmp(b)).unwrap();
    for arg in args {
        let doc = if let Some(doc) = arg.doc {
            doc.replace("\n", " ")
        } else {
            String::from("")
        };

        let default = if !arg.default.is_empty() && !arg.is_value_none() {
            format!(" [default: {}]", arg.default)
        } else {
            String::from("")
        };

        let arg_string = format!("{}{}", ARG_INDENT, arg);
        let extra_lines_indent = String::from(" ").repeat(DESC_SPACER.len() + column1_width + 1);
        let wrapper = Wrapper::new(term_width).subsequent_indent(extra_lines_indent.as_str());
        let mut line = wrapper
            .wrap(format!("{:width$}{}{}{}", arg_string, DESC_SPACER, doc, default, width = column1_width).as_str())
            .join("\n");

        line = line.trim_matches('\n').to_string();
        if arg.position.is_some() {
            line = format!("{}{}", ARG_INDENT, line.trim().to_string());
        }
        lines.push(line);
    }

    lines
}

use crate::arg::Error;
use crate::{Arg, ExitCode, Info, Kurisu};
use std::cmp::Ordering;
use textwrap::Wrapper;

// TODO: Add test for display layer...
const DESC_SPACER: &str = "  ";
const ARG_INDENT: &str = "    ";

pub fn print_usage_error<'a, T: Kurisu>(_kurisu_struct: &T, arg_error: Option<Error>) {
    if let Some(error) = arg_error {
        let info = T::get_info_instance(std::env::args().skip(1).collect()).lock().unwrap();
        let exit_code = match error {
            Error::NoArgs => print_help(&info),
            Error::Invalid(arg) => print_invalid_arg(arg, &info),
            Error::RequiresPositional(arg) => print_missing_positional(arg, &info),
            Error::RequiresValue(arg) => print_missing_value(arg, &info),
            Error::RequiresValueIf(a, _b) => print_missing_value(a, &info),
            Error::Custom(text) => print_custom_error(text, &info),
            Error::CustomArg(arg, text) => print_custom_arg_error(arg, text, &info),
        };

        std::process::exit(exit_code);
    }
}

pub fn print_custom_error(text: String, info: &Info) -> i32 {
    print_error_text(&info, text);
    print_usage(&info);
    print_more_info();
    ExitCode::USAGE.into()
}

pub fn print_custom_arg_error(arg: Arg, text: String, info: &Info) -> i32 {
    let arg_string = format!("{}", arg);
    print_error_text(&info, format!("{} {}", arg_string.trim(), text));
    print_usage(&info);
    print_more_info();
    ExitCode::USAGE.into()
}

pub fn print_invalid_arg(arg: String, info: &Info) -> i32 {
    let error = format!("unrecognized option {}", arg);
    print_error_text(&info, error);
    print_usage(&info);
    print_more_info();
    ExitCode::USAGE.into()
}

pub fn print_missing_positional(arg: Arg, info: &Info) -> i32 {
    let arg_string = format!("{}", arg);
    let error = format!("missing argument {}", arg_string.trim());
    print_error_text(&info, error);
    print_usage(&info);
    print_more_info();
    ExitCode::USAGE.into()
}

pub fn print_missing_value(arg: Arg, info: &Info) -> i32 {
    let arg_string = format!("{}", arg);
    let error = format!("missing value for option {}", arg_string.trim());
    print_error_text(&info, error);
    print_usage(&info);
    print_more_info();
    ExitCode::USAGE.into()
}

pub fn print_version(info: &Info) -> i32 {
    println!("{} {}", info.name.unwrap_or("Unknown"), info.version.unwrap_or("0"));
    ExitCode::OK.into()
}

pub fn print_help(info: &Info) -> i32 {
    let terminal_width = Wrapper::with_termwidth().width;

    let bin_name = info.name.unwrap_or("unknown");
    println!("{} {}", bin_name, info.version.unwrap_or("0"));
    if let Some(desc) = info.desc {
        println!("{}", textwrap::fill(desc, terminal_width));
    }

    print_usage(&info);

    let flags: Vec<&Arg> = info.get_flags();
    if !flags.is_empty() {
        println!();
        println!("FLAGS:");
        for line in get_arg_usage_lines(flags, terminal_width) {
            println!("{}", line);
        }
    }

    let options: Vec<&Arg> = info.get_options();
    if !options.is_empty() {
        println!();
        println!("OPTIONS:");
        for line in get_arg_usage_lines(options, terminal_width) {
            println!("{}", line);
        }
    }

    let args: Vec<&Arg> = info.get_positional_args();
    if !args.is_empty() {
        println!();
        println!("ARGS:");
        for line in get_arg_usage_lines(args, terminal_width) {
            println!("{}", line);
        }
    }

    let subcommands: Vec<&Arg> = info.get_subcommands();
    if !subcommands.is_empty() {
        println!();
        println!("SUBCOMMANDS:");
        for line in get_arg_usage_lines(subcommands, TERM_WIDTH) {
            println!("{}", line);
        }
    }

    if let Some(doc) = info.doc {
        println!();
        println!("DISCUSSION:");
        let wrapper = Wrapper::new(terminal_width).initial_indent(ARG_INDENT).subsequent_indent(ARG_INDENT);
        println!("{}", wrapper.wrap(doc).join("\n"));
    }

    ExitCode::USAGE.into()
}

fn print_error_text(info: &Info, error: String) {
    println!("{}: {}", info.name.unwrap_or("Unknown"), error);
}

fn print_more_info() {
    println!();
    println!("For more information try --help");
}

fn print_usage(info: &Info) {
    let args: Vec<&Arg> = info.get_positional_args();
    let flags: Vec<&Arg> = info.get_flags();
    let options: Vec<&Arg> = info.get_options();

    let usage_args = if args.len() == 1 {
        let first_arg = format!("{}", args[0]);
        format!(" {}", first_arg.trim())
    } else if !args.is_empty() {
        let mut position_args: Vec<(i8, String)> = Vec::new();
        for arg in args {
            if let Some(pos) = arg.position {
                position_args.push((pos, format!("{}", arg)));
            }
        }

        position_args.sort_by(|a, b| {
            // Infinite argument always goes last
            if a.0 == 0 {
                return Ordering::Greater;
            }

            a.0.cmp(&b.0)
        });
        let ordered_args: Vec<&str> = position_args.iter().map(|a| a.1.trim().as_ref()).collect();
        format!(" {}", ordered_args.join(" "))
    } else {
        String::from("")
    };

    let all_shorts: Vec<&Arg> = info.args.iter().filter(|a| a.short.is_some()).collect();
    let all_shorts_options: Vec<&Arg> = info.args.iter().filter(|a| a.short.is_some() || a.long.is_some()).collect();
    let usage_options = if all_shorts.len() == all_shorts_options.len() {
        // if we have only shorts we stack them
        let flag_stack: Vec<&str> = all_shorts_options.iter().map(|a| a.short.expect("Infallible")).collect();
        format!(" [-{}]", flag_stack.join(""))
    } else if !flags.is_empty() && !options.is_empty() {
        String::from(" [FLAGS | OPTIONS]")
    } else {
        String::from(" [FLAGS]")
    };

    let bin_name = info.name.unwrap_or("unknown");

    println!();
    println!("USAGE:");
    println!("{}{}{}{}", ARG_INDENT, bin_name, usage_options, usage_args);
}

fn get_arg_usage_lines(args: Vec<&Arg>, term_width: usize) -> Vec<String> {
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
        let extra_lines_indent = String::from(" ").repeat(DESC_SPACER.len() + column1_width + 2);
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

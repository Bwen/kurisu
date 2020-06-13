mod commands;
use commands::{Create, Delete};

use kurisu::*;

pub enum Command {
    Create(Create),
    Delete(Delete),
}

fn parse_command(name: &str, info: &'_ Info) -> Option<Command> {
    let arg = info.args.iter().find(|a| name == a.name).expect("Infallible");
    println!("----- {:?}", arg);
    match arg.value[0].as_str() {
        "create" => Some(Command::Create(Create::from_args(std::env::args().skip(1).collect()))),
        "delete" => Some(Command::Delete(Delete::from_args(std::env::args().skip(1).collect()))),
        _ => None,
    }
}

#[derive(Kurisu)]
struct Yargs {
    #[kurisu(subcommand, pos = 1, parser = "parse_command")]
    action: Option<Command>,
}

fn main() {
    let args = Yargs::from_args(std::env::args().skip(1).collect());

    match args.action {
        Some(Command::Create(ref command)) => exec_create(command),
        Some(Command::Delete(ref command)) => exec_delete(command),
        None => {
            println!("Test");
        }
    }

    // kurisu::valid_exit(&args);
    println!("Win!");
}

fn exec_create(command: &Create) {
    kurisu::valid_exit(command);
    println!("##### {:?}", command.name1);
}

fn exec_delete(command: &Delete) {
    kurisu::valid_exit(command);
}

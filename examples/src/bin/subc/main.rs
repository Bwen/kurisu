mod commands;
use commands::{Create, Delete};

use kurisu::*;

pub enum Command {
    Empty,
    Create(Create),
    Delete(Delete),
}

fn parse_command(name: &str, info: &'_ Info) -> Command {
    let arg = info.args.iter().find(|a| name == a.name).expect("Infallible");
    if arg.value.is_empty() {
        return Command::Empty;
    }

    match arg.value[0].as_str() {
        "create" => Command::Create(Create::from_args(std::env::args().skip(1).collect())),
        "delete" => Command::Delete(Delete::from_args(std::env::args().skip(1).collect())),
        _ => Command::Empty,
    }
}

#[derive(Kurisu)]
struct Yargs {
    test: bool,
    #[kurisu(subcommand, pos = 1, parser = "parse_command")]
    action: Command,
}

fn main() {
    let env_args = std::env::args().skip(1).collect();
    let args = Yargs {
        ..Yargs::from_args(env_args)
    };

    match args.action {
        Command::Create(ref command) => exec_create(command),
        Command::Delete(ref command) => exec_delete(command),
        Command::Empty => kurisu::valid_exit(&args),
    }

    println!("Win!");
}

fn exec_create(command: &Create) {
    kurisu::valid_exit(command);
    println!("##### {:?}", command.name1);
}

fn exec_delete(command: &Delete) {
    kurisu::valid_exit(command);
}

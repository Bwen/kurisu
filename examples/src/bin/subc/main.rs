mod commands;
use commands::{Create, Delete};

use kurisu::*;

pub enum Command {
    Create(Create),
    Delete(Delete),
}

fn parse_command(name: &str, info: &'_ Info) -> Option<Command> {
    let arg = info.args.iter().find(|a| name == a.name).expect("Infallible");
    if arg.value.is_empty() {
        return None;
    }

    let env_vars = std::env::args().skip(1).collect();
    match arg.value[0].as_str() {
        "create" => Some(Command::Create(Create::from_args(env_vars))),
        "delete" => Some(Command::Delete(Delete::from_args(env_vars))),
        _ => None,
    }
}

#[derive(Kurisu)]
struct Yargs {
    test: bool,
    #[kurisu(subcommand, pos = 1, parser = "parse_command")]
    /// Create new thingies!
    create: Option<Command>,
    #[kurisu(subcommand, pos = 1, parser = "parse_command")]
    /// Delete things, because we no longer like them...
    delete: Option<Command>,
}

fn main() {
    let env_args = std::env::args().skip(1).collect();
    let args = Yargs {
        ..Yargs::from_args(env_args)
    };

    if let Some(Command::Create(ref command)) = args.create {
        exec_create(command);
    } else if let Some(Command::Delete(ref command)) = args.delete {
        exec_delete(command);
    } else {
        kurisu::valid_exit(&args);
    }

    println!("Win!");
}

fn exec_create(command: &Create) {
    kurisu::valid_exit(command);
    println!("#####C {:?}", command.name1);
}

fn exec_delete(command: &Delete) {
    kurisu::valid_exit(command);
    println!("#####D {:?}", command.name2);
}

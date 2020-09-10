mod commands;
use commands::{Create, Delete};

use kurisu::*;

#[derive(Debug)]
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
        _ => {
            println!("# Sub Command NONE!?");
            None
        }
    }
}

fn exec_create(command: &Create) {
    println!("#####C {:?}", command.name1);
}

fn exec_delete(command: &Delete) {
    println!("#####D {:?}", command.name2);
}

#[derive(Kurisu)]
struct Yargs {
    test: bool,
    #[kurisu(subcommand, pos = 1, parse_with = "parse_command")]
    /// Create new thingies!
    create: Option<Command>,
    #[kurisu(subcommand, pos = 1, parse_with = "parse_command")]
    /// Delete things, because we no longer like them...
    delete: Option<Command>,
}

fn main() {
    let env_args = std::env::args().skip(1).collect();
    let args = Yargs {
        ..Yargs::from_args(env_args)
    };

    println!("# Create: {:?}", args.create);
    if let Some(Command::Create(ref command)) = args.create {
        println!("# bobby jones! 1");
        kurisu::valid_exit(command);
        exec_create(command);
    } else if let Some(Command::Delete(ref command)) = args.delete {
        println!("# bobby jones! 2");
        kurisu::valid_exit(command);
        exec_delete(command);
    } else {
        println!("# bobby jones! 3");
        kurisu::valid_exit(&args);
    }

    println!("Win!");
}

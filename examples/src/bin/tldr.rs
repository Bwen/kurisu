use kurisu::*;
use std::path::PathBuf;

#[derive(Debug, Kurisu)]
#[kurisu(name = "tldr", version = "1.0.2", desc = "Tool Long Didnt Read Example", auto_shorts)]
/// some helpful text, tuturu ♫
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫1
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫2
///
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫3
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫4
///
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫5
struct Yargs {
    /// This is to test1 long documentation problem that could occur at any time...
    /// This is to test2 long documentation problem that could occur at any time...
    #[kurisu(vname = "potatoe")]
    test: Vec<String>,
    #[kurisu(pos = 2)]
    /// The target directory plz
    target_dir: PathBuf,
    #[kurisu(pos = 1)]
    /// The source directory plz
    source_dir: PathBuf,
    #[kurisu(exit = "my_func")]
    zob: bool,
    #[kurisu(short, nolong, default = "203")]
    /// Blah blah blog
    short: usize,
    #[kurisu(short, parser = "parse_bobby")]
    bobby: String,
}

pub fn my_func() -> i32 {
    println!("I AM exiting early, thx!");

    ExitCode::OK.into()
}

pub fn parse_bobby(name: &str, info: &'_ Info) -> String {
    let arg = info.args.iter().find(|a| name == a.name).expect("Infallible");
    if arg.value.is_empty() {
        return String::from("");
    }

    arg.value[0].to_uppercase()
}

fn main() {
    let args = Yargs::from_args(std::env::args().skip(1).collect());
    kurisu::valid_exit(&args);

    // println!("{:?}", args.source_dir.exists());
    // println!("{:?}", args);
    println!("Win: {:?}", args.bobby);
}

use kurisu::*;
use std::path::PathBuf;

#[derive(Debug, Kurisu)]
#[kurisu(name = "tldr", version = "1.0.2", desc = "Tool Long Didnt Read Example")]
/// some helpful text, tuturu ♫
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫1
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫2
///
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫3
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫4
///
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫5
struct Yargs {
    #[kurisu(long = "long")]
    /// This is to test1 long documentation problem that could occur at any time...
    /// This is to test2 long documentation problem that could occur at any time...
    #[kurisu(vname = "potatoe")]
    test: Vec<String>,
    #[kurisu(pos = 1)]
    /// The source directory plz
    source_dir: Vec<PathBuf>,
    // #[kurisu(pos = 2)]
    // target_dir: PathBuf,
    #[kurisu(exit = "my_func")]
    bob: bool,
    #[kurisu(short, nolong, default = "203")]
    /// Blah blah blog
    short: usize,
}

pub fn my_func() -> i32 {
    println!("I AM exiting DA SHIAT OUTTA THIS NIGGA!");

    ExitCode::OK.into()
}

fn main() {
    let args = Yargs::from_args(std::env::args().skip(1).collect());
    kurisu::valid_exit(&args);

    // println!("{:?}", args.source_dir.exists());
    // println!("{:?}", args);
}

use kurisu::*;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Kurisu)]
/// some helpful text, tuturu ♫
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫
/// tuturu ♫ tuturu ♫ tuturu ♫ tuturu ♫
#[kurisu]
struct Yargs {
    #[kurisu(long = "very-long-option")]
    /// This is to test long documentation problem that could occur at any time...
    /// This is to test long documentation problem that could occur at any time...
    test: String,
    #[kurisu(pos = 1)]
    source_dir: PathBuf,
    // #[kurisu(pos = 2)]
    // target_dir: PathBuf,
    #[kurisu(exit = "my_func")]
    bob: bool,
}

pub fn my_func() -> i32 {
    println!("I AM exiting DA SHIAT OUTTA THIS NIGGA!");

    ExitCode::OK.into()
}

fn main() {
    let args = Yargs::from_args(std::env::args().skip(1).collect());
    kurisu::valid_exit(&args);

    println!("{:?}", args.source_dir.exists());
    println!("{:?}", args);
}

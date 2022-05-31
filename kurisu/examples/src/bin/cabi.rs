use kurisu::*;

#[derive(Debug, Kurisu)]
#[kurisu(name = "cabi", version = "1.0.2", desc = "infinite arguments", auto_shorts)]
struct Yargs {
    #[kurisu(pos = 1)]
    subcommand: String,
    #[kurisu(pos)]
    args: String,
}

fn main() {
    let args = Yargs::from_args(std::env::args().skip(1).collect());
    kurisu::valid_exit(&args);

    println!("Cabi args: {:?}", args);
}

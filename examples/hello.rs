use clap::Parser;
use samples::say_hello;

fn main() {
    let args = Args::parse();

    let greeting = say_hello(args.name);
    println!("{greeting}");
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Optional name to greet.
    #[clap(default_value = "samplesaurus")]
    name: String,
}

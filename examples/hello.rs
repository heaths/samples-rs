use clap::Parser;
use samples::{say_hello, CurrentUser};

fn main() {
    let args = Args::parse();

    let who = args.name.unwrap_or_else(|| {
        CurrentUser::try_new().map_or_else(|_| "samplesaurus".to_string(), |user| user.to_string())
    });
    let greeting = say_hello(who);
    println!("{greeting}");
}

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Optional name to greet.
    /// The default is the current username or "samplesaurus".
    name: Option<String>,
}

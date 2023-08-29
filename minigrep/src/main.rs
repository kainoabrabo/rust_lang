use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem with passing arguments: {err}");
        process::exit(1);
    });

    println!("Str: {}, File: {}", config.s, config.f);

    if let Err(e) = minigrep::run(config) {
        println!("Application Error: {e}");
        process::exit(1);
    };
}
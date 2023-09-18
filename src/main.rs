use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem causing args: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

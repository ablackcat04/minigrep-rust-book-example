use std::{env, process};

use minigrep::Config;

fn main() {
    let config = Config::build(env::args().into_iter()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

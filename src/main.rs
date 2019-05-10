use std::env;
use std::process;

mod lib;

use lib::{ self as minigrep, Config };

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|msg| {
        eprintln!("Problem parsing arguments: {}", msg);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

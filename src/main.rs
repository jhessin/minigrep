use std::env;
use std::process;

mod lib;

use lib::{ self as minigrep, Config };

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|msg| {
        println!("Problem parsing arguments: {}", msg);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

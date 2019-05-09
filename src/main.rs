use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|msg| {
        println!("Problem parsing arguments: {}", msg);
        process::exit(1);
    });

    println!(
        "Searching for {}\nIn file {}",
        config.query, config.filename
    );

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

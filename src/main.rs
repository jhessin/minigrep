use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match Config::new(&args) {
        Err(msg) => {
            println!("{}", msg);
        },
        Ok(config) => {
            println!(
                "Searching for {}\nIn file {}",
                config.query, config.filename
            );
            let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
            println!("With text: \n{}", contents);
        },
    }

}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

use std::env;
use std::process;

use io_tutorial::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for \"{}\" in:", config.query);
    println!("{:?}", config.filename);

    if let Err(e) = io_tutorial::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

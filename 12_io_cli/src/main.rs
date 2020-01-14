use std::env;
use std::process;

use chapter_12_io_cli::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else is used for error handling.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // this is much better error messaging for users
        // than the compiler's stock developer debug messages.

        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if-let is used instead of unwrap_or_else() for error checking
    // because the fn would only return the unit type () .

    if let Err(err) = chapter_12_io_cli::run(config) {
        eprintln!("Application Error: {}", err);

        process::exit(1);
    }
}

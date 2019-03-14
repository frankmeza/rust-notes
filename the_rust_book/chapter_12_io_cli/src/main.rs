use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // this is the &'static str in returned Result
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// aside: () is the unit type
// Box<dyn Error> returns a type that impl Error trait,
// to allow all kinds of Error. All kinds.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? returns the value from the current fn to caller
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    // This Ok(()) syntax might look a bit strange at first, but using () like this
    // is the idiomatic way to indicate that we’re calling run for its side effects only;
    // it doesn’t return a value we need.
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else is used for error handling
    let config = Config::new(&args).unwrap_or_else(|err| {
        // this is much better error messaging for users
        // than the compiler's stock developer debug messages
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

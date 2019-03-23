// The run function definition
// The relevant use statements
// The definition of Config
// The Config::new function definition

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? returns the value from the current fn to caller.
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);

    }

    // This Ok(()) syntax might look a bit strange at first, but using ()
    // like this is the idiomatic way to indicate that we’re calling run
    // for its side effects only; it doesn’t return a value we need.
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    // this is the rest of the production code in this file
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive"],
            search(query, contents)
        );
    }
}
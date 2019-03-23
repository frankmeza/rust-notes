// The run function definition
// The relevant use statements
// The definition of Config
// The Config::new function definition

use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // this is the &'static str in returned Result
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
// aside: () is the unit type
// Box<dyn Error> returns a type that impl Error trait,
// to allow all kinds of Error. All kinds.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? returns the value from the current fn to caller.
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // query is now a String, to_lowercase() creates new data
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // bc query is a String, we now pass a ref to it, as &query
        if line.to_lowercase().contains(&query) {
            results.push(line);
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
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

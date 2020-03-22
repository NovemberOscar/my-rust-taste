use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self};

pub struct Config {
    pub query: String,
    pub filename: Option<String>,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename: Option<String> = match args.get(2) {
            Some(filename) => Some(filename.clone()),
            None => None,
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match config.filename {
        Some(filename) => read_from_file(filename)?,
        None => read_from_stdin()?,
    };

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn read_from_file(filename: String) -> Result<String, Box<dyn Error>> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn read_from_stdin() -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    stdin.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}

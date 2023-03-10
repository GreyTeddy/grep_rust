#![allow(dead_code)]
use std::{env, error::Error, fs, path::Path};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build() -> Config {
        // get an iterator and only check for the first two arguments
        // to save on memory
        let mut iterator = env::args().skip(1);
        let query = iterator.next().unwrap_or_else(|| {
            eprintln!("Please specify filename and query."); 
            std::process::exit(1);
        });
        let file_path = iterator.next().unwrap_or_else(|| {
            eprintln!("Please specify filename."); 
            std::process::exit(1);
        });

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        if !Path::new(&file_path).exists() {
            eprintln!("{file_path}: File not found.");
            std::process::exit(1);
        }

        Config {
            query,
            file_path,
            ignore_case,
        }
    }
}
pub fn content(config: Config) -> String {
    let contents =
        fs::read_to_string(config.file_path).unwrap_or_else(|err| {
            eprintln!("Not able to read the file: {err}"); 
            std::process::exit(1);
        });
    contents
}

pub fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        println!("ignoring case\n");
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = String::from("duct");
        let contents = &String::from("Rust:\nsafe, fast, productive.\nPick three.");

        let config = Config {
            query: query,
            file_path: String::from("hello"),
            ignore_case: false,
        };

        assert_eq!(
            vec!["safe, fast, productive."],
            search(&config.query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = String::from("duct");
        let contents = &String::from("Rust:\nsafe, fast, productive.\nPick three\nDuct tape.");
        let config = Config {
            query: query,
            file_path: String::from("hello"),
            ignore_case: false,
        };
        assert_eq!(
            vec!["safe, fast, productive."],
            search(&config.query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = String::from("rUsT");
        let contents = &String::from("Rust:\nsafe, fast, productive.\nPick three.\nTrust me.");

        let config = Config {
            query: query,
            file_path: String::from("hello"),
            ignore_case: true,
        };
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&config.query, contents)
        );
    }
}

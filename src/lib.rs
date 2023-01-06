#![allow(dead_code)]
use std::{env, path::Path, fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build() -> Config {
        // get an iterator and only check for the first two arguments
        // to save on memory
        let mut iterator = env::args().skip(1);
        let query = iterator.next()
            .expect("No filename or query specified");
        let file_path = iterator.next()
            .expect("No filename specified");

        if !Path::new(&file_path).exists() {
            panic!("File not found");
        }

        Config {query, file_path}
    }



}
pub fn content(config: Config) -> String {
    let contents = fs::read_to_string(config.file_path)
            .expect("Should have been able to read the file");
    contents
}

pub fn search<'a>(query: &String, contents: &'a String) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
 
    for line in search(&config.query, &contents) {
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

        let config = Config { query: query, file_path: String::from("hello")};


        assert_eq!(vec!["safe, fast, productive."], search(&config.query,contents));
    }
}
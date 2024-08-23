use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // if args.len() < 3 {
        //     return Err("Not enough arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();

        // let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if !config.ignore_case {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search_case_intensive(&config.query, &contents) {
            println!("{line}");
        }
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

fn search_case_intensive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "rodu";
        let query2 = "ick";
        let query3 = "not_in_the_content";
        let empty_vec:Vec<&str> = Vec::new();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Shit I'm sick now.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        assert_eq!(vec!["Pick three.", "Shit I'm sick now."], search(query2, contents));
        assert_eq!(empty_vec, search(query3, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_intensive(query, contents));
    }
}
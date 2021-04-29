use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // ! I do not understand why we need `static lifetime for the error...
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
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
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    // search(config.query, contents)
    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let results: Vec<&str> = contents
        .lines()
        .flat_map(|line| {
            if line.to_lowercase().contains(&query) {
                return Some(line);
            }
            None
        })
        .collect();
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let config: Config = Config {
            query: String::from("ehh"),
            filename: String::from("poem.txt"),
            case_sensitive: true,
        };
        if let Err(err) = run(config) {
            panic!("{}", err)
        }
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        let config: Config = Config {
            query: String::from("ehh"),
            filename: String::from("poems.txt"),
            case_sensitive: true,
        };
        if let Err(err) = run(config) {
            panic!("{}", err)
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "RUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

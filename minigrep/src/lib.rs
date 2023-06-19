use greptools::Config;

pub mod greptools {
    use std::result::Result;
    use std::{env, fs};

    pub struct Config {
        pub query: String,
        pub filepath: String,
        pub content: String,
        pub ignore_case: bool,
    }

    impl Config {
        pub fn new(query: String, filepath: String, content: String, ignore_case: bool) -> Self {
            Self {
                query: query,
                filepath: filepath,
                content: content,
                ignore_case: ignore_case,
            }
        }

        pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("Not enough Arguments");
            }
            let content: String = match fs::read_to_string(&args[2]) {
                Ok(content) => content,
                Err(_) => String::from("Failed to read content"),
            };
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok(Config {
                query: args[1].to_string(),
                filepath: args[2].to_string(),
                content: content,
                ignore_case: ignore_case,
            })
        }
    }

    pub fn parse_config(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough Arguments");
        }
        let query: &str = &args[1];
        let filepath: &str = &args[2];
        let content: String = match fs::read_to_string(&filepath) {
            Ok(content) => content,
            Err(_) => String::from("Failed to read content"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config::new(
            query.to_string(),
            filepath.to_string(),
            content,
            ignore_case,
        ))
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let args = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        match greptools::parse_config(&args) {
            Ok(config) => assert_eq!(config.filepath, args[2]),
            Err(err) => panic!("Certain error has occurred! {}", err),
        }
    }

    #[test]
    fn test_search_case_insensitive() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        let args = vec![
            "I am very cool".to_string(),
            "duct".to_string(),
            contents.to_string(),
        ];
        let config = Config::new(args[1].clone(), args[0].clone(), args[2].clone(), true);
        let list = search_case_insensitive(config);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_search_case_sensitive() {
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";
        let args = vec![
            "I am very cool".to_string(),
            "duct".to_string(),
            contents.to_string(),
        ];
        let config = Config::new(args[1].clone(), args[0].clone(), args[2].clone(), false);
        let list = search_case_sensitive(config);
        assert_eq!(list.len(), 1);
    }
}

pub fn search_case_insensitive(config: Config) -> Vec<String> {
    let mut finds: Vec<String> = Vec::new();
    for line in config.content.lines() {
        if line.to_lowercase().contains(&config.query) {
            finds.push(line.to_lowercase().to_string());
        };
    }
    finds
}

pub fn search_case_sensitive(config: Config) -> Vec<String> {
    let mut finds: Vec<String> = Vec::new();
    for line in config.content.lines() {
        if line.contains(&config.query) {
            finds.push(line.to_string());
        };
    }
    finds
}

pub fn run(config: Config) {
    if config.ignore_case {
        let finds: Vec<String> = search_case_insensitive(config);
        for line in finds {
            println!("{line}");
        }
    } else {
        let finds: Vec<String> = search_case_sensitive(config);
        for line in finds {
            println!("{line}");
        }
    }
}

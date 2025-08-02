use std::fs;
use std::error::Error;
use std::env;

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    if config.count_only {
        let count = if config.case_sensitive {
            search(&config.query, &contents).len()
        } else {
            search_case_insensitive(&config.query, &contents).len()
        };
        println!("{}", count);
    } else if config.line_numbers {
        let results = if config.case_sensitive {
            search_with_line_numbers(&config.query, &contents)
        } else {
            search_case_insensitive_with_line_numbers(&config.query, &contents)
        };

        for (line_num, line) in results {
            println!("{}:{}", line_num, line);
        }
    } else {
        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub fn search_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents.lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

pub fn search_case_insensitive_with_line_numbers<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    contents.lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .map(|(i, line)| (i + 1, line))
        .collect()
}

pub struct Config{
    pub query:String,
    pub filename:String,
    pub case_sensitive: bool,
    pub line_numbers: bool,
    pub count_only: bool,
}

impl Config{
    pub fn new(args:&[String])->Result<Config,&str>{
        
        if args.len()<3{
            return Err("Not enough arguments");
        }
        
        let query=args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let line_numbers = env::var("LINE_NUMBERS").is_ok();
        let count_only = env::var("COUNT_ONLY").is_ok();

        Ok(Config{query,filename, case_sensitive, line_numbers, count_only})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
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

    #[test]
    fn search_with_line_numbers_test() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search_with_line_numbers(query, contents));
    }

    #[test]
    fn search_case_insensitive_with_line_numbers_test() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_case_insensitive_with_line_numbers(query, contents)
        );
    }
}
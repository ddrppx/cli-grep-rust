// Modules
use std::fs;
use std::error::Error;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Aquiring contents of the read file
    let contents: String = fs::read_to_string(config.filename)?;

    // Dealing with case insensitivenes
    let results = if config.case_sensitive {
        // Search case sensitive
        search(&config.query, &contents)
    } else {
        // Search not case sensitive
        search_case_insensitive(&config.query, &contents)
    };

    // Output
    // Printing that matches query
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// Destructing tuple (From parse_config)
// Connecting returned values from parse_config to query/filename
pub struct Config {
    pub query: String,
    pub filename: String,
    // Case sensitive search or not
    pub case_sensitive: bool,
}

// Coupling of Config struct and "parse_config" -> "new" function
impl Config {
    // Argument parsing

    // &[Strings] = Reference to an array of strings
    // '->' = Returns 
    // new = convention for constructor function
    // Returning Result type and Ok() to kill more noise when executing with no args
    pub fn new(args: &[String]) -> Result<Config, &str>{
        
        // Arg check 
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // Argument with the text to search
        // Index 1. Because args[0] = bin path
        let query: String = args[1].clone();

        // Argument with the filename
        let filename: String = args[2].clone();

        // If the environment variable CASE_INSENSITIVE is not set will get an error the 
        // variable case_sensitive will get 'false'
        // otherwise, 'true'.

        // on bash run: 
            // 'export CASE_INSENSITIVE=true' to set
            // 'unset CASE_INSENSITIVE' to unset.
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        // Passing variables to Config
        Ok(Config { query, filename, case_sensitive })
    }
}

// 'a = Strings and returned vector lifetime tied to the contents input parameter
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Results vector
    let mut results: Vec<&str> = Vec::new();

    // Looping through the contents
    for line in contents.lines() {
        // Checking if the line has the query
        if line.contains(query){
            // Pushes found string into the results vector
            results.push(line);
        }
    }
    // Returning the results vector
    results
}   

// Search for case insensitive
// Makes the needle and the haystack lowercase
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // Query search to lowercase
    let query: String = query.to_lowercase();
    let mut results: Vec <&str> = Vec::new();

    for line in contents.lines() {
        // Line to lowercase to match query
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    // Return results vector
    results
}

// TDD
#[cfg(test)]
mod tests{
    use super::*;

    // Test script
    #[test]
    fn case_sensitive() {
        // Text to search - needle
        let query: &str = "duct";
        // Text to search on - haystack
        // Indenting will include spaces and make the test fail
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        // Calls search passing query and contents
        // Expecting lines with the query string
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        // Text to search - needle
        let query: &str = "rUST";
        // Text to search on - haystack
        // Indenting will include spaces and make the test fail
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        // Calls search passing query and contents
        // Expecting lines with the query string
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}



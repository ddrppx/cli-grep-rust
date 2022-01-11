// Modules
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Aquiring contents of the read file
    let contents: String = fs::read_to_string(config.filename)?;

    // Output
    // Printing the lines containing the search string
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// Destructing tuple (From parse_config)
// Connecting returned values from parse_config to query/filename
pub struct Config {
    pub query: String,
    pub filename: String,
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

        Ok(Config { query, filename })
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

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str>{
    vec![]
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
Pick three.";

        // Calls search passing query and contents
        // Expecting lines with the query string
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}



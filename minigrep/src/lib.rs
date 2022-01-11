// Modules
use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Aquiring contents of the read file
    let contents: String = fs::read_to_string(config.filename)?;

    // Output
    println!("With text: \n{}", contents);

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}   

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}



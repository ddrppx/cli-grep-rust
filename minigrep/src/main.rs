// Modules
use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // Collecting args from cli
    let args: Vec<String> = env::args().collect();
    
    // Calling the argument parsing function
    // Destructing the returned tuple into two variables
    // unwrap_or_else = 
        // Ok case: returns the value in ok 
        // in the error case: executes closure passing it the error
    let config: Config = Config::new(&args).unwrap_or_else( |err| {
        // Printing error
        println!("Problem parsing arguments: {}", err);
        // Exits with status code 1
        process::exit(1);
    });

    // printing out the arguments
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // Checking if run() returns an error variant
    if let Err(e) = run(config){
        println!("Application error: {}", e);
        // Exits with status code 1
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Aquiring contents of the read file
    let contents: String = fs::read_to_string(config.filename)?;

    // Output
    println!("With text: \n{}", contents);

    Ok(())
}

// Destructing tuple (From parse_config)
// Connecting returned values from parse_config to query/filename
struct Config {
    query: String,
    filename: String,
}

// Coupling of Config struct and "parse_config" -> "new" function
impl Config {
    // Argument parsing

    // &[Strings] = Reference to an array of strings
    // '->' = Returns 
    // new = convention for constructor function
    // Returning Result type and Ok() to kill more noise when executing with no args
    fn new(args: &[String]) -> Result<Config, &str>{
        
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



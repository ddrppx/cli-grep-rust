// Modules
use std::env;
use std::fs;

fn main() {
    // Collecting args from cli
    let args: Vec<String> = env::args().collect();
    
    // Calling the argument parsing function
    // Destructing the returned tuple into two variables
    let config: Config = parse_config(&args);

    // printing out the arguments
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // Aquiring contents of the read file
    let contents: String = fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // Output
    println!("With text: \n{}", contents);

}

// Destructing tuple (From parse_config)
// Connecting returned values from parse_config to query/filename
struct Config {
    query: String,
    filename: String,
}

// Argument parsing

// &[Strings] = Reference to an array of strings
// -> = Returns a tuple with two strings
fn parse_config(args: &[String]) -> Config {
    
    // Argument with the text to search
    // Index 1. Because args[0] = bin path
    let query: String = args[1].clone();

    // Argument with the filename
    let filename: String = args[2].clone();

    Config { query, filename }
}

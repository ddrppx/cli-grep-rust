// Modules
use std::env;
use std::process;

// Crating
use minigrep::Config;

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
        // eprintln!() = Not printing error to standart output
        eprintln!("Problem parsing arguments: {}", err);
        // Exits with status code 1
        process::exit(1);
    });

    // printing out the arguments
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // Checking if run() returns an error variant
    if let Err(e) = minigrep::run(config){
        // eprintln!() = Not printing error to standart output
        eprintln!("Application error: {}", e);
        // Exits with status code 1
        process::exit(1);
    }
}


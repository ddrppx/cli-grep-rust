use std::env;

fn main() {
    // Collecting args from cli
    let args: Vec<String> = env::args().collect();
    
    // Argument with the text to search
    // Index 1. Because args[0] = bin path
    let query: &String = &args[1];

    // Argument with the filename
    let filename: &String = &args[2];

    // printing out the arguments
    println!("Searching for: {}", query);
    println!("In file: {}", filename);
}

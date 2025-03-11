use std::env; // For accessing command line arguments
use std::process; // For exiting the program
use std::fs::File; // For creating and writing to files
use::std::path::Path; // For creating file paths

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the number of arguments is correct
    if args.len() != 3 {
        // Print error message
        eprintln!("Usage: {} <input file> <output file>", args[0]);
        // Exit with error code 1
        process::exit(1);
    }

    // Get the input and output file names
    let input_file = &args[1];
    let output_file = &args[2];

    // Debug: Print the input and output file names
    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);

}

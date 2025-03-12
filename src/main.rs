use clap::Parser; // For parsing command line arguments
use std::process; // For exiting the program
use std::fs; // For creating and writing to files
use std::env; // For getting the current working directory

// Define a struct to hold the command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Path to the input file
    #[arg(short, long)]
    input: String,

    // Path to the output file
    #[arg(short, long)]
    output: String
}


// Read file
fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        process::exit(1);
    })
}

fn write_file(path: &str, data: Vec<u8>) {
    fs::write(path, data).unwrap_or_else(|err| {
        eprintln!("Failed to write file: {}", err);
        process::exit(1);
    })
}
fn main() {
    // Collect command line arguments
    let args = Args::parse();

    // Debug: Print the command line arguments
    println!("Command line arguments: {:#?}", args);

    // Debug: Print the current working directory
    println!("Current working directory: {}", env::current_dir().unwrap().display());

    // Read the input file
    let input_data = read_file(&args.input);

    // Debug: Print the input data
    println!("Input data: {:#?}", input_data);

    // Write the output file
    write_file(&args.output, input_data);
    
    // Exit the program
    process::exit(0);
}

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

// Encrypt file with XOR
fn encrypt_decrypt(data: &[u8], xor_key: u8) -> Vec<u8> {

    data.iter().map(|&byte| byte ^ xor_key).collect()
}

// Write file
fn write_file(path: &str, data: &[u8]) {
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


    // Encrypt the input data
    let key = 0x42;

    let encrypted_data = encrypt_decrypt(&input_data, key);
    println!("Encrypted data: {:#?}", encrypted_data);
    
    
    // Write the output file
    write_file(&args.output, &encrypted_data);
    
    // Debug: Decrypt the input data
    println!("Decrypted data: {:#?}", encrypt_decrypt(&encrypted_data, key));

    // Exit the program
    process::exit(0);
}

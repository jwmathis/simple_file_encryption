use clap::Parser; // For parsing command line arguments
use std::process; // For exiting the program
use std::fs; // For creating and writing to files
//use std::env; // For getting the current working directory

// Define a struct to hold the command line arguments
#[derive(Parser)]
#[command(author = "John Wesley Mathis", version, about = "Encrypt or decrypt a file")]
#[command(long_about = "Program to encrypt or decrypt a file using a simple XOR cipher. The default cipher type is encryption, but can be set to decryption. \n\nFor encryption set the cipher type to \"encrypt\" or \"e\"; For decryption set the cipher type to \"decrypt\" or \"d\"")]
struct Args {
    /// Cipher type
    #[arg(short = 'c', long = "cipher", default_value = "encrypt")]
    cipher_type: String,

    /// Path to the input file
    #[arg(short, long)]
    input: String,

    /// Path to the output file
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

fn print_usage() {
    println!("Usage: encryptor [OPTIONS] <input> <output>");
}

fn print_help() {
    println!("Program to encrypt or decrypt a file using a simple XOR cipher. 
    The default cipher type is encryption, but can be set to decryption. 
    \n\nFor encryption set the cipher type to \"encrypt\" or \"e\". 
    For decryption set the cipher type to \"decrypt\" or \"d\"
    ");

    print_usage();

    println!(
    "Options:
    -c, --cipher  Set the cipher type [default: encrypt]
    -i, --input   Set the input file
    -o, --output  Set the output file
    -h, --help    Print this help message"
    );

}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 || args.contains(&String::from("--help")) {
        print_help();
        process::exit(0);
    } else if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let mut input = None
    let mut output = None
    let mut cipher_type = None

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--input" => {b
        }
    }
    // Debug: Print the current working directory
    //println!("Current working directory: {}", env::current_dir().unwrap().display());
    
    // Read the input file
    let input_data = read_file(input);

    // Debug: Print the input data
    //println!("Input data: {:#?}", input_data);


    // Encrypt the input data
    let key = 0x42;

    let encrypted_data = encrypt_decrypt(&input_data, key);
    //println!("Encrypted data: {:#?}", encrypted_data);
    
    
    // Write the output file
    write_file(output, &encrypted_data);
    
    if cipher_type == "decrypt" || cipher_type == "d" {
        println!("Successfully decrypted and written to file: {}", args[2]);
    } else {
        println!("Successfully encrypted and written to file: {}", args[2]);
        println!("The encryption key is: {}", key);
    }
    // Debug: Decrypt the input data
    //println!("Decrypted data: {:#?}", encrypt_decrypt(&encrypted_data, key));

    // Exit the program
    process::exit(0);
}

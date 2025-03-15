use std::process; // For exiting the program
use std::fs; // For creating and writing to files
//use std::env; // For getting the current working directory
use colored::*;

// Read file
fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}.red()", err);
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

// Print CLI Application usage
fn print_help_summary() {
    println!("{} encryptor [OPTIONS] <input> <output>", "Usage:".green().bold());

    println!(
        "\n{}:
        -c, --cipher  Set the cipher type [default: encrypt]
        -i, --input   Set the input file
        -o, --output  Set the output file
        -h, --help    Print this help message (--help for complete help)" 
        , "Options".bold());
}

// Print CLI Application help
fn print_help() {
    println!("
    {}
    Program to encrypt or decrypt a file using a simple XOR cipher. 
    The default cipher type is encryption, but can be set to decryption. 
    \n\nFor encryption set the cipher type to \"encrypt\" or \"e\". 
    For decryption set the cipher type to \"decrypt\" or \"d\"
    ", "CLI Xor Cipher".blue().bold());

    println!("{} encryptor [OPTIONS] <input> <output>", "Usage:".green().bold());

    println!(
    "\n{}:
    -c, --cipher  Set the cipher type [default: encrypt]
    -i, --input   Set the input file
    -o, --output  Set the output file
    -h, --help    Print this help message (-h for summary)" 
    , "Options".bold());

}

fn main() {
    // Collect command line arguments
    let args: Vec<String> = std::env::args().collect();

    // Argument variables
    let mut input = None;
    let mut output = None;
    let mut cipher = None;

    
    let mut i = 1;

    // Parse command line arguments
    while i < args.len() {
        match args[i].as_str() {
                "--cipher" | "-c" => {
                    i += 1;
                    if i < args.len() {
                        cipher = Some(args[i].clone());
                    } else {
                        eprintln!("Error: Missing cipher type. --cipher requires an argument.");
                        process::exit(1);
                    }
                }
                "--input" | "-i" => {
                    i += 1;
                    if i < args.len() {
                        input = Some(args[i].clone());
                    } else {
                        eprintln!("Error: Missing input file. --input requires an argument.");
                        process::exit(1);
                    }
                }
               "--output" | "-o" => {
                    i += 1;
                    if i < args.len() {
                        output = Some(args[i].clone());
                    } else {
                        eprintln!("Error: Missing output file. --output requires an argument.");
                        process::exit(1);
                    }
                }
                "--help" => {
                    print_help();
                    process::exit(0);
                }
                "-h" => {
                    print_help_summary();
                    process::exit(0);
                }
                _ => {
                    eprintln!("Error: Unrecognized option: {}", args[i]);
                    process::exit(1);
                }
            }
            i += 1;             
    }
    
    // Check for missing arguments
    let cipher_type: String;
    if cipher.is_none() { 
        cipher_type = "encrypt".to_string(); // Default to encryption if no cipher type is set
    } else {
        cipher_type = cipher.expect("Error: Missing cipher type. --cipher requires an argument.");
    }
    let input_file = input.expect("Error: Missing input file. --input requires an argument.");
    let output_file = output.expect("Error: Missing output file. --output requires an argument.");
    
    // Read the input file
    let input_data = read_file(&input_file);

    // Debug: Print the input data
    //println!("Input data: {:#?}", input_data);


    // Encrypt the input data
    let key = 0x42;

    let encrypted_data = encrypt_decrypt(&input_data, key);
    //println!("Encrypted data: {:#?}", encrypted_data);
    
    // Write the output file
    write_file(&output_file, &encrypted_data);
    
    // Print success message
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

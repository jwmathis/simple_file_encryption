use std::process; // For exiting the program
use std::fs; // For creating and writing to files
use colored::*; // For colored text
use std::env; // For current working directory
use std::path::Path; // For file creation

// Read file
fn read_file(path: &str) -> Vec<u8> {
    fs::read(path).unwrap_or_else(|err| {
        eprintln!("Failed to read input file: {}
Current working directory: {},
Ensure the file exists and the path is correct.", 
        err,
        env::current_dir().unwrap().display());
        process::exit(0);
    })
}

// Encrypt file with XOR
fn encrypt_decrypt(data: &[u8], xor_key: u32) -> Vec<u8> {
    //data.iter().map(|&byte| byte ^ xor_key).collect()
    data.iter().enumerate().map(|(i, &byte)| byte ^ ((xor_key >> (8 * (i % 4))) as u8)).collect()
}

// Write file
fn write_file(path: &str, data: &[u8]) {

    // Check if the parent directory exists and create it if it doesn't
    let parent_dir = Path::new(path).parent();
    let file_stem = Path::new(path).file_stem();
    
    if let Some(inner) = parent_dir  {
        if !inner.exists() {
            if Path::new(path).is_relative() {
                match fs::create_dir_all(inner) {
                    Ok(_) => println!("Adding directory/file to current working directory: {}", env::current_dir().unwrap().display()),
                    Err(err) => println!("Failed to create directory: {}", err),
                }  
            } else {
                match fs::create_dir_all(inner) {
                    Ok(_) => println!("Created directory: {}", inner.display()),
                    Err(err) => println!("Failed to create directory: {}", err),
                } 
            }  
        }
    } 
    
    if file_stem.is_some() && !Path::new(path).exists() {
        match fs::File::create(path) {
            Ok(_) => println!("Created file: {}", path),
            Err(err) => println!("Failed to create file: {}", err),
        }
    }

    fs::write(path, data).unwrap_or_else(|err| {
        eprintln!("Failed to write file: {}
Current working directory: {}
Ensure the file exists and the path is correct.", 
        err,
        env::current_dir().unwrap().display());
        process::exit(0);
    })
}

// Print CLI Application usage
fn print_help_summary() {
    println!("{} encryptor [OPTIONS] --input <input_file> --output <output_file>", "Usage:".green().bold());

    println!(
        "\n{}:
        -c, --cipher <encrypt|e|decrypt|d> Set the cipher type [default: encrypt]
        -i, --input  <input_file> Set the input file
        -o, --output <output_file> Set the output file
        -h, --help    Print this help message (-h for summary)" 
        , "Options".bold());
}

// Print CLI Application help
fn print_help() {
    println!("\n{}\nA program to encrypt or decrypt a file using a simple XOR cipher. 
The default cipher type is encryption, but can be set to decryption. 
\nFor encryption set the cipher type to \"encrypt\" or \"e\". 
For decryption set the cipher type to \"decrypt\" or \"d\".\n",
"CLI XOR Cipher Application".blue().bold());

    println!("{} encryptor [OPTIONS] --input <input_file> --output <output_file>", "Usage:".green().bold());

    println!(
    "\n{}:
    -c, --cipher <encrypt|e|decrypt|d> Set the cipher type [default: encrypt]
    -i, --input  <input_file> Set the input file
    -o, --output <output_file> Set the output file
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
                        eprintln!("\n{}: Missing cipher type. --cipher requires an argument.", "Error".red());
                        process::exit(0);
                    }
                }
                "--input" | "-i" => {
                    i += 1;
                    if i < args.len() {
                        input = Some(args[i].clone());
                    } else {
                        eprintln!("\n{}: Missing input file. --input requires an argument.", "Error".red());
                        process::exit(0);
                    }
                }
               "--output" | "-o" => {
                    i += 1;
                    if i < args.len() {
                        output = Some(args[i].clone());
                    } else {
                        eprintln!("\n{}: Missing output file. --output requires an argument.", "Error".red());
                        process::exit(0);
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
                    eprintln!("\n{}: Unrecognized option: {}", "Error".red(), args[i]);
                    print_help_summary();
                    process::exit(0);
                }
            }
            i += 1;             
    }
    
    // Check for missing arguments
    let cipher_type: String;
    if cipher.is_none() || cipher.clone().unwrap() == "encrypt" || cipher.clone().unwrap() == "e"{ 
        cipher_type = "encrypt".to_string(); // Default to encryption if no cipher type is set
    } else if cipher.clone().unwrap() == "decrypt" || cipher.clone().unwrap() == "d" {
        cipher_type = "decrypt".to_string();
    } else {
        eprintln!("\n{}: Missing/Incorrect cipher type. --cipher requires an argument.", "Error".red());
        println!("Valid cipher types: encrypt, e, decrypt, d\n");
        print_help_summary();
        process::exit(0);
    }
    let input_file = input.expect("Error: Missing input file. --input requires an argument.");
    let output_file = output.expect("Error: Missing output file. --output requires an argument.");
    
    // Read the input file
    let input_data = read_file(&input_file);

    // Debug: Print the input data
    //println!("Input data: {:#?}", input_data);


    // Encrypt the input data
    let key = 0xdeadbeef;

    let encrypted_data = encrypt_decrypt(&input_data, key);
    //println!("Encrypted data: {:#?}", encrypted_data);
    
    // Write the output file
    write_file(&output_file, &encrypted_data);
    
    // Print success message
    if cipher_type == "decrypt" || cipher_type == "d" {
        println!("Successfully decrypted and written to file: {}", output_file);
    } else {
        println!("Successfully encrypted and written to file: {}", output_file);
        println!("The encryption key is: {}", key);
    }
    // Debug: Decrypt the input data
    //println!("Decrypted data: {:#?}", encrypt_decrypt(&encrypted_data, key));

    // Exit the program
    process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let input_data = b"Hello, world!!!!";
        let key = 0x12345678;
        let encrypted_data = encrypt_decrypt(input_data, key);
        let decrypted_data = encrypt_decrypt(&encrypted_data, key);
        assert_eq!(input_data.to_vec(), decrypted_data);
    }
}
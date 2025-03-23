# XOR Cipher CLI Application

## About
This is a simple Command-Line Interface (CLI) application for encrypting and decrypting files using a XOR cipher.
The program reads a file, applies a XOR operation to the contents of the file using a fixed key, and writes the result to an output file. It supports command-line arguments for specifying the cipher type (encrypt or decrypt), input file, and output file. The application was implemented using Rust.

## Features
- **Encryption and Decryption:** Use XOR cipher to encrypt or decrypt files
- **Command-Line Interface:** Specify input and output files, and select encryption or decryption mode via arguments
- **Directory Creation:** Automatically creates necessary directories for the output file if they do not exist
- **Help Menu:** Provides usage iinstructions and options via --help or -h flags
- **Error Handling:** Gracefully handles missing files, invalid arguments, and file I/O errors

## Usage

```bash
xor_cipher [OPTIONS] --input <input_file> --output <output_file>

Options:
-c, --cipher <encrypt|e|decrypt|d> Set the cipher type [default: encrypt]
-i, --input <input_file>           Specify the input file
-o, --output <output_file>         Specify the output file
-h, --help                         Print help message (-h for summary)
```


### Examples

#### Encrypt a File
```bash
encryptor -c encrypt -i input.txt -o encrypted_output.txt
```

#### Decrypt a File
```bash
encryptor -c decrypt -i encrypted_output.txt -o decrypted_output.txt
```

## Testing

Unit tests were written to validate key functionalities:

- **Encryption/Decryption**: Ensures data integrity after applying XOR cipher.


## Requirements

- **Rust Compiler**: Ensure `cargo` and `rustc` are installed.

## Building and Running

1. Clone the repository.
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the application:
   ```bash
   cargo run -- [OPTIONS]
   ```

This project demonstrates how to serialize the ciphertext and encrypt a small number using the TFHE (Threshold Fully Homomorphic Encryption) package. It handles serialization to a byte buffer and encryption with FheUint8.
## Features
**Key Generation:**  Creates encryption and decryption keys.
**Encryption:**  Encrypts an 8-bit integer.
**Random Bit Generation:** Generates pseudo-random bits using TFHE.
**Serialization:**  Serializes the encrypted data into a byte buffer.



## Requirements
- **Rust:** Ensure you have the latest stable version of Rust installed.
- **Rust version:** a minimum Rust version of $1.73$ is required to compile TFHE-rs.
- **TFHE Library:** The project depends on the TFHE library for fully homomorphic encryption. Ensure the library is included in your `Cargo.toml` file. 

``` 
#To include library run:
cargo add tfhe

#Alternatively paste the line below in 'Cargo.toml' 
#For x86_64 machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "x86_64-unix" ] }
#For ARM machine running a Unix-like OS:

tfhe = { version = "0.7.2", features = [ "boolean", "shortint", "integer", "aarch64-unix" ] }
#For x86_64 machines with the rdseed instruction running Windows:

tfhe = { version = "*", features = ["boolean", "shortint", "integer", "x86_64"] }

#ensure to build cargo after adding the tfhe library
cargo run build
```
## Run code
>[!TIP]
> Performance: for optimal performance, it is highly recommended to run code that uses TFHE-rs in release mode with cargo's --release flag.
>```
>cargo -- run release
>```
 ## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

  ## Installation

- To use this project, you need to have Rust installed on your machine.
- If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it.
- After installing Rust, clone this repository or copy the code into a Rust project, Compile and run the code using cargo run.
```
bash
git clone  https://github.com/cypriansakwa/Secure_Homomorphic_Computation_Using_TFHE_with_Customizable_Cryptographic_Parameters.git
```
```
cd Secure_Homomorphic_Computation_Using_TFHE_with_Customizable_Cryptographic_Parameters
```


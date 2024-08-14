This project demonstrates how to serialize the ciphertext and encrypt a small number using the TFHE (Threshold Fully Homomorphic Encryption) package. It handles serialization to a byte buffer and encryption with FheUint8.
## Features
**Key Generation:**  Creates encryption and decryption keys.
**Encryption:**  Encrypts an 8-bit integer.
**Random Bit Generation:** Generates pseudo-random bits using TFHE.
**Serialization:**  Serializes the encrypted data into a byte buffer.



## Requirements
- **Rust:** Ensure you have the latest stable version of Rust installed. 
- **TFHE Library:** The project depends on the TFHE library for fully homomorphic encryption. Ensure the library is included in your `Cargo.toml` file. 
## Notes
- The safe_serialize function is used to convert the encrypted data into a byte format, which can be stored or transmitted as needed.
- The generate_oblivious_pseudo_random function call is included for completeness but is not used further in this example.
247247   551815  2370

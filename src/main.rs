use tfhe::{
    generate_keys, set_server_key, ConfigBuilder, FheUint8, Seed, safe_serialize,
    prelude::FheTryEncrypt,
};

fn main() {
    let config = ConfigBuilder::default_with_small_encryption();
    let (client_key, server_key) = generate_keys(config);

    set_server_key(server_key);

    let input_message: u8 = 8;

    let ciphertext = FheUint8::try_encrypt(input_message, &client_key).unwrap();

    let random_bits_count = 3;

    let _ct_res = FheUint8::generate_oblivious_pseudo_random(Seed(0), random_bits_count);

    let mut buffer = Vec::new();

    // Increase the size limit as needed
    match safe_serialize(&ciphertext, &mut buffer, 1024 * 1024) { // Example: 1MB
        Ok(_) => println!("Serialized ciphertext: {:?}", buffer),
        Err(e) => println!("Failed to serialize ciphertext: {}", e),
    }

    println!("Completed processing.");
}







use bitcoin::network::constants::Network;
use bitcoin::util::key::{Error as KeyError, PrivateKey};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network::Testnet;
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs;

fn generate_random_mnemonic() -> String {
    let mut rng = OsRng;
    let mut mnemonic_bytes = [0u8; 16];
    rng.fill_bytes(&mut mnemonic_bytes);

    // Convert the random bytes to a hexadecimal string
    hex::encode(mnemonic_bytes)
}

fn create_wallet() -> Result<(), KeyError> {
    // Generate a random mnemonic
    let mnemonic = generate_random_mnemonic();

    // Create a private key from the mnemonic
    let secp = Secp256k1::new();
    let private_key = PrivateKey::from_wif(&mnemonic)?;

    // Get the corresponding public key and address
    let public_key = private_key.public_key(&secp);
    let address = Address::p2pkh(&public_key, Network::Testnet);

    // Print the wallet details
    println!("| Public Address | {} |", address);
    println!("| Private Key    | {} |", mnemonic); // WIF is not directly available, use the original mnemonic for the private key

    // Save wallet details to a file
    let json_data = format!(
        r#"{{"address": "{}", "privateKey": "{}"}}"#,
        address, mnemonic
    );

    // Handle potential IO error and convert it to bitcoin::util::key::Error
    if let Err(io_error) = std::fs::write("rust-wallet.json", json_data) {
        return Err(KeyError::IoError(io_error.to_string()));
    }

    Ok(())
}

fn main() {
    if let Err(err) = create_wallet() {
        eprintln!("Error: {}", err);
    }
}

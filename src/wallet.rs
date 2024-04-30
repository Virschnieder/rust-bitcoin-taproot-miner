use rand::{thread_rng, RngCore};
use bitcoin::secp256k1::{Secp256k1, SecretKey, PublicKey};
use bitcoin::network::constants::Network;
use bitcoin::Address;
use hex;

pub fn generate_p2wpkh_address() -> (String, String, String) {
    // Generate a random 32-byte secret key
    let mut rng = thread_rng();
    let mut key_bytes = [0u8; 32];
    rng.fill_bytes(&mut key_bytes);
    let private_key = SecretKey::from_slice(&key_bytes).unwrap();

    // Create a Secp256k1 context
    let secp = Secp256k1::new();

    // Derive the corresponding public key
    let public_key = PublicKey::from_secret_key(&secp, &private_key);

    // Convert the public key to a bitcoin::PublicKey
    let public_key_bytes = public_key.serialize();
    let bitcoin_public_key = bitcoin::PublicKey::from_slice(&public_key_bytes).unwrap();

    // Encode the public key into a P2WPKH address
    let network = Network::Regtest; // Assuming you're using regtest network
    let address = Address::p2wpkh(&bitcoin_public_key, network).unwrap();

    let private_key_str = hex::encode(&private_key[..]);
    let public_key_str = hex::encode(public_key_bytes);
    let address_str = address.to_string();

    (private_key_str, public_key_str, address_str)
}
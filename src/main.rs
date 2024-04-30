use bitcoincore_rpc::{Auth, Client, RpcApi};
use std::error::Error;

mod wallet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a new RPC client
    let rpc = Client::new(
        "http://localhost:18443".to_string(),
        Auth::UserPass("Mahavir".to_string(), "Hackable".to_string()),
    )?;

    // Get the block count
    let block_count = rpc.get_block_count()?;

    // Print the block count
    println!("Block count: {}", block_count);

    let (private_key, public_key, address) = wallet::generate_p2wpkh_address();
    println!("Taproot Private Key: {}", private_key);
    println!("Taproot Public Key: {}", public_key);
    println!("Taproot Address: {}", address);

    // Generate 101 blocks to the new address
    let address: bitcoincore_rpc::bitcoin::Address = address.parse()?;
    let block_hashes = rpc.generate_to_address(101, &address)?;

    // Print the transaction IDs of the coinbase transactions
    for block_hash in block_hashes {
        let block = rpc.get_block(&block_hash)?;
        let coinbase_txid = block.txdata[0].txid();
        println!("Coinbase transaction ID: {}", coinbase_txid);
    }


    Ok(())
}
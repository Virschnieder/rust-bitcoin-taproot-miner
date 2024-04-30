#!/bin/bash
RPC_USER="Mahavir"
RPC_PASSWORD="Hackable"

# Start the Bitcoin Core node in regtest mode with the RPC server enabled
bitcoind -regtest -daemon -rpcuser=$RPC_USER -rpcpassword=$RPC_PASSWORD

# Wait for a bit to ensure the node has started
sleep 1

# Print the block count to verify the node is running
bitcoin-cli -regtest -rpcuser=$RPC_USER -rpcpassword=$RPC_PASSWORD getblockcount

cargo run --release --bin braidpool_assignment -- -r $RPC_USER:$RPC_PASSWORD
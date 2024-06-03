#!/bin/bash

# Find process IDs listening on port 8545 (anvil)
anvil=$(lsof -t -i:8545)

# Check if any PIDs were found
if [ -z "$anvil" ]; then
    echo "Anvil not running."
else
    # Kill the processes
    kill $anvil && echo "Terminated running Anvil process."
    sleep 3
fi
# start anvil with slots in an epoch send to 1 for faster finalised blocks
anvil --slots-in-an-epoch 1 --fork-url https://rpc.ankr.com/eth_sepolia/d37b5f42b07c9f937a58be9e76ea7c565598e80d0be0eb9ef8831a77541dcf1a --fork-block-number 5974751 &
# kill caddyserver
caddy stop
# start caddyserver
caddy start
dfx stop
# Find process IDs listening on port 4943 (dfx)
dfx=$(lsof -t -i:4943)
# Check if any PIDs were found
if [ -z "$dfx" ]; then
    echo "dfx not running."
else
    # Kill the processes
    kill $dfx && echo "Terminating running dfx instance."
    sleep 3
fi
dfx start --clean --background
dfx ledger fabricate-cycles --icp 10000 --canister $(dfx identity get-wallet)
dfx deploy evm_rpc
# because our local NFT contract deployment is deterministic, we can hardcode the 
# the `get_logs_address` here. in our case we are listening for mint events,
# that is transfer events with the `from` field being the zero address.
# you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
cargo build --release --target wasm32-unknown-unknown --package chain_fusion_backend
dfx canister create --with-cycles 10_000_000_000_000 chain_fusion_backend
dfx canister install --wasm target/wasm32-unknown-unknown/release/chain_fusion_backend.wasm chain_fusion_backend --argument '(
  record {
    ecdsa_key_id = record {
      name = "dfx_test_key";
      curve = variant { secp256k1 };
    };
    get_logs_topics = opt vec {
      vec {
        "0x52a6cdf67c40ce333b3d846e4e143db87f71dd7935612a4cafcf6ba76047ca1f";
      };
    };
    last_scraped_block_number = 5974751: nat;
    rpc_services = variant {
      Custom = record {
        chainId = 11155111 : nat64;
        services = vec { record { url = "https://localhost:8546"; headers = null } };
      }
    };
    rpc_service = variant {
      Custom = record {
        url = "https://localhost:8546";
        headers = null;
      }
    };
    donation_address = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8";
    get_logs_address = vec { "0x219f09C912A765E456e154C14a0Cf9aC5e84F188" };
    block_tag = variant { Latest = null };
  },
)'
# sleep for 3 seconds to allow the evm address to be generated
sleep 3
# safe the chain_fusion canisters evm address
export EVM_ADDRESS=$(dfx canister call chain_fusion_backend get_evm_address | awk -F'"' '{print $2}')
# deploy the contract passing the chain_fusion canisters evm address to receive the fees and create a couple of new jobs
forge script script/EthDepositHelper.s.sol:MyScript --fork-url http://localhost:8545 --broadcast --sig "run(address)" $EVM_ADDRESS
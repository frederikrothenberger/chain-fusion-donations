echo "Provide private key funded with ETH to deploy to sepolia:"
read private_key
dfx deploy chain_fusion_backend --ic --argument '(
  record {
    ecdsa_key_id = record {
      name = "key_1";
      curve = variant { secp256k1 };
    };
    get_logs_topics = opt vec {
      vec {
        "0x52a6cdf67c40ce333b3d846e4e143db87f71dd7935612a4cafcf6ba76047ca1f";
      };
    };
    last_scraped_block_number = 5974751: nat;
    rpc_services = variant { EthMainnet = null };
    get_logs_address = vec { "0x219f09C912A765E456e154C14a0Cf9aC5e84F188" };
    block_tag = variant { Finalized = null };
  },
)'
# get ethereum address from canister
export EVM_ADDRESS=$(dfx canister call --ic chain_fusion_backend get_evm_address | awk -F'"' '{print $2}')
# provide ethereum address from canister as argument to deploy the contract
output$(forge create -r wss://ethereum-sepolia-rpc.publicnode.com --private-key $private_key src/foundry/EthDepositHelper.sol:EthDeposit --constructor-args $EVM_ADDRESS)
# Parse the Deployed to address using sed
deployed_address=$(echo "$output" | sed -n 's/.*Deployed to: \([^ ]*\).*/\1/p')
# reinstall canister with correct deposit helper address 
dfx deploy chain_fusion_backend --mode reinstall --ic --argument '(
  record {
    ecdsa_key_id = record {
      name = "key_1";
      curve = variant { secp256k1 };
    };
    get_logs_topics = opt vec {
      vec {
        "0x52a6cdf67c40ce333b3d846e4e143db87f71dd7935612a4cafcf6ba76047ca1f";
      };
    };
    last_scraped_block_number = 5980780 : nat;
    rpc_services = variant { EthMainnet = null };
    get_logs_address = vec { '"$deployed_address"' };
    block_tag = variant { Finalized = null };
  },
)'
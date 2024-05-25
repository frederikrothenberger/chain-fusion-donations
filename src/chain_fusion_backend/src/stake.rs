use ethers_core::types::U256;
use ethers_core::utils::keccak256;

use crate::{
    evm_rpc::SendRawTransactionStatus,
    evm_signer::{self, SignRequest},
    state::{mutate_state, read_state},
    transactions::{create_sign_request, send_raw_transaction},
};

pub async fn stake_eth(value: u128) {

    deposit_lido(value).await;

}


async fn deposit_lido(value: u128) {

    let contract_address = String::from("0x3e3FE7dBc6B4C189E7128855dD526361c49b40Af");
    let gas_limit = U256::from(500000);

    let request = create_sign_request(
        U256::from_dec_str(&value.to_string()).unwrap(),
        Some(contract_address),
        None,
        Some(gas_limit),
        None,
    )
    .await;  

    sign_and_submit(request).await;

}

// async fn deposit_rocket(_value: &str) {

    
    
//     let function_signature = "getAddress(bytes32)";


//     let enc_1 = ethers_core::abi::AbiEncode::encode("contract.address");
//     let enc_2 = ethers_core::abi::AbiEncode::encode("rocketDepositPool");

//     let mut encoded_data = Vec::new();
//     encoded_data.extend_from_slice(&enc_1);
//     encoded_data.extend_from_slice(&enc_2);

//     let argument = keccak256(&encoded_data);

//     let mut data = keccak256(function_signature).as_ref()[0..4].to_vec();
//     data.extend(ethers_core::abi::AbiEncode::encode(argument));
// }



async fn sign_and_submit(request: SignRequest) {


    let tx = evm_signer::sign_transaction(request).await;

    let status = send_raw_transaction(tx.clone()).await;

    println!("Transaction sent: {:?}", tx);

    match status {
        SendRawTransactionStatus::Ok(transaction_hash) => {
            println!("Success {transaction_hash:?}");
            mutate_state(|s| {
                s.nonce += U256::from(1);
            });
        }
        SendRawTransactionStatus::NonceTooLow => {
            println!("Nonce too low");
        }
        SendRawTransactionStatus::NonceTooHigh => {
            println!("Nonce too high");
        }
        SendRawTransactionStatus::InsufficientFunds => {
            println!("Insufficient funds");
        }
    }
}

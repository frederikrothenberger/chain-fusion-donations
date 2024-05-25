use ethers_core::types::U256;

use crate::transactions::transfer_eth;

pub async fn deposit_lido(value: U256) {
    let contract_address = String::from("0x3e3FE7dBc6B4C189E7128855dD526361c49b40Af");
    let gas_limit = Some(U256::from(500000));

    transfer_eth(value, contract_address, gas_limit).await;
}

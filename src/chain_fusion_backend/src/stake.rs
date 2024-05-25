use ethers_core::types::U256;

use crate::{fees, transactions::transfer_eth};

pub async fn deposit_lido(value: U256) {
    let contract_address = String::from("0x3e3FE7dBc6B4C189E7128855dD526361c49b40Af");
    let gas_limit = U256::from(500000);
    let fee_estimates = fees::estimate_transaction_fees(9).await;
    let actual_stake = value - (fee_estimates.max_fee_per_gas * gas_limit);
    transfer_eth(actual_stake, contract_address, gas_limit, fee_estimates).await;
}

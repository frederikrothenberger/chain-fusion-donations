use ethers_core::types::U256;
use ethers_core::utils::parse_ether;
use crate::balances::{move_unstaked_to_staked, total_unstaked_balance};

use crate::{fees, transactions::transfer_eth};

const STAKE_THRESHOLD: &str = "10";

pub async fn deposit_lido_if_threshold_reached() {
    let total_unstaked = total_unstaked_balance();
    if total_unstaked > parse_ether(STAKE_THRESHOLD).unwrap() {
        let contract_address = String::from("0x3e3FE7dBc6B4C189E7128855dD526361c49b40Af");
        let gas_limit = U256::from(500000);
        let fee_estimates = fees::estimate_transaction_fees(9).await;
        let actual_stake = total_unstaked - (fee_estimates.max_fee_per_gas * gas_limit);

        transfer_eth(actual_stake, contract_address, gas_limit, fee_estimates).await;
        move_unstaked_to_staked();
    }
}

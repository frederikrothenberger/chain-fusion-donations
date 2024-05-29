use crate::{
    balances::{move_unstaked_to_staked, total_staked_balance, total_unstaked_balance},
    eth_call::erc20_balance_of,
    state::read_state,
    utils::{nat_to_u256, u256_to_nat},
};
use candid::Nat;
use ethers_core::types::U256;
use ethers_core::utils::parse_ether;
use ic_cdk::println;

use crate::{fees, transactions::transfer_eth};

const STAKE_THRESHOLD: &str = "10";
const WITHDRAWAL_THRESHOLD: &str = "0.1";
const LIDO_CONTRACT_ADDRESS: &str = "0x3e3FE7dBc6B4C189E7128855dD526361c49b40Af";

pub async fn deposit_steth_if_threshold_reached() {
    let total_unstaked = total_unstaked_balance();
    if total_unstaked > parse_ether(STAKE_THRESHOLD).unwrap() {
        let gas_limit = U256::from(500000);
        let fee_estimates = fees::estimate_transaction_fees(9).await;
        let actual_stake = total_unstaked - (fee_estimates.max_fee_per_gas * gas_limit);

        transfer_eth(
            actual_stake,
            LIDO_CONTRACT_ADDRESS.into(),
            gas_limit,
            fee_estimates,
        )
        .await;
        move_unstaked_to_staked();
    }
}

pub async fn check_steth_balance() -> U256 {
    // get canisters eth address
    let account = read_state(|s| s.evm_address.clone()).unwrap();
    erc20_balance_of(LIDO_CONTRACT_ADDRESS.into(), account).await
}

pub async fn withdraw_steth(withdrawal: U256) {
    let beneficary = read_state(|s| s.donation_address.clone());
    let gas_limit = U256::from(500000);
    let fee_estimates = fees::estimate_transaction_fees(9).await;
    transfer_eth(withdrawal, beneficary, gas_limit, fee_estimates).await
}

pub async fn withdraw_steth_if_threshold_reached() {
    let steth_balance = u256_to_nat(&check_steth_balance().await);
    println!("steth_balance {}", steth_balance);
    let total_staked = u256_to_nat(&total_staked_balance());
    println!("total_staked{}", total_staked);
    if steth_balance > total_staked {
        let available_withdrawal = steth_balance - total_staked;
        if available_withdrawal > u256_to_nat(&parse_ether(WITHDRAWAL_THRESHOLD).unwrap()) {
            withdraw_steth(nat_to_u256(&available_withdrawal)).await;
        }
    } else {
        withdraw_steth(nat_to_u256(&(steth_balance / Nat::from(2u16)))).await;
    }
}

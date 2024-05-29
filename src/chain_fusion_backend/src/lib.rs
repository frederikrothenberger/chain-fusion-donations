mod eth_call;
mod eth_logs;
mod evm_rpc;
mod evm_signer;
mod fees;
mod guard;
mod job;
mod lido;
mod lifecycle;
mod state;
// mod storage;
mod balances;
mod storage;
mod transactions;
mod utils;

use std::time::Duration;

use eth_logs::scrape_eth_logs;

use ic_cdk::println;
use lido::withdraw_steth_if_threshold_reached;
use lifecycle::InitArg;
use state::read_state;

use crate::state::{initialize_state, mutate_state};

pub const SCRAPING_LOGS_INTERVAL: Duration = Duration::from_secs(3 * 60);
pub const CHECK_REDEEMABLE_FUNDS_INTERVAL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

fn setup_timers() {
    // as timers are synchronous, we need to spawn a new async task to get the public key
    ic_cdk_timers::set_timer(Duration::ZERO, || {
        ic_cdk::spawn(async {
            let public_key = evm_signer::get_public_key().await;
            let evm_address = evm_signer::pubkey_bytes_to_address(&public_key);
            mutate_state(|s| {
                s.ecdsa_pub_key = Some(public_key);
                s.evm_address = Some(evm_address);
            });
        })
    });
    // Start scraping logs almost immediately after the install, then repeat with the interval.
    ic_cdk_timers::set_timer(Duration::from_secs(10), || ic_cdk::spawn(scrape_eth_logs()));
    ic_cdk_timers::set_timer_interval(SCRAPING_LOGS_INTERVAL, || ic_cdk::spawn(scrape_eth_logs()));
    ic_cdk_timers::set_timer_interval(CHECK_REDEEMABLE_FUNDS_INTERVAL, || {
        ic_cdk::spawn(withdraw_steth_if_threshold_reached())
    });
}

#[ic_cdk::init]
fn init(arg: InitArg) {
    println!("[init]: initialized minter with arg: {:?}", arg);
    initialize_state(state::State::try_from(arg).expect("BUG: failed to initialize minter"));
    setup_timers();
}

#[ic_cdk::query]
fn get_evm_address() -> String {
    read_state(|s| s.evm_address.clone()).expect("evm address should be initialized")
}

// Enable Candid export, read more [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/generating-candid/)
ic_cdk::export_candid!();

mod submit_result;

use ethers_core::types::{Address, U256};
use ic_cdk::println;
use std::str::FromStr;
// use submit_result::submit_result;

use crate::{
    evm_rpc::LogEntry,
    state::{mutate_state, LogSource},
};

pub async fn job(event_source: LogSource, event: LogEntry) {
    mutate_state(|s| s.record_processed_log(event_source.clone()));
    // because we deploy the canister with topics only matching
    // NewJob events we can safely assume that the event is a NewJob.
    let received_eth_event = ReceivedEthEvent::from(event);
    println!("Received Eth Event: {:?}", received_eth_event);
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ReceivedEthEvent {
    pub from: Address,
    pub value: U256,
}

impl From<LogEntry> for ReceivedEthEvent {
    fn from(entry: LogEntry) -> ReceivedEthEvent {
        // we expect exactly 3 topics from the ReceivedEth event.
        // you can read more about event signatures [here](https://docs.alchemy.com/docs/deep-dive-into-eth_getlogs#what-are-event-signatures)
        let from =
            ethers_core::types::Address::from_str(&entry.topics[1][entry.topics[1].len() - 40..])
                .expect("the address contained in the first topic should be valid");
        let value = U256::from_str_radix(&entry.data, 16).expect("the token id should be valid");

        ReceivedEthEvent { from, value }
    }
}

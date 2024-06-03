use super::{RequestResult, EVM_RPC};
use crate::state::read_state;

pub async fn request(tx: String, max_response_bytes: u64) -> RequestResult {
    let rpc_provider = read_state(|s| s.rpc_service.clone());
    let cycles = 10_000_000_000;

    match EVM_RPC
        .request(rpc_provider, tx, max_response_bytes, cycles)
        .await
    {
        Ok((res,)) => res,
        Err(e) => ic_cdk::trap(format!("Error: {:?}", e).as_str()),
    }
}

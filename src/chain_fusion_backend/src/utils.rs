use std::str::FromStr;

use candid::Nat;
use ethers_core::types::U256;

pub fn nat_to_u256(n: &Nat) -> U256 {
    let be_bytes = n.0.to_bytes_be();
    U256::from_big_endian(&be_bytes)
}

pub fn u256_to_nat(u: &U256) -> Nat {
    Nat::from_str(&u.to_string()).expect("Conversion should not fail")
}

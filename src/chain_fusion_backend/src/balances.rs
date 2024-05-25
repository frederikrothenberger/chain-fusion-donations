use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::Arc;
use ethers_core::types::{Address, U256};
use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use crate::storage::{BALANCES_MEMORY_ID, with_memory_manager};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EthAddress(Address);
impl Storable for EthAddress {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(self.0.as_bytes())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(Address::from_slice(&bytes))
    }

    const BOUND: Bound = Bound::Bounded { is_fixed_size: true, max_size: 20 };
}


struct EthBalance(U256);
impl Storable for EthBalance {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = Vec::with_capacity(32);
        self.clone().0.to_big_endian(&mut buf);
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(U256::from_big_endian(&bytes))
    }

    const BOUND: Bound = Bound::Bounded { is_fixed_size: true, max_size: 20 };
}

thread_local! {
  /// The memory reference to the Account repository.
  static DB: RefCell<StableBTreeMap<EthAddress, EthBalance , VirtualMemory<DefaultMemoryImpl>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(BALANCES_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref BALANCES_REPOSITORY: Arc<BalancesRepository> =
        Arc::new(BalancesRepository::default());
}

#[derive(Debug, Clone, Default)]
struct BalancesRepository {}


use crate::storage::{with_memory_manager, BALANCES_MEMORY_ID};
use ethers_core::types::{Address, U256};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct EthAddress(Address);

impl Storable for EthAddress {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Vec::from(self.0.to_fixed_bytes()))
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(Address::from_slice(&bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: true,
        max_size: 20,
    };
}

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct EthBalance(U256);

impl Storable for EthBalance {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![0;32];
        self.0.to_big_endian(&mut buf);
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(U256::from_big_endian(&bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        is_fixed_size: true,
        max_size: 32,
    };
}

thread_local! {
  /// The memory reference to the Balances repository.
  static DB: RefCell<StableBTreeMap<EthAddress, EthBalance , VirtualMemory<DefaultMemoryImpl>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(BALANCES_MEMORY_ID))
    )
  });
}

#[derive(Debug, Clone, Default)]
pub struct BalancesRepository;

impl BalancesRepository {

    pub fn get_balance(address: Address) -> Option<U256> {
        DB.with_borrow(|db| db.get(&EthAddress(address))).map(|balance| balance.0)
    }

    pub fn store_balance(address: Address, new_balance: U256) {
        DB.with_borrow_mut(|db| db.insert(EthAddress(address), EthBalance(new_balance)));
    }

    pub fn total_balance() -> U256 {
        DB.with_borrow(|db| db.iter().fold(U256::zero(), |acc, (_, balance)| acc + balance.0))
    }
}

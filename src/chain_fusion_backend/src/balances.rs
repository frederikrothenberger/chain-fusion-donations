use crate::storage::{with_memory_manager, STAKED_BALANCES_MEMORY_ID, UNSTAKED_BALANCES_MEMORY_ID};
use ethers_core::types::{Address, U256};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{BTreeMap, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::borrow::Cow;
use std::cell::RefCell;
use std::thread::LocalKey;

type BalanceDB =
    &'static LocalKey<RefCell<BTreeMap<EthAddress, EthBalance, VirtualMemory<DefaultMemoryImpl>>>>;

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
        let mut buf = vec![0; 32];
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
  static UNSTAKED_DB: RefCell<StableBTreeMap<EthAddress, EthBalance , VirtualMemory<DefaultMemoryImpl>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(UNSTAKED_BALANCES_MEMORY_ID))
    )
  });

  static STAKED_DB: RefCell<StableBTreeMap<EthAddress, EthBalance , VirtualMemory<DefaultMemoryImpl>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(STAKED_BALANCES_MEMORY_ID))
    )
  });
}

fn get_balance(address: Address, db: BalanceDB) -> Option<U256> {
    db.with_borrow(|db| db.get(&EthAddress(address)))
        .map(|balance| balance.0)
}

fn store_balance(address: Address, new_balance: U256, db: BalanceDB) {
    db.with_borrow_mut(|db| db.insert(EthAddress(address), EthBalance(new_balance)));
}

pub fn total_unstaked_balance() -> U256 {
    total_balance(&UNSTAKED_DB)
}

pub fn total_staked_balance() -> U256 {
    total_balance(&STAKED_DB)
}

fn total_balance(db: BalanceDB) -> U256 {
    db.with_borrow(|db| {
        db.iter()
            .fold(U256::zero(), |acc, (_, balance)| acc + balance.0)
    })
}

pub fn add_unstaked_balance(address: Address, added_balance: U256) {
    add_balance(address, added_balance, &UNSTAKED_DB)
}

fn add_balance(address: Address, added_balance: U256, db: BalanceDB) {
    let mut new_balance = added_balance;
    if let Some(prev_balance) = get_balance(address, db) {
        new_balance += prev_balance;
    }
    store_balance(address, new_balance, db);
}

pub fn move_unstaked_to_staked() {
    let addresses: Vec<EthAddress> =
        UNSTAKED_DB.with_borrow(|db| db.iter().map(|(key, _)| key).collect());
    for address in addresses {
        if let Some(balance) =
            UNSTAKED_DB.with_borrow_mut(|unstaked_db| unstaked_db.remove(&address))
        {
            add_balance(address.0, balance.0, &STAKED_DB);
        }
    }
}

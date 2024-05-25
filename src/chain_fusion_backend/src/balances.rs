use std::borrow::Cow;
use std::cell::RefCell;
use ic_stable_structures::memory_manager::MemoryId;
use ic_stable_structures::{StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use crate::storage::{BALANCES_MEMORY_ID, with_memory_manager};

#[derive(Debug, Serialize, Deserialize)]
struct EthAddress(ByteBuf);

impl Storable for EthAddress {
    fn to_bytes(&self) -> Cow<[u8]> {
        todo!()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        todo!()
    }

    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
  /// The memory reference to the Account repository.
  static DB: RefCell<StableBTreeMap<AccountKey, Account, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(BALANCES_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ACCOUNT_REPOSITORY: Arc<AccountRepository> =
        Arc::new(AccountRepository::default());
}


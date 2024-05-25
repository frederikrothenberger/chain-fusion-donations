use crate::storage::{with_memory_manager, BALANCES_MEMORY_ID};
use ethers_core::types::{Address, U256};
use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell, Storable};
use lazy_static::lazy_static;
use minicbor_derive::{Decode, Encode};
use std::borrow::Cow;
use std::cell::RefCell;
use std::sync::Arc;
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct EthAddress(Address);

impl Storable for EthAddress {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Vec::from(self.0.as_bytes()))
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
        let mut buf = Vec::with_capacity(32);
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

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct RepositoryMetadata {
    total_balance: EthBalance,
}

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Encode, Decode)]
struct RepositoryMetadataStorable {
    #[n(0)]
    total_balance: Vec<u8>,
}

impl From<RepositoryMetadataStorable> for RepositoryMetadata {
    fn from(value: RepositoryMetadataStorable) -> Self {
        Self {
            total_balance: EthBalance::from_bytes(Cow::Owned(value.total_balance)),
        }
    }
}

impl From<RepositoryMetadata> for RepositoryMetadataStorable {
    fn from(value: RepositoryMetadata) -> Self {
        Self {
            total_balance: value.total_balance.to_bytes().to_vec(),
        }
    }
}
impl Storable for RepositoryMetadataStorable {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        minicbor::encode(self, &mut buf).expect("failed to serialize repository metadata");
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        minicbor::decode(bytes.as_ref()).unwrap_or_else(|e| {
            panic!(
                "failed to deserialize repository metadata {}: {e}",
                hex::encode(bytes)
            )
        })
    }

    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
  /// The memory reference to the Balances repository.
  static DB: RefCell<StableBTreeMap<EthAddress, EthBalance , VirtualMemory<DefaultMemoryImpl>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(BALANCES_MEMORY_ID))
    )
  });

    /// The metadata of the Balances repository (such as total balance, etc.)
  static METADATA: RefCell<StableCell<RepositoryMetadataStorable, VirtualMemory<DefaultMemoryImpl>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableCell::init(memory_manager.get(BALANCES_MEMORY_ID), RepositoryMetadataStorable::default()).expect("failed to initialize balances repository metadata")
    )
  });
}

lazy_static! {
    pub static ref BALANCES_REPOSITORY: Arc<BalancesRepository> =
        Arc::new(BalancesRepository::default());
}

#[derive(Debug, Clone, Default)]
pub struct BalancesRepository;

impl BalancesRepository {

    pub fn store_balance(&mut self, address: EthAddress, balance: EthBalance) {
        let previous_value = DB.with_borrow(|db| db.get(&address));

        

    }
}

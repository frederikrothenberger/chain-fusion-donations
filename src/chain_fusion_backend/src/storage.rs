use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, storable::Bound, storable::Storable, DefaultMemoryImpl, StableBTreeMap, Memory};
use minicbor_derive::{Decode, Encode};
use std::borrow::Cow;
use std::cell::RefCell;

pub const BALANCES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub const AGGREGATIONS_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

pub fn with_memory_manager<R>(f: impl FnOnce(&MemoryManager<DefaultMemoryImpl>) -> R) -> R {
    MEMORY_MANAGER.with(|cell| f(&cell.borrow()))
}

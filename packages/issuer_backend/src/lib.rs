mod eth;
mod guard;
mod service;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

const CREDENTIAL_TYPE: &str = "GitCoinPassportScore";

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // SCORES maps principals to scores. The Ethereum address to which the principal is linked is
    // not stored here
    static PRINCIPAL_SCORE: RefCell<StableBTreeMap<[u8;29], f32, VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );


    /// Maps Ethereum addresses to principals. This is used to prevent multiple principals from
    /// linking to the same Ethereum address. The ethereum address is the key and the principal is the value.
    ///
    /// The eth address is stored as a 32 keccak hash to preserve the privacy of the user.
    ///
    /// The principal is stored as a 29 byte slice to save space.
    static ETH_PRINCIPAL: RefCell<StableBTreeMap<[u8;32], [u8;29], VirtualMemory<DefaultMemoryImpl>>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );

}

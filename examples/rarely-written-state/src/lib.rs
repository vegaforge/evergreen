#![no_std]
//! A contract whose one piece of state is written once and then mostly read.
//!
//! `evergreen-core` extends an entry's TTL on access, so a contract that is
//! touched often keeps itself alive. This one is not. After the initial write
//! it might sit untouched for long stretches, and extend-on-access cannot help
//! an entry that nothing is accessing. That is the gap the off-chain
//! `evergreen-monitor` closes: it watches the entry and extends it on a schedule
//! between accesses. See the crate README and docs/src/roadmap.md.

use evergreen_core::{PersistentStore, TtlPolicy};
use soroban_sdk::{contract, contractimpl, contracttype, Env};

const POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    Ok(policy) => policy,
    Err(_) => panic!("policy constants are invalid"),
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Record,
}

#[contract]
pub struct RarelyWritten;

#[contractimpl]
impl RarelyWritten {
    /// Write the record. In a real deployment this happens rarely.
    pub fn put(env: Env, value: i128) {
        PersistentStore::new(&env, POLICY).set(&DataKey::Record, &value);
    }

    /// Read the record. Zero if nothing was written.
    pub fn get(env: Env) -> i128 {
        PersistentStore::new(&env, POLICY)
            .get(&DataKey::Record)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::{RarelyWritten, RarelyWrittenClient};
    use soroban_sdk::Env;

    #[test]
    fn put_then_get() {
        let env = Env::default();
        let id = env.register(RarelyWritten, ());
        let client = RarelyWrittenClient::new(&env, &id);

        assert_eq!(client.get(), 0);
        client.put(&42);
        assert_eq!(client.get(), 42);
    }
}

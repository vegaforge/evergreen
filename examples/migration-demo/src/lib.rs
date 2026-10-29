#![no_std]
//! Version 1 of a contract, used to demonstrate a storage migration.
//!
//! This is the "before" state. It stores a profile under one key shape and
//! reports its storage version as 1. The companion `evergreen-migrate` engine
//! will define a migration to version 2 that reshapes this data, dry-run it to
//! show the full diff, and execute it across a Wasm upgrade. See the crate
//! README and PRD section 6.3.

use soroban_sdk::{contract, contractimpl, contractmeta, contracttype, Env, Symbol};

contractmeta!(key = "binver", val = "1.0.0");

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Version,
    DisplayName,
}

#[contract]
pub struct ProfileV1;

#[contractimpl]
impl ProfileV1 {
    /// Set the storage version at deploy time.
    pub fn __constructor(env: Env) {
        env.storage().instance().set(&DataKey::Version, &1u32);
    }

    /// Store the profile display name.
    pub fn set_name(env: Env, name: Symbol) {
        env.storage().instance().set(&DataKey::DisplayName, &name);
    }

    /// Read the profile display name.
    pub fn name(env: Env) -> Option<Symbol> {
        env.storage().instance().get(&DataKey::DisplayName)
    }

    /// The storage version. A migration to version 2 would bump this.
    pub fn version(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::Version).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::{ProfileV1, ProfileV1Client};
    use soroban_sdk::{symbol_short, Env};

    #[test]
    fn stores_name_and_reports_version() {
        let env = Env::default();
        let id = env.register(ProfileV1, ());
        let client = ProfileV1Client::new(&env, &id);

        assert_eq!(client.version(), 1);
        client.set_name(&symbol_short!("alice"));
        assert_eq!(client.name(), Some(symbol_short!("alice")));
    }
}

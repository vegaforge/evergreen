#![no_std]
//! A small token contract that stores balances through `evergreen-core`.
//!
//! The point of the example: there is no `extend_ttl` call anywhere in this
//! file. Balances go through a `PersistentStore` and the admin through an
//! `InstanceStore`, both carrying a TTL policy, so every read and write keeps
//! the data alive on its own.

use evergreen_core::{InstanceStore, PersistentStore, TtlPolicy};
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

// Extend balances when they fall under ~1 day of TTL, push them back to ~30
// days. Values are in ledgers (~5s each) and clamp to the network max at run
// time. Checked at compile time: a bad policy fails the build.
const BALANCE_POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    Ok(policy) => policy,
    Err(_) => panic!("balance policy constants are invalid"),
};

// Same shape for the admin in instance storage.
const CONFIG_POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    Ok(policy) => policy,
    Err(_) => panic!("config policy constants are invalid"),
};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Balance(Address),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TokenError {
    NotInitialized = 1,
    InvalidAmount = 2,
    InsufficientBalance = 3,
    Overflow = 4,
}

#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    /// Set the admin at deploy time.
    pub fn __constructor(env: Env, admin: Address) {
        config(&env).set(&DataKey::Admin, &admin);
    }

    /// Mint `amount` to `to`. Admin only.
    pub fn mint(env: Env, to: Address, amount: i128) -> Result<(), TokenError> {
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }
        read_admin(&env)?.require_auth();

        let balances = balances(&env);
        let key = DataKey::Balance(to);
        let current: i128 = balances.get(&key).unwrap_or(0);
        let next = current.checked_add(amount).ok_or(TokenError::Overflow)?;
        balances.set(&key, &next);
        Ok(())
    }

    /// Move `amount` from `from` to `to`. Requires `from`'s authorization.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), TokenError> {
        if amount <= 0 {
            return Err(TokenError::InvalidAmount);
        }
        from.require_auth();

        let balances = balances(&env);
        let from_key = DataKey::Balance(from);
        let to_key = DataKey::Balance(to);

        let from_balance: i128 = balances.get(&from_key).unwrap_or(0);
        if from_balance < amount {
            return Err(TokenError::InsufficientBalance);
        }
        let to_balance: i128 = balances.get(&to_key).unwrap_or(0);
        let to_next = to_balance.checked_add(amount).ok_or(TokenError::Overflow)?;

        balances.set(&from_key, &(from_balance - amount));
        balances.set(&to_key, &to_next);
        Ok(())
    }

    /// Balance of `who`. Zero if the account has never held any.
    pub fn balance(env: Env, who: Address) -> i128 {
        balances(&env).get(&DataKey::Balance(who)).unwrap_or(0)
    }
}

fn balances(env: &Env) -> PersistentStore<'_> {
    PersistentStore::new(env, BALANCE_POLICY)
}

fn config(env: &Env) -> InstanceStore<'_> {
    InstanceStore::new(env, CONFIG_POLICY)
}

fn read_admin(env: &Env) -> Result<Address, TokenError> {
    config(env)
        .get(&DataKey::Admin)
        .ok_or(TokenError::NotInitialized)
}

#[cfg(test)]
mod test;

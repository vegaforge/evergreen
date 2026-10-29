#![no_std]
//! Declarative TTL policy and typed storage wrappers for Soroban contracts.
//!
//! `evergreen-core` is the on-chain layer of Evergreen. It wraps Soroban's
//! storage tiers in typed handles that carry a TTL policy, so a contract author
//! declares intent once ("this is a balance, keep it alive") and never writes
//! TTL extension logic by hand.
//!
//! # The model
//!
//! A [`TtlPolicy`] is a small declarative rule: extend an entry when its
//! remaining TTL drops below a threshold, and extend it to a target. You attach
//! a policy to a tier through a store handle:
//!
//! - [`PersistentStore`] for data that must survive archival, such as balances.
//! - [`InstanceStore`] for small global state that shares the contract instance
//!   TTL, such as an admin address.
//!
//! Each store applies its policy on access. Writes store and then extend; reads
//! that find data extend on the way out. Every extension is first clamped to the
//! network's live maximum TTL, which Evergreen reads from the ledger rather than
//! hardcoding, so it never submits an extension the host would reject.
//!
//! # Example
//!
//! Declare a policy as a compile-time constant, then route balances through it:
//!
//! ```rust,ignore
//! use evergreen_core::{PersistentStore, TtlPolicy};
//! use soroban_sdk::{Address, Env};
//!
//! const BALANCE_POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
//!     Ok(policy) => policy,
//!     Err(_) => panic!("balance policy constants are invalid"),
//! };
//!
//! fn credit(env: &Env, who: &Address, amount: i128) {
//!     let balances = PersistentStore::new(env, BALANCE_POLICY);
//!     let current: i128 = balances.get(who).unwrap_or(0);
//!     balances.set(who, &(current + amount));
//!     // No extend_ttl call here: the store kept the entry alive for you.
//! }
//! ```
//!
//! See the crate README and the [roadmap](https://github.com/vegaforge/evergreen/blob/main/docs/src/roadmap.md)
//! for the wider framework: the off-chain monitor, the CLI, and the upgrade and
//! migration toolkit.

mod error;
mod policy;
mod store;
mod tier;

pub use error::EvergreenError;
pub use policy::{ResolvedTtl, TtlPolicy};
pub use store::{InstanceStore, PersistentStore};
pub use tier::Tier;

#[cfg(test)]
mod tests;

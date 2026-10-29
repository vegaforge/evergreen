//! Storage handles that carry a TTL policy.
//!
//! Each handle wraps one tier and applies its policy on access: writes store
//! then extend, reads that hit extend on the way out. Extensions clamp to the
//! network max first. Extending a healthy entry is a no-op, so touching on every
//! access is cheap.

mod instance;
mod persistent;

pub use instance::InstanceStore;
pub use persistent::PersistentStore;

use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

use crate::policy::TtlPolicy;
use crate::tier::Tier;

/// Persistent storage with a TTL policy attached.
///
/// Persistent entries are archived, not deleted, when their TTL lapses, so they
/// can be restored. The policy keeps them alive so it does not come to that.
/// Cheap to construct: build one per call from a shared policy constant.
pub struct PersistentStore<'a> {
    env: &'a Env,
    policy: TtlPolicy,
}

impl<'a> PersistentStore<'a> {
    /// Bind `policy` to the persistent tier.
    pub fn new(env: &'a Env, policy: TtlPolicy) -> Self {
        Self { env, policy }
    }

    /// Always [`Tier::Persistent`].
    pub const fn tier(&self) -> Tier {
        Tier::Persistent
    }

    /// The attached policy.
    pub const fn policy(&self) -> TtlPolicy {
        self.policy
    }

    /// Store a value and extend its TTL.
    pub fn set<K, V>(&self, key: &K, value: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        let storage = self.env.storage().persistent();
        storage.set(key, value);
        self.extend(key);
    }

    /// Read a value, extending its TTL if it exists.
    ///
    /// The extension changes the entry's TTL, which costs rent, but the key
    /// stays in the read-only footprint. Use the raw SDK handle if you need a
    /// read that never pays for an extension.
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        let storage = self.env.storage().persistent();
        let value = storage.get(key);
        if value.is_some() {
            self.extend(key);
        }
        value
    }

    /// Whether the entry exists. Leaves the TTL alone.
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        self.env.storage().persistent().has(key)
    }

    /// Remove an entry.
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, Val>,
    {
        self.env.storage().persistent().remove(key);
    }

    // Extend one key, clamped to the network max.
    fn extend<K>(&self, key: &K)
    where
        K: IntoVal<Env, Val>,
    {
        let resolved = self.policy.resolve(self.env);
        self.env
            .storage()
            .persistent()
            .extend_ttl(key, resolved.threshold, resolved.extend_to);
    }
}

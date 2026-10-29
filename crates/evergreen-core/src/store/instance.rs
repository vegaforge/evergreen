use soroban_sdk::{Env, IntoVal, TryFromVal, Val};

use crate::policy::TtlPolicy;
use crate::tier::Tier;

/// Instance storage with a TTL policy attached.
///
/// The instance map shares one TTL, so the whole thing lives and is archived
/// together. Extending touches that shared TTL, not a single key, which makes
/// this the cheap place for small global state like an admin address. Reads
/// extend too, since any access means the contract is in use.
pub struct InstanceStore<'a> {
    env: &'a Env,
    policy: TtlPolicy,
}

impl<'a> InstanceStore<'a> {
    /// Bind `policy` to the instance tier.
    pub fn new(env: &'a Env, policy: TtlPolicy) -> Self {
        Self { env, policy }
    }

    /// Always [`Tier::Instance`].
    pub const fn tier(&self) -> Tier {
        Tier::Instance
    }

    /// The attached policy.
    pub const fn policy(&self) -> TtlPolicy {
        self.policy
    }

    /// Store a value and extend the instance TTL.
    pub fn set<K, V>(&self, key: &K, value: &V)
    where
        K: IntoVal<Env, Val>,
        V: IntoVal<Env, Val>,
    {
        self.env.storage().instance().set(key, value);
        self.extend();
    }

    /// Read a value and extend the instance TTL.
    pub fn get<K, V>(&self, key: &K) -> Option<V>
    where
        K: IntoVal<Env, Val>,
        V: TryFromVal<Env, Val>,
    {
        let value = self.env.storage().instance().get(key);
        self.extend();
        value
    }

    /// Whether the entry exists. Leaves the TTL alone.
    pub fn has<K>(&self, key: &K) -> bool
    where
        K: IntoVal<Env, Val>,
    {
        self.env.storage().instance().has(key)
    }

    /// Remove an entry.
    pub fn remove<K>(&self, key: &K)
    where
        K: IntoVal<Env, Val>,
    {
        self.env.storage().instance().remove(key);
    }

    // Extend the shared instance TTL. No key: the whole map shares one TTL.
    fn extend(&self) {
        let resolved = self.policy.resolve(self.env);
        self.env
            .storage()
            .instance()
            .extend_ttl(resolved.threshold, resolved.extend_to);
    }
}

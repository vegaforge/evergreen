# evergreen-core API

Everything `evergreen-core` exports. The crate is `no_std` and depends only on
`soroban-sdk`. The full reference is the rustdoc on
[docs.rs](https://docs.rs/evergreen-core); this page is the overview.

## `TtlPolicy`

When and how far to extend an entry's TTL. `Copy`, `Eq`, `Debug`.

```rust
pub const fn new(extend_threshold: u32, extend_to: u32) -> Result<TtlPolicy, EvergreenError>
pub const fn extend_threshold(&self) -> u32
pub const fn extend_to(&self) -> u32
pub fn resolve(&self, env: &Env) -> ResolvedTtl
```

- `new` rejects `extend_to == 0` and `extend_threshold > extend_to` with
  `EvergreenError::InvalidPolicy`.
- `resolve` caps both values at `env.storage().max_ttl()`, read live, and keeps
  the threshold at or below the target.

See [TTL policies](../concepts/ttl-policies.md).

## `ResolvedTtl`

A policy clamped to the live network limits, as returned by `resolve`.

```rust
pub struct ResolvedTtl {
    pub threshold: u32,
    pub extend_to: u32,
}
```

## `PersistentStore<'a>`

Persistent storage with a policy attached. Each key has its own TTL.

```rust
pub fn new(env: &'a Env, policy: TtlPolicy) -> Self
pub const fn tier(&self) -> Tier                 // always Tier::Persistent
pub const fn policy(&self) -> TtlPolicy
pub fn set<K, V>(&self, key: &K, value: &V)      // writes, then extends the key
pub fn get<K, V>(&self, key: &K) -> Option<V>    // extends the key if found
pub fn has<K>(&self, key: &K) -> bool            // no extension
pub fn remove<K>(&self, key: &K)                 // no extension
```

Keys and values use the usual Soroban bounds: `K: IntoVal<Env, Val>`, and
`V: IntoVal<Env, Val>` for `set` or `V: TryFromVal<Env, Val>` for `get`.

## `InstanceStore<'a>`

Instance storage with a policy attached. All keys share the contract instance's
TTL.

```rust
pub fn new(env: &'a Env, policy: TtlPolicy) -> Self
pub const fn tier(&self) -> Tier                 // always Tier::Instance
pub const fn policy(&self) -> TtlPolicy
pub fn set<K, V>(&self, key: &K, value: &V)      // writes, then extends the instance
pub fn get<K, V>(&self, key: &K) -> Option<V>    // extends the instance, found or not
pub fn has<K>(&self, key: &K) -> bool            // no extension
pub fn remove<K>(&self, key: &K)                 // no extension
```

## `Tier`

The three Soroban storage tiers as a named type.

```rust
pub enum Tier { Persistent, Temporary, Instance }

pub const fn is_restorable(self) -> bool  // true for Persistent and Instance
```

## `EvergreenError`

A `#[contracterror]` enum, so a contract can return it.

| Variant | Code | Meaning |
|---|---|---|
| `InvalidPolicy` | 1 | A zero target, or a threshold above the target |

# Choosing a store

Soroban has three storage tiers. `evergreen-core` wraps two of them today. This
page helps you pick one for each kind of data.

## Quick answer

| Data | Use | Why |
|---|---|---|
| Per-user state: balances, positions, allowances | `PersistentStore` | Must survive, and there can be many entries |
| Small global state: admin, configuration, a version number | `InstanceStore` | Shares one TTL with the contract, cheap to keep alive |
| Data you can recompute or lose: nonces, short-lived caches | Raw temporary storage, for now | Deleted for good when it lapses |

## `PersistentStore`

Each key has its own TTL. When it lapses the entry is archived, not deleted, and
can be restored. Use it for anything that would be a loss if it disappeared.

```rust
let balances = PersistentStore::new(&env, BALANCE_POLICY);
balances.set(&DataKey::Balance(user.clone()), &amount);
```

Build a store per call from a shared policy constant. Construction only stores a
reference to the `Env` and copies the policy.

## `InstanceStore`

Everything in instance storage shares the contract instance's TTL. Extending it
keeps the whole map, and the contract instance, alive together.

```rust
let config = InstanceStore::new(&env, CONFIG_POLICY);
config.set(&DataKey::Admin, &admin);
```

Keep instance storage small. The whole instance map is loaded whenever the
contract is called, so a growing collection, such as one entry per user, belongs
in `PersistentStore`.

## Temporary storage

Temporary entries are the cheapest, and they are deleted outright when their TTL
lapses. They can never be restored. Only put data there that the contract can
recompute or safely forget.

Evergreen has no `TemporaryStore` yet; issue
[#7](https://github.com/vegaforge/evergreen/issues/7) adds one with a clamp that
cannot trap. Until then, use `env.storage().temporary()` directly.

Putting durable data in temporary storage is the one storage mistake that loses
data for good. The planned `evergreen-lint` rule
([#10](https://github.com/vegaforge/evergreen/issues/10)) flags it.

## When not to use a store

A store extends on every read that finds a value. If you need a read that never
pays for an extension, for example in a view function that is called very often
and must stay as cheap as possible, read through the raw SDK handle
(`env.storage().persistent().get(...)`) instead.

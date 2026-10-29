# evergreen-core

The on-chain layer of [Evergreen](../../README.md). It wraps Soroban's storage
tiers in typed handles that carry a TTL policy, so a contract declares intent
once and never writes TTL extension logic by hand.

## What it gives you

- `TtlPolicy`: a declarative rule, "extend when remaining TTL drops below X,
  extend to Y", validated at construction.
- `PersistentStore` and `InstanceStore`: typed handles that apply a policy on
  every read and write.
- `Tier`: the three storage tiers as a named type, so tier choice is explicit.

## How it works

A store applies its policy on access. Writes store the value and then extend its
TTL. Reads that find a value extend it on the way out. Before any extension, the
policy is clamped to the network's current maximum TTL, which the store reads
live from the ledger rather than hardcoding, because that maximum is a network
parameter that can change on a protocol upgrade.

Extending a healthy entry is a no-op at the host level, so applying the policy on
every access costs nothing when the entry is not close to archival.

```rust,ignore
use evergreen_core::{PersistentStore, TtlPolicy};
use soroban_sdk::{Address, Env};

const BALANCE_POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    Ok(policy) => policy,
    Err(_) => panic!("balance policy constants are invalid"),
};

fn credit(env: &Env, who: &Address, amount: i128) {
    let balances = PersistentStore::new(env, BALANCE_POLICY);
    let current: i128 = balances.get(who).unwrap_or(0);
    balances.set(who, &(current + amount));
    // No extend_ttl call: the store kept the entry alive.
}
```

## Scope

This crate handles the on-chain case: data that is written often enough that
extend-on-access keeps it alive. Data that is written rarely can still lapse
between writes. The off-chain `evergreen-monitor` closes that gap. See the
workspace README for the full picture.

## License

Apache-2.0.

# TTL policies

A `TtlPolicy` is the one decision Evergreen asks you to make about an entry's
lifetime. This page explains what the two numbers mean, how Evergreen turns them
into a host call, and how to choose them.

## The two numbers

```rust
TtlPolicy::new(extend_threshold, extend_to)
```

Both are in ledgers, about five seconds each.

- `extend_threshold`: extend once the entry's remaining TTL is at or below this.
  The comparison is inclusive: an entry with exactly `extend_threshold` ledgers
  left is extended.
- `extend_to`: how many ledgers the entry should have left after the extension.

They map straight onto Soroban's `extend_ttl(threshold, extend_to)` host call,
described in [State archival on Soroban](state-archival.md#time-to-live).

## What `new` rejects

`TtlPolicy::new` returns `EvergreenError::InvalidPolicy` for two inputs that can
never extend anything:

- `extend_to` of zero;
- `extend_threshold` greater than `extend_to`, which could never trigger.

`new` is a `const fn`, so declaring a policy as a constant with a `match` that
panics on `Err` turns a bad policy into a compile error:

```rust
const BALANCE_POLICY: TtlPolicy = match TtlPolicy::new(17_280, 518_400) {
    Ok(policy) => policy,
    Err(_) => panic!("balance policy constants are invalid"),
};
```

## How a policy is applied

Every store calls `TtlPolicy::resolve` before it extends. `resolve` reads the
network's live maximum TTL with `env.storage().max_ttl()`, then:

1. caps `extend_to` at that maximum;
2. caps the threshold at the new `extend_to`, so it never ends up above it.

The result is a `ResolvedTtl { threshold, extend_to }` that is valid for every
tier. That is why no Evergreen store can make the host return an error for an
extension, even after the network lowers its limits.

The stores apply the resolved policy like this:

| Call | `PersistentStore` | `InstanceStore` |
|---|---|---|
| `set` | Extends the key, after writing | Extends the instance, after writing |
| `get` that finds a value | Extends the key | Extends the instance |
| `get` that finds nothing | No extension | Extends the instance |
| `has` | No extension | No extension |
| `remove` | No extension | No extension |

`InstanceStore` extends on every `get`, even a miss, because any read means the
contract is in use and the whole instance shares one TTL.

An extension only does anything when the remaining TTL is at or below the
threshold. Above it, the host treats the call as a no-op.

## Choosing the numbers

**Set the threshold from how often the entry is touched.** The threshold must be
longer than the longest gap you expect between two accesses. If a balance is
read at least once a week, a threshold of a few weeks keeps it alive with room to
spare. If you cannot name a gap, the entry probably needs the monitor rather than
a bigger number (see [rarely-written-state](../examples/rarely-written-state.md)).

**Set the target from how much rent you want to pay at once.** A longer
`extend_to` means fewer extensions but more rent each time one happens, paid by
whichever transaction triggers it. Today there is no per-access cap; issue
[#8](https://github.com/vegaforge/evergreen/issues/8) adds one.

**Expect new entries to start above your target on some networks.** A new
persistent entry starts with the network's minimum persistent TTL. On mainnet
that was about 120 days on 2026-10-29, longer than the 30-day target in the
examples, so a fresh entry is not extended until it has aged down to the
threshold. The store never shortens a TTL.

**Use the same policy constant for data with the same access pattern.** Policies
are `Copy` and cheap to construct stores from, so one constant per kind of data
(balances, configuration, and so on) keeps the choices in one place.

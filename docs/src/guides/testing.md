# Testing storage lifetimes

The useful test for an Evergreen contract is not "the value was stored" but "the
value is still alive after time passes". This page shows how to write that test
with `soroban-sdk`'s test utilities. It follows the
`balances_stay_alive_through_reads` test in the
[`token-with-policy`](../examples/token-with-policy.md) example.

## What you need

Enable the `testutils` feature for tests:

```toml
[dev-dependencies]
soroban-sdk = { version = "26", features = ["testutils"] }
```

Then import the traits that add ledger control and TTL reads:

```rust
use soroban_sdk::testutils::storage::Persistent as _;
use soroban_sdk::testutils::{Address as _, Ledger as _};
```

`get_ttl` only exists in the test utilities. Contract code never reads a TTL; it
calls `extend_ttl` and lets the host decide.

## The pattern

1. Write the entry through the contract.
2. Check its TTL landed on the policy target.
3. Move the ledger forward until the entry is close to archival.
4. Call an ordinary contract function that reads the entry.
5. Check the TTL is back at the target.

```rust
#[test]
fn balances_stay_alive_through_reads() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let id = env.register(Token, (admin.clone(),));
    let client = TokenClient::new(&env, &id);
    let user = Address::generate(&env);

    client.mint(&user, &500);

    let key = DataKey::Balance(user.clone());
    // The target is capped at the network maximum, as the store does.
    let target = env.as_contract(&id, || TARGET.min(env.storage().max_ttl()));

    let ttl = env.as_contract(&id, || env.storage().persistent().get_ttl(&key));
    assert_eq!(ttl, target);

    // Move forward until the balance is nearly archived.
    let start = env.ledger().sequence();
    env.ledger().set_sequence_number(start + target - 100);

    // A plain read brings it back. The contract never calls extend_ttl.
    let _ = client.balance(&user);
    let ttl = env.as_contract(&id, || env.storage().persistent().get_ttl(&key));
    assert_eq!(ttl, target);
}
```

TTL reads and `max_ttl()` need a contract context, which is what
`env.as_contract(&id, || ...)` provides.

## Test the boundary

The threshold is inclusive: an entry with exactly `extend_threshold` ledgers left
is extended. Add one test that lands exactly on the threshold and one that stops
a ledger above it, so a change to the comparison cannot slip through.

## Test limits are not network limits

`Env::default()` uses the SDK's test network settings, which differ from mainnet
and testnet. For example, the test default for `max_entry_ttl` is 6,312,000
ledgers, about twice mainnet's. Write tests against `env.storage().max_ttl()`
rather than a number copied from the network. The live values are listed in
[State archival on Soroban](../concepts/state-archival.md#network-limits).

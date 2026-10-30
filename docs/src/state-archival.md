# State archival background

Soroban refuses to let the ledger grow without bound. To do that, it archives
state that is not kept alive. Understanding that model is the key to
understanding what Evergreen does, so this page covers it before the
architecture.

## Three storage tiers

A Soroban contract reads and writes through three independent key-value maps,
each physically stored in the ledger:

- **Persistent.** The default for data that must remain accessible, such as
  balances and token metadata. When its TTL lapses it is archived, not deleted,
  and can be restored.
- **Temporary.** The cheapest tier. When its TTL lapses it is deleted for good and
  can never be restored. Only safe for data the contract can recompute or
  discard.
- **Instance.** Tied to the contract instance and sharing the instance TTL. Best
  for a small amount of global state, such as an admin address.

## Time to live

Every entry carries a TTL measured in ledgers, roughly five seconds each. The TTL
must be extended periodically or the entry is archived. The host call that does
this has the shape `extend_ttl(threshold, extend_to)`: when an entry's remaining
TTL is at or below `threshold`, extend it so it lives for `extend_to` more
ledgers. An entry with exactly `threshold` ledgers left is extended. Extending an
entry whose TTL is above the threshold is a no-op.

### Network limits

TTL limits are network parameters, not constants. The network can change them
through a configuration upgrade, so Evergreen never hardcodes them. Read them:

- on chain, with `env.storage().max_ttl()`, which is what `TtlPolicy::resolve`
  does on every extension;
- off chain, with `stellar network settings --network <name>`, in the
  `state_archival` section.

`max_ttl()` is one less than the network's `max_entry_ttl` setting. The host
counts the current ledger as part of an entry's life, so the furthest an entry
can live is `sequence + max_entry_ttl - 1` (`ledger_info.rs` in
`soroban-env-host` 26.1.3), and `max_ttl()` is that ledger minus the current
sequence. A new persistent entry starts with `min_persistent_ttl - 1` ledgers by
the same counting.

What happens when an extension asks for more than the maximum depends on the
tier. Persistent entries and the contract instance are capped at the maximum by
the host, with no error. A temporary entry returns an error. The host also
returns an error if `threshold` is greater than `extend_to`. `TtlPolicy::resolve`
caps the target and keeps the threshold at or below it before calling the host,
so neither error can come from an Evergreen store.

As an example only, these were the values on 2026-10-29, read with
`stellar network settings` (stellar-cli 27.1.0). Day figures assume five-second
ledgers.

| Setting | Mainnet | Testnet | `soroban-sdk` 26.1.0 test default |
|---|---|---|---|
| `max_entry_ttl` | 3,110,400 (about 180 days) | 3,110,400 | 6,312,000 |
| `min_persistent_ttl` | 2,073,600 (about 120 days) | 120,960 (about 7 days) | 4,096 |
| `min_temporary_ttl` | 17,280 (about 1 day) | 720 | 16 |

Check the live values before relying on any of these numbers.

### What an extension costs

Extending an entry during a read does not move it into the read-write
footprint. A read through `PersistentStore::get` that extends the entry leaves
the key in the read-only footprint; the transaction pays rent for the new TTL.
This was checked on testnet on 2026-10-29: a contract whose policy extends on
every read had its entry's `liveUntilLedgerSeq` raised from 5,952,979 to
5,952,991 by a transaction whose footprint listed the key as read-only and whose
read-write footprint was empty.

## Restoration

An archived persistent or instance entry is brought back with
`RestoreFootprintOp`. The operation has rules: the read-only footprint must be
empty, it must be the only operation in its transaction, and a restored entry
gets the network-minimum new TTL. An archived persistent entry can never be
recreated from scratch, only restored.

Protocol 23 (CAP-0066) softened the read path. An archived persistent or instance
entry can be automatically restored before a host function runs, but only if it
is in the transaction's restore list, which the RPC simulation populates when it
detects access to an archived entry. This helps reads. It does not manage
extension proactively, and it does not help with tier selection or upgrade-time
migration.

## Where developers stumble

The model is powerful, and the unusual parts are exactly where teams trip:
choosing the wrong tier and losing data, forgetting to extend and watching state
archive, building restore footprints by hand, and having no tooling at all for
migrating stored data across an upgrade. Evergreen exists to take those off the
contract author's plate.

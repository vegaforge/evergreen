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
TTL drops below `threshold`, extend it so it lives for `extend_to` more ledgers.
Extending an entry whose TTL is already healthy is a no-op.

A single extension is capped at the network maximum, which is around 535,679
ledgers, or about 30 days. The maximum is a network parameter and can change
through a protocol upgrade, so it should be read from the network rather than
hardcoded.

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

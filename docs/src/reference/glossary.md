# Glossary

**Archival.** What happens to a persistent or instance entry when its TTL
lapses. The entry leaves the live ledger but can be restored. Temporary entries
are deleted instead.

**Extension.** Raising an entry's TTL with `extend_ttl(threshold, extend_to)`.
It only takes effect when the remaining TTL is at or below the threshold.

**Footprint.** The list of ledger keys a transaction declares it will read
(read-only) or write (read-write). Extending an entry during a read keeps it in
the read-only footprint.

**Instance storage.** A key-value map tied to the contract instance. All its
keys share one TTL with the instance.

**Ledger.** One closed block of the Stellar network, about five seconds. TTLs are
counted in ledgers.

**`max_ttl`.** The furthest an entry can be extended, read on chain with
`env.storage().max_ttl()`. It is one less than the network's `max_entry_ttl`
setting.

**Persistent storage.** The tier for data that must survive. Entries are archived,
not deleted, when their TTL lapses.

**Policy.** A `TtlPolicy`: the threshold and target Evergreen uses to extend an
entry.

**Rent.** What a transaction pays to keep entries in the live ledger. Extending
an entry's TTL costs rent.

**Restore.** Bringing an archived entry back with `RestoreFootprintOp`, or
automatically through the restore list that RPC simulation fills in since
Protocol 23.

**Temporary storage.** The cheapest tier. Entries are deleted for good when their
TTL lapses and cannot be restored.

**Threshold.** The remaining TTL, in ledgers, at or below which an extension
happens. Inclusive.

**TTL (time to live).** How many more ledgers an entry stays in the live ledger.

**Target (`extend_to`).** How many ledgers an entry should have left after an
extension.

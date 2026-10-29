# rarely-written-state

A contract whose one piece of state is written once and then mostly read.

It exists to show the limit of the on-chain library. `evergreen-core` extends an
entry's TTL on access, so a contract that is touched often keeps itself alive.
This one is not. After the initial write it might sit untouched for long
stretches, and extend-on-access cannot help an entry that nothing is accessing.

That is the gap the off-chain `evergreen-monitor` closes: it watches the entry
and extends it on a schedule between accesses. See the
[roadmap](../../docs/src/roadmap.md#phase-2-the-off-chain-monitor).

```bash
cargo test -p rarely-written-state
cargo build -p rarely-written-state --target wasm32v1-none --release
```

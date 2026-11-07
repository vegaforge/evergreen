# Introduction

Evergreen is a framework that abstracts Soroban's storage lifecycle. It keeps a
contract's storage alive, correctly tiered, and safe to migrate across upgrades,
so the people who build on Soroban can declare a storage policy once and stop
managing TTLs by hand.

## The one-sentence version

Set the policy once and stop thinking about archival.

## Who it is for

- **Contract authors** who want storage to stay alive and correctly tiered
  without writing extension logic.
- **Protocol operators** running deployed contracts who need a monitor that keeps
  state from archiving and restores it when it does.
- **Teams upgrading contracts** who need to migrate stored data safely across a
  Wasm upgrade.

If your contract is trivial and short-lived, archival may never bite you, and raw
storage is fine. Evergreen earns its place once a contract holds data that must
survive.

## What works today

Evergreen is built in phases. This table says what you can use now and what is
still being built, so nothing in this book reads as more finished than it is.

| Crate | What it does | Status |
|---|---|---|
| `evergreen-core` | Typed storage handles that extend TTL on access | Usable |
| `evergreen-lint` | Flags durable data in temporary storage | Interface only |
| `evergreen-monitor` | Watches, extends and restores entries over RPC | Interface only |
| `evergreen-cli` | The `evergreen` command | Interface only; commands print where the work is tracked |
| `evergreen-migrate` | Dry-run and execute storage migrations | Interface only |
| `evergreen-js` | TypeScript binding for the monitor | Interface only |

"Interface only" means the types and function signatures are in place and the
work is tracked in a GitHub issue. The [roadmap](project/roadmap.md) lists them.

## How this book is organized

- **Getting started** adds `evergreen-core` to a contract and keeps a balance
  alive.
- **Concepts** explains Soroban's storage model, how a TTL policy works, and how
  the layers fit together. Read
  [State archival on Soroban](concepts/state-archival.md) first if TTL, archival
  and restore are new to you.
- **Guides** cover specific tasks: picking the right store and testing that your
  entries stay alive.
- **Examples** walk through the contracts in the repository's `examples/`
  folder.
- **Reference** lists the public API, the CLI commands and the terms used in this
  book.
- **Project** has the roadmap, how to contribute, and the changelog.

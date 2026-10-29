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

## How this book is organized

- [State archival background](state-archival.md) explains the Soroban storage
  model Evergreen sits on top of. Read this if the terms TTL, archival, and
  restore are new to you.
- [Architecture](architecture.md) walks through the three layers and how they fit
  together.
- [Getting started](getting-started.md) gets you building with `evergreen-core`.

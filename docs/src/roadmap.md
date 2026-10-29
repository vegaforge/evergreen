# Roadmap

Evergreen is built in phases. Each phase delivers one layer that is useful on its
own. This page lists the phases, what each one is for, and the GitHub issues that
track the work. The [issue tracker](https://github.com/vegaforge/evergreen/issues)
is the source of truth for status; this page explains how the pieces fit.

Issues labelled `wave` are open to outside contributors through Drips Wave.
Issues labelled `maintainer` are reserved.

## Phase 0: the on-chain library (shipped)

**Goal:** a contract author declares a TTL policy once and never writes extension
logic by hand.

`evergreen-core` provides `TtlPolicy`, `PersistentStore`, `InstanceStore` and
`Tier`. Every extension is clamped to the network's live maximum TTL. Version
0.1.0 is described in the [changelog](https://github.com/vegaforge/evergreen/blob/main/CHANGELOG.md).

Follow-up work on the core:

- Correct the TTL facts in the docs and lock the inclusive threshold with a test:
  [#18](https://github.com/vegaforge/evergreen/issues/18).
- `TemporaryStore`, with a clamp that cannot trap:
  [#7](https://github.com/vegaforge/evergreen/issues/7).
- An optional cap on how far a single access extends, to bound rent per call:
  [#8](https://github.com/vegaforge/evergreen/issues/8).
- Measure the resource cost of extend-on-access against raw storage:
  [#9](https://github.com/vegaforge/evergreen/issues/9).
- Crate metadata for a first crates.io release:
  [#15](https://github.com/vegaforge/evergreen/issues/15).

## Phase 1: the tier lint

**Goal:** routing durable data to temporary storage, the one storage mistake that
loses data for good, becomes a visible finding in review instead of a silent bug.

- `evergreen-lint`: the durable-in-temporary rule, built on `syn`:
  [#10](https://github.com/vegaforge/evergreen/issues/10).
- `evergreen lint <path>` in the CLI, with exit codes and JSON output:
  [#11](https://github.com/vegaforge/evergreen/issues/11).

## Phase 2: the off-chain monitor

**Goal:** entries that are written too rarely to stay alive through
extend-on-access are extended or restored before anyone notices, within an
explicit rent budget.

- `Monitor::scan`, reading entry TTLs over Stellar RPC:
  [#1](https://github.com/vegaforge/evergreen/issues/1).
- `Monitor::predict`, classifying archival risk with the entry's durability in
  mind (an expired temporary entry is gone, not archived):
  [#2](https://github.com/vegaforge/evergreen/issues/2).
- `Monitor::extend_due`, building `ExtendFootprintTTL` transactions that stop when
  the rent budget is spent, so the monitor cannot be driven into unbounded spend:
  [#3](https://github.com/vegaforge/evergreen/issues/3).
- `Monitor::restore_archived`, preferring the Protocol 23 simulation-driven
  restore and falling back to an explicit `RestoreFootprint`:
  [#4](https://github.com/vegaforge/evergreen/issues/4).

## Phase 3: the CLI

**Goal:** one command surface for everything outside a long-running monitor.

- `evergreen inspect`, with table and JSON output:
  [#5](https://github.com/vegaforge/evergreen/issues/5).
- `evergreen forecast`, projecting rent from simulation:
  [#6](https://github.com/vegaforge/evergreen/issues/6).
- `evergreen extend` and `evergreen restore` call the monitor directly and land
  with [#3](https://github.com/vegaforge/evergreen/issues/3) and
  [#4](https://github.com/vegaforge/evergreen/issues/4).
- `evergreen upgrade` has no issue yet. It depends on the upgrade model settled in
  Phase 4.

## Phase 4: upgrades and migrations

**Goal:** a change to the shape of stored data across a Wasm upgrade is dry-run
first, shows the full diff, and comes with a rollback plan.

- Make `migration-demo` upgradeable and add a v2 with an on-chain `migrate`
  entrypoint: [#12](https://github.com/vegaforge/evergreen/issues/12).
- The in-memory dry-run planner and rollback plan in `evergreen-migrate`:
  [#13](https://github.com/vegaforge/evergreen/issues/13).

## TypeScript binding

**Goal:** teams whose operations tooling runs on Node can drive the monitor
without adopting Rust on the ops side. The binding mirrors the Rust monitor; it is
not a second implementation.

- `Monitor.scan` and `Monitor.predict` with `@stellar/stellar-sdk`, after the Rust
  `predict` semantics land: [#14](https://github.com/vegaforge/evergreen/issues/14).

## Design questions

### Does the storage transform share the upgrade transaction?

Answered: no, not inside the upgrade call. Only a contract's own code can write
its storage, so an off-chain engine cannot apply a migration directly. And after
`update_current_contract_wasm`, the new code takes effect only when the current
invocation finishes, so v2's migration logic cannot run inside v1's `upgrade`
call. The model Evergreen follows:

1. v1 exposes an admin-guarded `upgrade` entrypoint.
2. v2 exposes an admin-guarded, version-monotonic, idempotent `migrate`
   entrypoint.
3. `evergreen-migrate` dry-runs the transforms off-chain against the current
   entries, then sequences the two calls, or calls an upgrader contract that
   makes both in one transaction.

[#12](https://github.com/vegaforge/evergreen/issues/12) builds this into the
example, and [#13](https://github.com/vegaforge/evergreen/issues/13) builds the
dry-run.

### Where do types shared by on-chain and off-chain crates live?

Open. `evergreen-migrate` keeps its own copy of `Tier` so off-chain builds do not
pull in the `no_std` on-chain crate and `soroban-sdk`. A small shared crate that
builds both with and without `std` would remove the copy. It needs an ADR before
anyone starts.

### How should the monitor identify entries?

`EntryKey` is a string for now. It becomes the XDR `LedgerKey` once the monitor
depends on `stellar-xdr`, as part of
[#1](https://github.com/vegaforge/evergreen/issues/1).

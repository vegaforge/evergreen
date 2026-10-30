---
name: evergreen-engineering-standards
description: Engineering standards for Evergreen, the Soroban storage-lifecycle framework. Read before writing or reviewing any code, comment, or document in this repository.
---

# Evergreen Engineering Standards

These rules apply to every contributor, human or agent. They are not suggestions.
`AGENTS.md` holds the governance rules; this file holds the craft rules.
`docs/src/architecture.md` explains the design for users.

## Architecture

Evergreen is a Cargo workspace with one TypeScript package beside it. The layers,
from the bottom up:

```
                     evergreen-cli  (std binary `evergreen`)
                      |          |
          evergreen-monitor   evergreen-migrate      evergreen-lint
          (std library)       (std library)          (std library)

  examples/*  (no_std contracts, wasm32v1-none)
      |
  evergreen-core  (no_std, compiled into contracts)
      |
  soroban-sdk

  bindings/evergreen-js  (TypeScript, mirrors evergreen-monitor's types)
```

- `evergreen-core` is the on-chain layer. It is `#![no_std]`, depends only on
  `soroban-sdk`, and is linked into every contract that uses Evergreen. It must
  never depend on a `std` crate or on any other workspace crate.
- `evergreen-monitor`, `evergreen-migrate` and `evergreen-lint` are off-chain `std`
  libraries. They do not depend on each other. They do not depend on
  `evergreen-core` either, so they do not pull the on-chain crate and
  `soroban-sdk` into off-chain builds. `evergreen-migrate` keeps its own copy of
  `Tier` for that reason, until shared types have a home that works for both
  sides.
- `evergreen-cli` sits on top. It may depend on any off-chain library. Nothing
  depends on it.
- `examples/` are contracts that depend on `evergreen-core` and `soroban-sdk`
  only. They are documentation that compiles.
- `bindings/evergreen-js` is a thin TypeScript surface over the monitor. It mirrors
  the Rust types by hand and is not a second implementation.

Dependencies point downward only. A lower layer never imports a higher one, and the
three off-chain libraries never import each other.

What enforces it today:

- `#![no_std]` in `crates/evergreen-core/src/lib.rs`, plus gate 5 of the preflight,
  which builds every example for `wasm32v1-none`. That target has no `std`, so a
  `std` dependency anywhere under core fails the build.
- Each crate's `Cargo.toml` lists its dependencies explicitly. A new path
  dependency is a design change and needs an ADR.
- Review. There is no automated check for the off-chain import direction yet.

## Correctness

- Verify each implementation before moving to the next. Do not stack unverified work.
- No shortcuts: no `unwrap()` or `expect()` on external data (RPC responses, files,
  arguments, user input). In tests and in `const` policy definitions they are fine,
  because a panic there is the point.
- No `#[allow(...)]` to silence clippy or rustc unless the same item carries a
  comment saying why the lint is wrong here.
- No `unsafe`. Nothing in this workspace needs it.
- Libraries return typed errors (`#[contracterror]` on-chain, a crate error enum
  off-chain). No stringly typed errors, no `Box<dyn Error>` in public APIs.
- No damage control: find the cause and fix it, never the symptom.
- Do not guess Soroban storage or TTL semantics. Read the `soroban-sdk` and
  `soroban-env-host` source in `~/.cargo/registry`, or the current Stellar docs,
  and cite what you read in the ADR or PR.
- Network limits (maximum TTL, minimum persistent TTL, contract size) are network
  parameters. Read them at run time (`env.storage().max_ttl()`) or state the date
  and source when a number appears in docs.
- Tests prove behaviour against the `soroban-sdk` test utilities: the happy path,
  the invalid policy, the boundary (remaining TTL equal to the threshold), and the
  ledger advancing. Commit the test snapshots and review their diffs.
- Off-chain tests never touch the network. Use recorded JSON fixtures.

## Comments and naming

- Write every comment by hand, in plain English. Short, clear, specific.
- Default to no comment. A comment explains why, or a non-obvious what.
- No AI tone, no filler. Do not use em-dashes; use a comma, colon, or period.
- Name things for what they are. Ledger counts are ledgers, not "time".
- Public items carry rustdoc. `cargo doc` runs with warnings denied, so broken
  intra-doc links fail the preflight.

## Documentation

- Technically correct, unambiguous, jargon defined on first use, examples runnable.
- A number that comes from the network (TTL limits, size limits) carries the date
  it was read and where it came from.
- `README.md`, the crate READMEs, and the mdBook under `docs/src/` describe the
  same behaviour. Change them together.

## Git and review

- One issue, one branch, one pull request. Branch prefixes: `feat/`, `fix/`,
  `chore/`, `docs/`.
- Never push to `main`. Squash merges only. Branch deleted on merge.
- Commits are atomic with a Conventional Commit subject (`feat(core): ...`) and a
  body that says what changed and why.
- Never add AI co-author trailers or session footers to commits or PRs.
- Never commit or push on a contributor's behalf without being asked.
- Do not commit secrets. `.env` files stay out of git. No Stellar secret seeds in
  tests, fixtures, or docs.

## Tooling

- Rust is pinned in `rust-toolchain.toml`. The declared minimum (`rust-version` in
  `Cargo.toml`) is checked by the `msrv` CI job. Bump either in its own PR.
- Node for the binding is pinned in `bindings/evergreen-js/.nvmrc`; install with
  `npm ci` so the lockfile is respected.
- Run `./scripts/preflight.sh` before opening a pull request. CI runs the same
  gates and the `quality` job is the required check.
- Git hooks come from `lefthook.yml`. They are a convenience; CI decides.
- When a task needs a library, read its current docs first; do not code from memory.

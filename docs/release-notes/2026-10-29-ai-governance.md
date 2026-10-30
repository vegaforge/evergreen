---
date: 2026-10-29
feature: AI governance rulebook, five-gate preflight, pinned toolchain and CI quality check
scope: chore
scenario-impact: none
---

# AI governance and preflight

Contributors and agents now have one command that says whether a change is ready:
`./scripts/preflight.sh`. It runs format, clippy, check plus rustdoc plus the
TypeScript typecheck, tests, and the wasm32v1-none build of every example with a
size check against the 131,072-byte mainnet limit. `--fast` skips the build and is
what the optional lefthook pre-push hook runs.

Technical shape:

- `rust-toolchain.toml` pins Rust 1.91.1. `rust-version` in `Cargo.toml` is now
  `1.91`, the minimum `soroban-sdk 26.1.0` accepts; it was `1.81`, which could not
  build the workspace.
- CI is split into `checks`, `msrv`, `tests` and `build`, aggregated by a
  `quality` job that branch protection can require. Builds use `--locked`.
- `bindings/evergreen-js` is renamed `@vegaforge/evergreen-js`, marked private,
  and has a committed lockfile and `.nvmrc`.
- Shipped-code changes need a release note under `docs/release-notes/`.

ADR: `docs/adr/ADR-2026-10-29-ai-governance.md`. Closes #16.

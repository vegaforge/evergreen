## Summary

<!-- What does this PR do, and why? Which crate or layer does it touch? -->

## Scenario Impact Declaration

- [ ] **scenario-impact**: `updated` | `needs_automation` | `none`
- [ ] Added a release note in `docs/release-notes/<date>-<topic>.md` (required for changes under `crates/`, `bindings/`, `examples/`, or the root `Cargo.toml`)

## Related

<!-- Closes #123 -->

## Type of change

- [ ] Feature
- [ ] Bug fix
- [ ] Refactor / chore
- [ ] Docs

## Governance & Pre-Review Checklist

- [ ] **Worktree Isolation**: work was done in an isolated branch and worktree off `main`.
- [ ] **Human Approval**: both ADRs and the plan approved by the maintainer (if non-trivial).
- [ ] **TTL Clamp**: every extension goes through `TtlPolicy::resolve`; no hardcoded network limit.
- [ ] **Tier Choice**: data that must survive is not in temporary storage.
- [ ] **No On-Chain TTL Reads**: `get_ttl` appears only in tests.
- [ ] **Resource Cost**: new storage access or extension cost is stated (and measured if the ADR asked).
- [ ] **no_std and wasm**: `evergreen-core` stays `no_std`; examples build for `wasm32v1-none` under the size limit.
- [ ] **Migration Safety**: storage changes run through a guarded, version-monotonic entrypoint with a dry-run and rollback plan (if applicable).
- [ ] **Boundary Validation**: external data is parsed into typed values; no `unwrap`/`expect` on it.
- [ ] **Typed Errors**: public APIs return typed errors; no new `#[allow(...)]` without a written reason; no `unsafe`.
- [ ] **Tests**: written against the `soroban-sdk` test utilities, including boundary cases; snapshot diffs reviewed.
- [ ] **Test-First Fix**: the bug fix was preceded by a failing regression test.
- [ ] **Layering**: dependencies point downward; `evergreen-core` depends on nothing but `soroban-sdk`.
- [ ] **Docs Agree**: rustdoc, READMEs and the mdBook say the same thing.
- [ ] **Preflight Gate**: `./scripts/preflight.sh` passed with zero errors and warnings.

## How this was verified

<!-- The exact commands you ran and what came back. For network-dependent
     behaviour, the testnet simulation and its result. -->

## Notes

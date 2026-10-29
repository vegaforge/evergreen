# Contributing to Evergreen

Thanks for your interest in Evergreen. This is open-source infrastructure for the
Soroban ecosystem, and contributions of every size are welcome.

## Ground rules

- Be respectful and assume good faith.
- Keep changes focused. One concern per pull request makes review faster.
- Discuss large changes first. Open an issue before starting significant work so
  we can agree on the approach.

## Project layout

Evergreen is a Cargo workspace. Each crate is scoped so you can work on one
without understanding the whole framework:

- `crates/evergreen-core` is the on-chain library. Changes here affect every
  contract that depends on Evergreen, so they get the closest review.
- `crates/evergreen-monitor`, `crates/evergreen-cli`, `crates/evergreen-migrate`,
  and `crates/evergreen-lint` are off-chain crates. Several are still stubs, with
  typed interfaces and TODO markers that point at the GitHub issue tracking the
  work. `docs/src/roadmap.md` shows how those issues fit together, and the ones
  labelled `wave` are open to outside contributors.
- `examples/` holds working contracts that double as documentation.

## Setup

You need a stable Rust toolchain and the Soroban Wasm target. Current Soroban
builds contracts for `wasm32v1-none`, not the older `wasm32-unknown-unknown`.

```bash
rustup target add wasm32v1-none
```

## Before you open a pull request

Run the same checks CI runs, from the repository root:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p token-with-policy --target wasm32v1-none --release
```

All four must pass. CI runs them on every pull request.

## Conventions

- **Formatting and lints.** Code is formatted with `rustfmt` and must be clean
  under `clippy` with warnings denied.
- **Errors.** Library code returns typed errors. Do not use `unwrap` or `expect`
  on a path that can fail at run time.
- **Comments.** Write them like a person, short and to the point. Explain why,
  not what the code already says.
- **Storage API.** Do not guess Soroban storage or TTL method names. Use the
  documented `soroban-sdk` interface and verify against the current docs.
- **Tests.** New behavior comes with tests. On-chain code uses the `soroban-sdk`
  test utilities.

## Commit and pull request style

- Write clear commit messages in the imperative mood, for example "add restore
  helper" rather than "added" or "adds".
- Fill in the pull request template. Describe what changed and how you verified
  it.

## License

By contributing, you agree that your contributions are licensed under the
Apache-2.0 License, the same license as the project.

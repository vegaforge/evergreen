## What this changes

A short description of the change and the motivation behind it.

## Related issue

Closes #

## How it was verified

Describe how you tested this. For most changes that means the full check set,
run from the repository root:

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `cargo build -p <contract> --target wasm32v1-none --release` (if a contract changed)

## Checklist

- [ ] The change is focused on one concern.
- [ ] New behavior has tests.
- [ ] Library code returns typed errors rather than panicking.
- [ ] Public items are documented.
- [ ] No Soroban storage or TTL method names were guessed; they match the SDK.

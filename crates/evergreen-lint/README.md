# evergreen-lint

Lints that flag Soroban storage tier misuse.

The sharpest mistake in the Soroban storage model is putting durable data, such
as a balance, in temporary storage, where it vanishes for good once its TTL
lapses. This crate turns that from a silent bug into a flagged, visible choice
that shows up in code review.

## Status

Stub. The types in `src/lib.rs` are the intended interface; `TierLint::check`
returns `LintError::NotImplemented`. This is a Phase 1 deliverable. See PRD
section 7.1.

## Related work

The OpenZeppelin Soroban security detectors and CoinFabrik's Scout cover adjacent
ground. Evergreen's lint is focused specifically on tier and TTL discipline and
pairs with the type-level guardrails in `evergreen-core`.

# Engineering Standards & AI Governance Rulebook

All AI coding agents (Claude Code, Codex, Antigravity, Qwen Code, and custom LLM dev
harnesses) operating in this repository MUST abide by the rules, architectural
layering, and lifecycle in this document and in
`.claude/skills/evergreen-engineering-standards/SKILL.md`.

Human contributors follow the same gates. `CONTRIBUTING.md` is the short version
for people; this file is the full version for agents.

---

## Core Mandatory AI Governance Directives

### Directive 1: Read-Only Long-Lived Branches & Mandatory Worktree Isolation

Direct commits or pushes to `main` are strictly forbidden for all AI agents. `main`
is the only long-lived branch in this repository.
All feature development, refactoring, and bug fixing MUST be conducted in isolated Git
worktrees created off `main`, using `git worktree add`.
AI agents must never assume a local `main` is current without explicitly syncing
ground truth.

### Directive 2: Human Approval Gate (Phase 1 DECIDE)

AI agents are strictly prohibited from implementing non-trivial features,
architectural changes, or framework modifications without authoring TWO Architectural
Decision Records under `docs/adr/`:

1. Technical ADR (`docs/adr/ADR-<YYYY-MM-DD>-<topic>.md`): full technical
   specification covering context, decision, architectural impact, trade-offs, and
   a component or sequence diagram.
2. Simplified Companion ADR (`docs/adr/ADR-<YYYY-MM-DD>-<topic>-for-dummies.md`):
   plain-English context, decision, and impact for non-technical stakeholders.

- An AI agent is strictly forbidden from approving its own ADRs or implementation plan.
- Execution MUST NOT begin until explicit, unambiguous human approval is granted on
  both ADR documents. Silence is not approval. A question about the ADR is not approval.

Anything that changes on-chain behaviour in `evergreen-core`, the public API of any
crate, the storage layout of an example contract, or the migration model is
non-trivial by definition.

### Directive 3: Red-Green Test-First Bug Fixing Protocol

When addressing any bug, regression, or defect, the AI agent MUST follow this order:

1. Red: write a failing unit or integration test that reproduces the defect. Run it
   and confirm it fails for the exact reason identified.
2. Green: implement the minimal production fix. Re-run and confirm clean passage with
   no secondary regressions.

On-chain behaviour is tested against the `soroban-sdk` test utilities
(`Env::default()`, `testutils::Ledger`, `testutils::storage`), not against a mock of
the host.

### Directive 4: Prohibition of Error Swallowing & Symptom Masking

AI agents are forbidden from resolving build errors, type errors, or test failures by
masking them:

- Do NOT discard a `Result` with `let _ =`, `.ok()`, or an empty `match` arm.
- Do NOT call `unwrap()` or `expect()` on data that comes from outside the program
  (RPC responses, files, user input, contract arguments).
- Do NOT add `#[allow(...)]` to silence a clippy or rustc lint without a comment on
  the same item that says why the lint is wrong in this case.
- Do NOT add `unsafe`, `#[ignore]` on a failing test, or `#[cfg(not(test))]` to hide
  code from the test build.
- Do NOT return dummy, empty, or default values to hide missing data or a broken
  upstream.
- Do NOT comment out, weaken, or delete failing assertions or test cases.
- Do NOT raise a size limit, a timeout, or a threshold to make a gate pass. Fix the
  cause.

### Directive 5: Local Preflight Verification Enforcement

Before submitting any Pull Request or marking a task complete, the AI agent MUST run
`./scripts/preflight.sh`. All five gates (format, lint, typecheck, tests, build)
must complete with zero warnings and zero errors. A gate that passes only when run
alone does not count; the full run is the verdict.

---

## Multi-Stage Development Lifecycle (Phases 0-8)

```
[Phase 0: SYNC] --> [Phase 1: DECIDE] --> [Phase 2: PLAN]
                                                |
[Phase 5: BUILD] <-------------- [Phase 3: ISOLATE]
       |
       v
[Phase 6: VERIFY] --> [Phase 7: DOCUMENT] --> [Phase 8: DELIVER]
```

### Phase 0: SYNC
`git fetch origin main`. Never rely on a stale local ref.

### Phase 1: DECIDE
For non-trivial work, author both ADRs, present them, and pause until the human
approves. Trivial work (a typo, a copy change, a one-line fix with a regression test)
skips this phase; when unsure, it is not trivial.

### Phase 2: PLAN
`docs/plans/<date>-<feature>-plan.md`: crate boundaries, public API signatures,
storage layout changes, test strategy, and the failing tests to write first.

### Phase 3: ISOLATE
`git worktree add .worktrees/<branch-name> -b <branch-name> origin/main`.
Run `npm ci --prefix bindings/evergreen-js` inside the worktree; the preflight needs it.

### Phase 5: BUILD
Test-driven implementation inside the architectural layering rules in SKILL.md.

### Phase 6: VERIFY
`./scripts/preflight.sh`, then exercise the change for real: run the CLI command,
or, for on-chain behaviour the ADR says depends on the network, simulate it against
testnet with the Stellar CLI and record what came back.

### Phase 7: DOCUMENT
`docs/release-notes/<date>-<feature>.md` with the required frontmatter and a
`scenario-impact` declaration (`updated | needs_automation | none`). Required for any
change under `crates/`, `bindings/`, `examples/`, or to the root `Cargo.toml`.

### Phase 8: DELIVER
Open the PR against `main`, using `.github/pull_request_template.md`. All CI checks
green before requesting review.

---

## Mandatory Pre-Review Defect Audit Checklist

1. TTL clamp: every extension goes through `TtlPolicy::resolve`, which clamps the
   target to `env.storage().max_ttl()` and keeps the threshold at or below the
   target. No raw `extend_ttl` call with a hardcoded network limit.
2. Tier choice: data that must survive is never in temporary storage. The tier is
   visible at the call site through `PersistentStore` or `InstanceStore`.
3. No on-chain TTL reads: `get_ttl` exists only in `soroban_sdk::testutils`. On-chain
   code decides with `extend_ttl(threshold, extend_to)`; off-chain code reads TTL
   over RPC.
4. Footprint and resource cost: every new extension or storage access adds to the
   transaction footprint and rent. Say what it costs, and measure it when the ADR
   asks for it.
5. `no_std` and wasm: `evergreen-core` stays `#![no_std]` with no `std` dependency,
   and every example builds for `wasm32v1-none` under the network size limit.
6. Migration safety: only a contract's own code can write its storage, so migrations
   run through an admin-guarded, version-monotonic, idempotent entrypoint, are
   dry-run first, and come with a rollback plan.
7. Boundary validation: off-chain crates parse RPC and file input into typed domain
   values at the edge and return typed errors. Tests use recorded fixtures, never
   the network.
8. Test-first fix verification: every bug fix was locked by a failing test first,
   including boundary cases such as remaining TTL exactly equal to the threshold.
9. Layering: dependencies point the way SKILL.md describes. `evergreen-core` never
   depends on an off-chain crate.
10. Preflight: `./scripts/preflight.sh` passed in full.

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
  typed interfaces and TODO markers that point at the relevant PRD section. Those
  TODOs are good first issues.
- `examples/` holds working contracts that double as documentation.

## Setup

You need:

- Rust through [rustup](https://rustup.rs). The repository pins the toolchain in
  `rust-toolchain.toml` (Rust 1.91.1 with `rustfmt`, `clippy` and the
  `wasm32v1-none` target), and rustup switches to it inside the repository.
  Current Soroban builds contracts for `wasm32v1-none`, not the older
  `wasm32-unknown-unknown`. If your rustup does not install the pinned
  toolchain on its own, run `rustup toolchain install` from the repository root.
- Node.js, at the version in `bindings/evergreen-js/.nvmrc`, for the TypeScript
  binding. Install its dependencies once per clone or worktree:

  ```bash
  npm ci --prefix bindings/evergreen-js
  ```

## Before you open a pull request

Run the preflight from the repository root:

```bash
./scripts/preflight.sh
```

It runs the same five gates CI runs, in order, and stops at the first failure:

1. Format: `cargo fmt --all -- --check`
2. Lint: `cargo clippy --workspace --all-targets --locked -- -D warnings`
3. Typecheck: `cargo check`, `cargo doc` with warnings denied, and `tsc` on the
   binding
4. Tests: `cargo test --workspace --locked`
5. Build: every example for `wasm32v1-none` in release mode, a check that each
   Wasm file fits the network's contract size limit, and the binding's build

`./scripts/preflight.sh --fast` skips gate 5 when you want a quicker answer.

If your change touches `crates/`, `bindings/`, `examples/` or the root
`Cargo.toml`, add a release note: copy `docs/release-notes/TEMPLATE.md` to
`docs/release-notes/<YYYY-MM-DD>-<topic>.md` and fill it in. CI fails without one.

### Git hooks (optional)

The repository ships a [lefthook](https://lefthook.dev) config. With it, each
commit runs rustfmt on the staged Rust files and each push runs
`./scripts/preflight.sh --fast`. Install it once per clone:

```bash
brew install lefthook        # or: npm install --global lefthook
lefthook install
```

The hooks only save you a round trip. CI is the gate that decides a merge, and
the `quality` check must be green.

### Working with AI agents

If you use a coding agent, point it at [AGENTS.md](AGENTS.md). It holds the rules
agents must follow here: work in a git worktree, write an ADR pair under
`docs/adr/` and get approval before non-trivial work, fix bugs test-first, never
mask an error, and pass the preflight. The engineering standards it refers to are
in `.claude/skills/evergreen-engineering-standards/SKILL.md` and apply to people
too.

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

## Drips Wave contributions

Evergreen takes part in Drips Wave. Issues labelled `wave` are scoped for outside
contributors, and the same rules apply to every pull request:

- **Get assigned first.** Comment on the issue to ask for it, and wait to be
  assigned before you start. Pull requests for unassigned issues are not
  reviewed.
- **One issue, one pull request.** Keep the change to what the issue asks for.
  If it grows past one Wave week, say so on the issue so it can be split.
- **Link the issue.** Put `Closes #N` in the pull request body.
- **Add a release note** for any change under `crates/`, `bindings/` or
  `examples/`, in `docs/release-notes/<YYYY-MM-DD>-<topic>.md`.
- **Pass the preflight.** Run `./scripts/preflight.sh` before you open the pull
  request. CI runs the same gates.
- **Ask, do not guess.** Questions about Soroban storage, TTL or RPC behaviour
  belong on the issue.

## License

By contributing, you agree that your contributions are licensed under the
Apache-2.0 License, the same license as the project.

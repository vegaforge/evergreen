# Contributing

Evergreen is open source under Apache-2.0, and contributions of every size are
welcome. The full guide is
[CONTRIBUTING.md](https://github.com/vegaforge/evergreen/blob/main/CONTRIBUTING.md)
in the repository. This page is the short version.

## Find something to work on

Open issues are on [GitHub](https://github.com/vegaforge/evergreen/issues).
Issues labelled `wave` are open to outside contributors through Drips Wave;
issues labelled `maintainer` are reserved. The [roadmap](roadmap.md) shows how
the issues fit together.

## Set up

You need Rust through rustup (the repository pins Rust 1.91.1 with `rustfmt`,
`clippy` and the `wasm32v1-none` target in `rust-toolchain.toml`) and Node.js
for the TypeScript binding:

```bash
npm ci --prefix bindings/evergreen-js
```

## Before you open a pull request

Run the same gates CI runs:

```bash
./scripts/preflight.sh
```

It checks formatting, clippy, `cargo check` and `cargo doc`, the tests, and that
every example builds for `wasm32v1-none` under the network's size limit.

A change under `crates/`, `bindings/`, `examples/` or to the root `Cargo.toml`
needs a release note in `docs/release-notes/`, copied from `TEMPLATE.md`.

## Improve these docs

Every page has an edit button in the top right that opens the file on GitHub.
The book's source is in `docs/src/`, and `docs/src/SUMMARY.md` sets the order of
the pages.

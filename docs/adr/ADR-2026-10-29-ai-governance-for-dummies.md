# ADR for Dummies: AI governance, preflight gates and CI quality check

- **Status:** Accepted 2026-10-29
- **Date:** 2026-10-29

## The Idea

Evergreen is about to get help from outside contributors and from AI coding
agents. This change writes down the rules they follow and makes the computer check
them, so the maintainer decides what gets built and the machine proves it works
before anything is merged.

## What the Person Sees

1. A contributor (or an agent) picks an issue and works on it in its own branch.
2. For anything bigger than a small fix, they first write two short decision
   documents, one technical and one plain-English, and wait for the maintainer to
   say yes.
3. Before pushing, they run one command, `./scripts/preflight.sh`. It checks
   formatting, lints, types, tests, and that the example contracts still build
   and fit the network's size limit.
4. If they installed the optional git hooks, formatting is fixed on every commit
   and the quick checks run on every push.
5. On the pull request, GitHub runs the same checks and shows one green tick
   called `quality`. The maintainer merges only when it is green.

## Important Limitation

The rules only bind people and agents who follow them, and the hooks can be
skipped. The real guarantee is the `quality` check on GitHub, and that only blocks
a merge once the maintainer turns on branch protection, which is a manual setting.

## What Changes

- New rulebook files: `AGENTS.md`, `CLAUDE.md`, and an engineering standards guide.
- One preflight script and an optional git hook setup.
- The Rust version is pinned, so everyone builds with the same compiler.
- CI is split into clear jobs, adds documentation, minimum-version and TypeScript
  checks, and ends in a single `quality` result.
- Changes to shipped code need a short release note.
- The TypeScript package gets the project's own name and cannot be published by
  accident.

## What Does Not Change

- No Rust library or contract behaviour changes.
- Nothing is published to crates.io or npm.
- Contributors who do not use AI agents follow the same steps they would anyway.

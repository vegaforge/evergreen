---
date: 2026-10-29
feature: roadmap chapter replaces references to an unpublished requirements document
scope: docs
scenario-impact: none
---

# Roadmap chapter and issue links

The code, docs and CLI used to point readers at numbered sections of a product
requirements document that was never published. They now point at a new mdBook chapter, `docs/src/roadmap.md`, and at
the GitHub issue that tracks each piece of work.

- The roadmap lists Phase 0 (core) through Phase 4 (migrate) and the TypeScript
  binding, with the goal of each phase and its issues.
- It records the design questions the old references carried. The question of
  whether the storage transform can share the upgrade transaction is answered:
  it cannot run inside the upgrade call, so v2 exposes a guarded `migrate`
  entrypoint and the engine sequences the two calls.
- `evergreen inspect`, `extend`, `restore`, `forecast` and `migrate` now print
  the issue URL that tracks them when they report that they are pending.
  `evergreen upgrade` points at the roadmap, since it has no issue yet. Exit
  codes are unchanged.
- `TODO` markers in the stub crates and the binding name their issue number.
- The feature request template and CONTRIBUTING link the roadmap.

Closes #19.

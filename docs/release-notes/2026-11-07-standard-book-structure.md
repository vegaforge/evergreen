---
date: 2026-11-07
feature: The Evergreen book follows a standard mdBook layout with getting started, concepts, guides, examples, reference and project sections
scope: docs
scenario-impact: none
---

# Evergreen book reorganized into standard sections

The book at https://evergreen-soroban.netlify.app was five flat chapters. It is
now organized the way most Rust project books are:

- **Getting started:** Installation (new) and Quick start.
- **Concepts:** State archival on Soroban, TTL policies (new) and Architecture.
- **Guides:** Choosing a store and Testing storage lifetimes (both new).
- **Examples:** one page per contract in `examples/`.
- **Reference:** the `evergreen-core` API, the planned CLI commands and their
  issues, and a glossary.
- **Project:** Roadmap, Contributing, and the Changelog, included from
  `CHANGELOG.md`.

The introduction now has a table of what works today and what is interface
only, so no page reads as more finished than the code is.

`book.toml` adds an edit link on every page, fails the build if `SUMMARY.md`
names a missing page, and redirects the four old page addresses to their new
locations. Links to the roadmap in crate docs, READMEs, the CLI's pending
messages and the feature request template point to the new location.

No crate behaviour changed.

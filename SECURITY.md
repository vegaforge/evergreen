# Security Policy

## Evergreen's security model

Evergreen never holds funds and never takes custody of anything. It only extends,
restores, and migrates a contract's own storage. That keeps its blast radius
small, but two failure modes still matter and shape the design:

- **Irrecoverable data loss.** Temporary storage cannot be restored once it
  lapses. The tier guardrails and the lint exist to make this mistake loud
  instead of silent.
- **Silent archival.** A forgotten extension lets data archive quietly. The
  on-chain policy and the off-chain monitor exist to prevent it.

Two safety properties are hard gates:

- The migration engine never executes without producing a full dry-run diff
  first, and every migration records a rollback plan.
- The monitor operates within an explicit rent budget, so it cannot be driven
  into unbounded spend.

All TTL operations respect the network minimum and maximum, so Evergreen never
submits an invalid extension.

## Reporting a vulnerability

Please do not open a public issue for security problems.

Report privately through GitHub's [private vulnerability
reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability)
on this repository, under the Security tab. If that is not available to you, open
a minimal issue that asks for a private contact channel, without any details.

When you report, please include:

- A description of the issue and why it is a problem.
- Steps to reproduce, ideally a minimal proof of concept.
- The affected crate and version or commit.

## What to expect

- We aim to acknowledge a report within three business days.
- We will keep you informed as we investigate and work on a fix.
- We will credit you in the release notes once a fix ships, unless you prefer to
  stay anonymous.

## Scope

This policy covers the crates in this repository. The underlying Stellar and
Soroban platform has its own bug bounty program at
https://stellar.org/grants-and-funding/bug-bounty.

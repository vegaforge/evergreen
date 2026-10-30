# ADR for Dummies: Correct the TTL facts and lock the inclusive threshold

- **Status:** Accepted 2026-10-29
- **Date:** 2026-10-29

## The Idea

Evergreen's documentation describes how Soroban keeps data alive, and four of
those descriptions are slightly or badly wrong. The code itself behaves
correctly. This change fixes the words, removes numbers that go stale, and adds
tests so the correct behaviour cannot quietly break later.

## What the Person Sees

1. A contract author reads that an entry is topped up "at or below" the
   threshold, which is what the network actually does, instead of "below".
2. They read that going past the network's maximum lifetime is capped for normal
   data, not a crash, and why Evergreen still caps it first.
3. Instead of "about 30 days", which was true years ago, the docs tell them how to
   read the current limit from the network, with today's mainnet figures (about
   180 days maximum, about 120 days minimum for new data) shown as a dated
   example.
4. The example contracts explain why topping up to 30 days at a time is still a
   good choice: it spreads the cost out and does not prepay for data nobody uses.

## Important Limitation

One claim, about how a read with a top-up is recorded in a transaction, cannot be
settled by reading code. It needs a real test transaction on Stellar's test
network. Until that is run and pasted into the pull request, that one sentence
stays as it is.

## What Changes

- Wording in the library's reference docs, the READMEs, the book, and the
  example comments.
- New tests that check the exact boundary where a top-up happens.
- A short release note.

## What Does Not Change

- No code behaviour, no function names, no stored data.
- Contracts already using Evergreen need no change.

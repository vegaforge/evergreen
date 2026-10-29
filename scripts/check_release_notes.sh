#!/usr/bin/env bash
# CI gate: if shipped code changed, the same diff must add a release note.
#
# Warns locally. CI sets STRICT_RELEASE_NOTES=true, which turns the warning
# into a failure.
set -euo pipefail

GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RED='\033[0;31m'; NC='\033[0m'
BASE="${RELEASE_NOTES_BASE:-origin/main}"

list() {
  while IFS= read -r line; do
    echo "  - $line"
  done <<<"$1"
}

# crates/ and bindings/ are shipped code. examples/ are contracts users copy,
# so changes there need a note too. The root Cargo.toml carries the release
# profile and shared dependency versions. Cargo.lock is left out on purpose so
# dependency bumps do not need a note.
APP_PATHS='^(crates|bindings|examples)/|^Cargo\.toml$'

# Make sure the base ref exists. A failed fetch is reported, not hidden: the
# diff below fails loudly if the ref is still missing.
if ! git fetch origin "${BASE#origin/}" --quiet; then
  echo -e "${YELLOW}Could not fetch ${BASE}; using the local ref.${NC}"
fi

CHANGED=$(git diff --name-only "$BASE...HEAD")
CHANGED_APP_FILES=$(echo "$CHANGED" | grep -E "$APP_PATHS" || true)
RELEASE_NOTES=$(echo "$CHANGED" | grep -E '^docs/release-notes/.+\.md$' | grep -v '/TEMPLATE\.md$' || true)

if [ -z "$CHANGED_APP_FILES" ]; then
  echo -e "${GREEN}No shipped code changed. Release note not required.${NC}"
  exit 0
fi

if [ -z "$RELEASE_NOTES" ]; then
  echo -e "${YELLOW}Shipped code changed but no release note was added under docs/release-notes/:${NC}"
  list "$CHANGED_APP_FILES"
  echo -e "${YELLOW}Add docs/release-notes/<YYYY-MM-DD>-<topic>.md (copy docs/release-notes/TEMPLATE.md).${NC}"
  if [ "${STRICT_RELEASE_NOTES:-false}" = "true" ]; then
    echo -e "${RED}Release note check failed in strict mode.${NC}"
    exit 1
  fi
  exit 0
fi

echo -e "${GREEN}Release note found:${NC}"
list "$RELEASE_NOTES"

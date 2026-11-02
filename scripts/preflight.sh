#!/usr/bin/env bash
# One entry point for "is this change clean". CI runs the same gates.
#
#   ./scripts/preflight.sh              all five gates
#   ./scripts/preflight.sh --fast       gates 1 to 4; the pre-push hook uses this
#   ./scripts/preflight.sh --build-only gate 5 alone; CI's build job uses this
set -euo pipefail

# Git hooks run with a minimal PATH. Make cargo, rustup and node visible.
if [ -d "$HOME/.cargo/bin" ]; then
  PATH="$HOME/.cargo/bin:$PATH"
fi
if [ -d "${ASDF_DATA_DIR:-$HOME/.asdf}/shims" ]; then
  PATH="${ASDF_DATA_DIR:-$HOME/.asdf}/shims:$PATH"
fi
export PATH
# Load a Node version manager only when npm is not already reachable. nvm
# refuses to load under npm or npx (npm_config_prefix is set there), and a
# working npm on PATH is all the gates need.
if ! command -v npm >/dev/null 2>&1; then
  if command -v mise >/dev/null 2>&1; then
    eval "$(mise activate bash --shims)"
  else
    export NVM_DIR="${NVM_DIR:-$HOME/.nvm}"
    if [ -s "$NVM_DIR/nvm.sh" ]; then
      # shellcheck disable=SC1091
      . "$NVM_DIR/nvm.sh"
    fi
  fi
fi

BLUE='\033[0;34m'; GREEN='\033[0;32m'; RED='\033[0;31m'; NC='\033[0m'

fail() {
  echo -e "${RED}$1${NC}" >&2
  exit 1
}

RUN_CHECKS="true"
RUN_BUILD="true"
case "${1:-}" in
  "") ;;
  --fast) RUN_BUILD="false" ;;
  --build-only) RUN_CHECKS="false" ;;
  *) fail "Unknown argument: $1. Use --fast, --build-only, or no argument." ;;
esac

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

JS_DIR="bindings/evergreen-js"
EXAMPLES=(token-with-policy rarely-written-state migration-demo)
# Mainnet contract_max_size_bytes, read from `stellar network settings` on
# 2026-10-29. It is a network parameter; update it if the network raises it.
WASM_MAX_BYTES=131072

# Fail before any gate runs if a tool is missing, so the error is about the
# tool and not a confusing compiler message halfway through.
command -v cargo >/dev/null 2>&1 || fail "cargo not found. Install Rust with rustup: https://rustup.rs"
command -v rustup >/dev/null 2>&1 || fail "rustup not found. Install Rust with rustup: https://rustup.rs"
command -v npm >/dev/null 2>&1 || fail "npm not found. Install Node.js (version in $JS_DIR/.nvmrc)."
if ! rustup target list --installed | grep -qx wasm32v1-none; then
  fail "Rust target wasm32v1-none is not installed for $(rustc --version). Run: rustup target add wasm32v1-none"
fi
if [ ! -d "$JS_DIR/node_modules" ]; then
  fail "$JS_DIR/node_modules is missing. Run: npm ci --prefix $JS_DIR"
fi

echo -e "${BLUE}=== Evergreen preflight ($(rustc --version)) ===${NC}"

if [ "$RUN_CHECKS" = "true" ]; then
  echo -e "\n${BLUE}[1/5] Format check${NC}"
  cargo fmt --all -- --check

  echo -e "\n${BLUE}[2/5] Lint (clippy, warnings denied; knip)${NC}"
  cargo clippy --workspace --all-targets --locked -- -D warnings
  # Knip fails on JS dependencies nothing imports and imports nothing declares.
  npm --prefix "$JS_DIR" run knip

  echo -e "\n${BLUE}[3/5] Typecheck (cargo check, rustdoc, tsc)${NC}"
  cargo check --workspace --all-targets --locked
  RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked
  npm --prefix "$JS_DIR" run typecheck

  echo -e "\n${BLUE}[4/5] Tests${NC}"
  cargo test --workspace --locked
  # The PR security checks have their own tests (standard library only).
  PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s scripts/security -q
else
  echo -e "\n${BLUE}[1-4/5] Skipped in --build-only mode${NC}"
fi

if [ "$RUN_BUILD" = "false" ]; then
  echo -e "\n${BLUE}[5/5] Build (skipped in --fast mode)${NC}"
else
  echo -e "\n${BLUE}[5/5] Build (wasm32v1-none examples, size check, JS binding)${NC}"
  for pkg in "${EXAMPLES[@]}"; do
    cargo build -p "$pkg" --target wasm32v1-none --release --locked
  done
  for pkg in "${EXAMPLES[@]}"; do
    wasm="target/wasm32v1-none/release/${pkg//-/_}.wasm"
    [ -f "$wasm" ] || fail "Expected build output $wasm was not produced."
    size=$(wc -c <"$wasm" | tr -d ' ')
    if [ "$size" -gt "$WASM_MAX_BYTES" ]; then
      fail "$wasm is $size bytes, over the $WASM_MAX_BYTES byte network limit."
    fi
    echo "$wasm: $size bytes (limit $WASM_MAX_BYTES)"
  done
  npm --prefix "$JS_DIR" run build
fi

if [ "$RUN_CHECKS" = "true" ] && [ "$RUN_BUILD" = "true" ]; then
  echo -e "\n${GREEN}All five preflight gates passed.${NC}"
elif [ "$RUN_CHECKS" = "true" ]; then
  echo -e "\n${GREEN}Gates 1 to 4 passed. Gate 5 (build) was skipped.${NC}"
else
  echo -e "\n${GREEN}Gate 5 (build) passed. Gates 1 to 4 were skipped.${NC}"
fi

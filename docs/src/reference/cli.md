# CLI

The `evergreen` command is built from the `evergreen-cli` crate. Its command
surface is in place, but the commands are not implemented yet: each one prints
the issue that tracks it and exits. This page lists the planned commands so you
can see where the CLI is going.

## Build it

From a clone of the repository:

```bash
cargo run -p evergreen-cli -- --help
```

## Commands

Commands that act on a deployed contract take the same two options:

| Option | Meaning |
|---|---|
| `--rpc <URL>` | Stellar RPC endpoint |
| `--contract <ID>` | Contract id |

| Command | What it will do | Tracked in |
|---|---|---|
| `evergreen inspect --rpc … --contract …` | List entries with TTL, time to archival and projected cost, as a table or JSON | [#5](https://github.com/vegaforge/evergreen/issues/5) |
| `evergreen extend --rpc … --contract …` | Extend entries approaching archival, within a rent budget | [#3](https://github.com/vegaforge/evergreen/issues/3) |
| `evergreen restore --rpc … --contract …` | Restore archived entries | [#4](https://github.com/vegaforge/evergreen/issues/4) |
| `evergreen forecast --rpc … --contract …` | Project rent cost over a horizon from simulation | [#6](https://github.com/vegaforge/evergreen/issues/6) |
| `evergreen upgrade --wasm-hash <HEX>` | Upgrade a contract's Wasm | No issue yet; see [Phase 3](../project/roadmap.md#phase-3-the-cli) |
| `evergreen migrate --to-version <N>` | Run a storage migration, dry-run first | [#13](https://github.com/vegaforge/evergreen/issues/13) |
| `evergreen lint <PATH>` | Flag durable data in temporary storage, with exit codes and JSON output | [#11](https://github.com/vegaforge/evergreen/issues/11) (not in the CLI yet) |

Options and output formats may change as each command is built. The issue for
each command is the source of truth.

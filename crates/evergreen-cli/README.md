# evergreen-cli

The command line tool that wraps Evergreen for use outside a long-running
monitor. It installs as the `evergreen` binary.

## Commands

| Command | What it does | Backed by |
| ------- | ------------ | --------- |
| `inspect` | List entries with TTL, time to archival, and projected cost | `evergreen-monitor` |
| `extend` | Extend entries approaching archival | `evergreen-monitor` |
| `restore` | Restore archived entries | `evergreen-monitor` |
| `forecast` | Project rent cost over a horizon | cost forecasting |
| `upgrade` | Upgrade a contract's Wasm | `evergreen-migrate` |
| `migrate` | Run a declarative storage migration, dry-run first | `evergreen-migrate` |

## Status

Stub. The command surface is wired up with clap and is real; the handlers report
what each command will do and which crate will do it. Inspection and forecasting
are Phase 3, upgrade and migration are Phase 4. See the
[roadmap](../../docs/src/roadmap.md) for the issue behind each command.

## Try the surface

```bash
cargo run -p evergreen-cli -- --help
cargo run -p evergreen-cli -- inspect --rpc <url> --contract <id>
```

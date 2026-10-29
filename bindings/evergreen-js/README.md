# @evergreen/js

Thin TypeScript bindings for Evergreen, so teams running their operational
tooling in Node can drive the monitor and CLI without rewriting their ops stack
in Rust.

This is a convenience surface, not a parallel implementation. The on-chain
library and the core tooling stay in Rust. These bindings wrap the Rust core's
interface, or use js-stellar-sdk directly for the read and submit paths.

## Status

Stub. The interface in `src/index.ts` mirrors the Rust `evergreen-monitor` crate.
Every method throws `NotImplementedError` until the bindings are built out. See the
[roadmap](../../docs/src/roadmap.md#typescript-binding) and
[#14](https://github.com/vegaforge/evergreen/issues/14).

## Develop

```bash
npm install
npm run typecheck
```

# Picocodex contributor guide

Picocodex is a library embedded by Satyr. Keep the runtime small and
host-oriented.

## Ownership

Picocodex owns model conversation state, turn lifecycle, typed model events,
steering, cancellation, snapshots, forks and model-visible tool definitions.

The embedding application owns credentials, tenants, authorization, durable
state, artifact storage, tool effects, executor placement, quotas, billing and
deployment.

## Constraints

- Preserve the OpenAI Codex conversation and tool-call semantics.
- Keep direct tools, Code Mode and hybrid mode behind one typed host boundary.
- Keep raw credentials out of Wasm configuration and snapshots.
- Use versioned opaque snapshots as the persistence boundary.
- Avoid compatibility aliases when changing active APIs.
- Keep upstream names inside provenance files only.
- Do not add VM, UI, payment, deployment or application-server products.

## Verification

Run the narrowest relevant checks first:

```sh
cargo +1.97.0 check --workspace
cargo +1.97.0 test --workspace
cargo +1.97.0 check --target wasm32-unknown-unknown -p picocodex-wasm
RUSTUP_TOOLCHAIN=1.97.0 ./scripts/build-js-package.sh
npm test --prefix js/bindings
```

# Picocodex

Picocodex is Satyr's small, embeddable ChatGPT/Codex harness library. It owns
the model conversation and turn lifecycle while an embedding host owns durable
state, credentials, authorization, tools, compute placement and billing.

The primary target is Rust compiled to `wasm32-unknown-unknown` and embedded in
a Cloudflare Worker or Durable Object. Native Rust and Node bindings remain
available for focused development and differential testing.

## Capabilities

- typed Responses lifecycle and events;
- steering, cancellation, compaction and safe forks;
- versioned opaque session snapshots;
- host-owned Responses WebSocket transport;
- host-managed authentication without a bearer credential in Wasm;
- direct typed tools;
- Code Mode tools; and
- hybrid direct-tool and Code Mode exposure.

Picocodex contains no VM manager, deployment platform, durable database,
payment system, user interface or tenant control plane.

## Build

```sh
cargo +1.97.0 check --workspace
cargo +1.97.0 test --workspace
cargo +1.97.0 check --target wasm32-unknown-unknown -p picocodex-wasm
RUSTUP_TOOLCHAIN=1.97.0 ./scripts/build-js-package.sh
npm test --prefix js/bindings
```

## Repository layout

- `crates/picocodex-agent`: owned agent and turn lifecycle.
- `crates/picocodex-oai-api`: Responses protocol and transport.
- `crates/picocodex-tools`: tool contracts and Code Mode runtimes.
- `crates/picocodex`: public Rust facade.
- `js/bindings`: Node and Worker/browser Wasm bindings.

See [UPSTREAM.md](UPSTREAM.md) and [SOURCE_IMPORT.json](SOURCE_IMPORT.json) for
the independently preserved upstream provenance.

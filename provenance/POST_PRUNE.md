# Post-prune verification

Picocodex was verified after source pruning, the forward-only identity rename,
and addition of the Satyr host contracts.

Toolchain:

```text
rustc 1.97.0 (2d8144b78 2026-07-07)
wasm32-unknown-unknown
Node.js 26.4.0
```

Results:

- `cargo check --workspace --all-targets`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Rust unit and integration tests: 386 passed, 2 environment-dependent tests
  ignored.
- Rust doctests: 24 passed, 1 macro example ignored.
- `picocodex-wasm` compiled for `wasm32-unknown-unknown`.
- JavaScript and Wasm lifecycle, host-transport, direct-tool, Hybrid, and
  package tests: 36 passed.
- Binding performance checks: 3 passed.
- TypeScript type checks and package completeness checks: passed.

The preserved reachability reports are stored beside the upstream bundle:

```text
2e1b3650b6d7a02b77f41f61957d11268ead00a39b0d10b54adfaae3690a14d4  picocodex-cargo-metadata.json
decd7630aad5317642548ea8cbfe1be9c6e606235f6a27b5e68307b74a9461cd  picocodex-native-tree.txt
cd8098e7fd7313e483c093969d25634d903cce99b2e2ff3d2658b3481871a426  picocodex-wasm-tree.txt
```

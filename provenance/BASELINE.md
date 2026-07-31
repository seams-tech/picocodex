# Imported baseline

The baseline was recorded after applying the selected PR #75 commits and before
source pruning.

Toolchain:

```text
rustc 1.97.0 (2d8144b78 2026-07-07)
wasm32-unknown-unknown
Node.js 26.4.0
```

Results:

- Retained Rust packages: 404 unit and integration tests passed.
- Environment-dependent Rust tests: 2 ignored by upstream policy.
- Rust doctests: all passed.
- `nanocodex-wasm` compiled for `wasm32-unknown-unknown`.
- JavaScript and Wasm lifecycle, host-transport and direct-tool tests: 37
  passed.
- Binding performance checks: 3 passed.
- TypeScript type checks: passed.
- JavaScript package completeness check: passed.

Commands:

```sh
cargo +1.97.0 test \
  -p nanocodex \
  -p nanocodex-agent \
  -p nanocodex-oai-api \
  -p nanocodex-tools \
  -p nanocodex-wasm
cargo +1.97.0 check --target wasm32-unknown-unknown -p nanocodex-wasm
RUSTUP_TOOLCHAIN=1.97.0 ./scripts/build-js-package.sh
npm test --prefix js/bindings
```

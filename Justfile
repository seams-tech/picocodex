set shell := ["bash", "-euo", "pipefail", "-c"]

check:
    cargo +1.97.0 check --workspace

test:
    cargo +1.97.0 test --workspace

wasm:
    RUSTUP_TOOLCHAIN=1.97.0 ./scripts/build-js-package.sh

test-js: wasm
    npm test --prefix js/bindings

verify:
    cargo +1.97.0 fmt --all -- --check
    cargo +1.97.0 clippy --workspace --all-targets -- -D warnings
    cargo +1.97.0 test --workspace
    cargo +1.97.0 check --target wasm32-unknown-unknown -p picocodex-wasm
    RUSTUP_TOOLCHAIN=1.97.0 ./scripts/build-js-package.sh
    npm test --prefix js/bindings

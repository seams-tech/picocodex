# JavaScript libraries

- [`bindings`](bindings) publishes `picocodex`: runtime-specific `Agent`
  namespaces, domain-grouped `Actions`, decorators, and Node/browser WASM hosts.
- [`react`](react) publishes `picocodex-react`: the external store, provider,
  and hooks for a browser Worker owned by the embedding application.
- [`tui`](tui) publishes `picocodex-tui`: framework-independent transcript
  state and event reduction.
- [`tui-react`](tui-react) publishes `picocodex-tui-react`: the accessible,
  virtualized, unstyled-by-default terminal renderer and optional theme.

Applications consume package entrypoints. Generated `wasm-bindgen` output
stays private to `picocodex` and is produced by `just build-wasm`.

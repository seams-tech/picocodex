# Upstream provenance

Picocodex was imported from
[`gakonst/nanocodex`](https://github.com/gakonst/nanocodex) on 2026-07-31.

The imported base was:

```text
3d4548b0c30f2ffbda686b16d7221ffd4faacbca
```

The reusable Wasm host-transport and direct-tool work from upstream PR #75 was
then applied in this order:

```text
66c9f38  feat: expose reusable WASM host transport
3f522fc  fix(wasm): harden host socket upgrades
e52dc62  feat(wasm): support CSP-safe direct host tools
8516800  fix(agent): dispatch unnamespaced hosted tools
```

PR #76 was used as architectural evidence. Its standalone Cloudflare
application, Durable Object, browser client, subscription storage and
deployment surface were not imported.

The original MIT and Apache-2.0 license files are retained at the repository
root. Active source is being renamed and reduced to the Satyr-owned Picocodex
library. Historical `nanocodex` names remain only in this provenance boundary
after the independent repository is initialized.

## Preserved source

The complete upstream repository bundle and source archive are stored outside
the working repository under:

```text
/Users/pta/Dev/rust/picocodex-upstream-preservation
```

Their SHA-256 checksums are:

```text
5be7b8c63445a06865b2f88f7158ca74fb95e6413f39726e8cd96d6fe16f8b69  nanocodex-3d4548b-all.bundle
72b96174edca7f0fa73fdc4038fb2c9b54021a92b0e8e574ce2513a328972305  nanocodex-3d4548b-source.tar.gz
d1b0299539e4ea2b06ecb4c6580fead2f83399ed71b31ae82a7a79dd98c9995a  import-to-picocodex.patch
```

The transformation patch was generated against the imported
upstream-plus-PR-75 tree before this checksum was recorded in the provenance
metadata.

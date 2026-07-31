# Picocodex Implementation Plan

Status: in progress
Date: 2026-07-31
Depends on:
[Product Direction](./file-native-commerce/product-direction.md),
[Plan 12](./plan-12-policy-governed-harness-secrets-vault.md),
[Plan 14](./plan-14-actor-runtime-architecture.md), and
[Metered Code Mode](./plan-8-metered-code-mode.md)

## Decision

Create an independent Satyr-owned repository named `picocodex` from upstream
Nanocodex commit `3d4548b0c30f2ffbda686b16d7221ffd4faacbca`, then apply the
four reusable host-transport and direct-tool commits from upstream PR
[#75](https://github.com/gakonst/nanocodex/pull/75):

- `66c9f38` — reusable Wasm host transport;
- `3f522fc` — hardened host socket upgrades;
- `e52dc62` — CSP-safe direct host tools; and
- `8516800` — unnamespaced hosted-tool dispatch.

Preserve a complete upstream Git bundle and source archive, apply those commits,
remove the clone's `.git` directory, prune the source tree, and initialize a new
repository at `seams-tech/picocodex`. The new repository will have no GitHub
fork relationship or inherited Git object dependency on `gakonst/nanocodex`.

The resulting import is based on the latest inspected Nanocodex core
architecture and contains:

- runtime-owned Wasm instantiation;
- host-managed Responses WebSocket transport;
- CSP-safe direct hosted tools;
- typed tool input and output;
- socket-upgrade hardening; and
- focused Rust, JavaScript, and Wasm tests.

Prune Picocodex to the dependency closure Satyr actually builds. Remove KVM,
retained VM workspaces, standalone applications, experimental agents, and
unrelated language and UI consumers.

Use upstream PR [#76](https://github.com/gakonst/nanocodex/pull/76) as design
evidence. Implement its useful subscription-authentication and bounded-egress
behaviour inside Satyr's credential vault and Model Gateway. Do not import its
standalone Cloudflare Durable Object agent, browser client, REPL, or session
protocol.

Selectively port the retained library changes from upstream PR
[#80](https://github.com/gakonst/nanocodex/pull/80). Keep its model identity,
thread-lifetime model binding, model-aware snapshots, forks, prompt caches,
usage, and telemetry. Exclude its CLI, TUI, Python, Harbor, embedded pricing,
and foreign-rollout compatibility paths. Unknown or mismatched models fail at
the boundary instead of falling back to Sol.

Do not import upstream PR
[#74](https://github.com/gakonst/nanocodex/pull/74). Its Rivet Actor, local
server, authentication, REPL, browser, and JSONL session implementation
duplicates Satyr-owned responsibilities. Retain its commit only as
architectural evidence when documenting provenance.

The target repository is `seams-tech/picocodex`. Satyr will consume an exact
commit rather than a floating branch or package version.

The first implementation step installs a narrow Actor-owned adapter around the
current pinned Nanocodex runtime. Picocodex is the first new backend placed
behind that proven contract. Satyr then completes its direct-tool path and
proves one Actor-embedded commerce canary without allocating a Container. That
milestone establishes Picocodex as the reference implementation of the neutral
adapter.

Only after that milestone may a bounded Pi challenger probe begin. The first
probe is limited to `@earendil-works/pi-agent-core`,
`@earendil-works/pi-ai`, the Anthropic provider, one direct Satyr tool, and
workerd. It has no persistence, checkpoint restore, production selection,
customer setting, or deployment. A successful first probe unlocks checkpoint
reconstruction and matched Slack and Cue evaluation fixtures.

Eval quality, latency, operational complexity, and cost then determine whether
Pi becomes the Claude backend, the default commerce backend, or an unsupported
experiment. Picocodex remains Satyr's first proven backend throughout that
evaluation.

Reverse this order only when Claude or customer-selectable model providers
become requirements for Satyr's initial product launch. Provider breadth would
then be a launch constraint and the Pi feasibility work would take immediate
priority.

## Why an independent repository now

Plan 14 originally prohibited a speculative fork. The missing generic direct
tool seam is now demonstrated:

- the pinned revision `a4ad251…` exposes caller tools only through Code Mode;
- an ordinary typed tool call therefore allocates a Dynamic Worker;
- Dynamic Workers add invocation, startup CPU, execution CPU, and potentially
  unique-worker charges; and
- PR #75 supplies the reusable host seam, while its exclusive `code | direct`
  selection still needs a Satyr-specific hybrid mode.

The current pin is 146 upstream commits behind PR #75's base. A literal
backport conflicts with Nanocodex's intervening crate and Wasm-host refactor.
Maintaining that older architecture would require manually reconstructing new
host behaviour and separately auditing skipped protocol fixes.

Current upstream also carries a much broader product surface: KVM, retained
VMs, browser and voice agents, CLI and TUI products, Python and Node consumers,
payment experiments, and multiple hosting examples. Most unused source does
not enter Satyr's Wasm artifact, but it expands the audit surface and makes
upgrades noisy.

Picocodex adopts the modern core and removes unrelated workspace members. It
will not routinely merge upstream. An independent repository also remains
available if the original Nanocodex repository, PR branches, tags, or release
artifacts disappear.

Removing upstream Git metadata does not remove attribution. The first
Picocodex commit must contain:

- the complete upstream MIT and Apache-2.0 license files;
- `UPSTREAM.md` with the source repository, commit, PRs, import date, and
  licenses;
- `SOURCE_IMPORT.json` with machine-readable source and pruning metadata;
- the original source archive checksum and Satyr-controlled archive location;
  and
- a patch showing the difference between the imported upstream-plus-PR-75 tree
  and the initial pruned Picocodex tree.

Establish a pre-prune build and behavioral baseline after applying the selected
PR #75 commits. Preserve its command output alongside the untouched source
archive and Git bundle so pruning failures can be distinguished from inherited
upstream failures.

## Current Satyr architecture assessment

Satyr is harness-neutral at the durable-authority layer:

- `agent-session-harness-authority.ts` owns generic turn leases, accepted event
  ordering, idempotency, delivery segments, and checkpoint commits;
- `agent-session-harness-checkpoint.ts` stores opaque checkpoint bytes with
  semantic bindings;
- Actor lifecycle, workspace receipts, budgets, surfaces, and executor
  placement use harness-independent contracts; and
- sandbox harnesses already normalize into a shared protocol.

The embedded Actor operating path is currently Nanocodex-specific:

- `AgentSessionActor` imports the Nanocodex runtime, model host, event bridge,
  Code Mode host, input binder, and their error unions directly;
- Actor RPC methods and active-turn state use Nanocodex names and types;
- construction, restore, turn execution, event translation, tool exposure,
  model transport, and cleanup are assembled inside `AgentSessionActor`;
- checkpoint records bind build and format without an explicit embedded
  backend identity; and
- the existing `HarnessSpec` describes sandbox process placement and must not
  double as the embedded Actor backend contract.

Therefore, the current system has neutral durable primitives and a
backend-coupled embedded execution path. Picocodex and Pi cannot be cleanly
supported side by side until the Actor depends only on a neutral embedded
harness adapter.

Installing that boundary is the first Satyr implementation phase. It wraps the
current pinned Nanocodex runtime without changing behavior, proves the boundary
against the existing tests, and then allows Picocodex to replace the backend
behind the same contract.

## Architectural boundary

Satyr owns:

- tenants, users, policy, credentials, quotas, and billing;
- durable Actor state and accepted event history;
- surface attachment, replay, and delivery;
- workspace revisions and artifact references;
- tool authorization and provider effect receipts;
- direct-tool versus executor placement;
- Dynamic Worker and Container allocation; and
- production rollout, observability, and incident response.

Picocodex owns:

- the model conversation and turn lifecycle;
- typed model events;
- steering and cancellation;
- session snapshots and safe forks;
- Responses transport contracts;
- model-visible tool definitions; and
- host callbacks for direct tools and Code Mode.

The durable rule remains:

> The Actor's accepted event history and referenced artifacts are the durable
> agent. Harnesses and executors are replaceable processes that animate it.

No credential, tenant policy, provider client, canonical file, surface state,
or execution allocation becomes Picocodex-owned.

## Harness-neutral Actor adapter

`AgentSessionActor` must depend on a small harness-neutral lifecycle contract.
Backend-specific Wasm or JavaScript bindings, host globals, model transports,
snapshot parsing, event translation, and error translation remain behind one
backend adapter.

The domain boundary has one supported backend initially:

```ts
type EmbeddedHarnessBackendId = "nanocodex";

type EmbeddedHarnessSource =
  | {
      readonly kind: "fresh";
    }
  | {
      readonly kind: "restore";
      readonly checkpoint: ArrayBuffer;
    };

type EmbeddedHarnessManifest = {
  readonly backendId: EmbeddedHarnessBackendId;
  readonly harnessBuild: HarnessBuildId;
  readonly checkpointFormat: HarnessCheckpointFormatVersion;
};

type EmbeddedHarnessConstructionInput = {
  readonly source: EmbeddedHarnessSource;
  readonly session: SessionIdentity;
  readonly modelRouteLease: ModelRouteLease;
  readonly instructions: string;
  readonly toolHost: HarnessToolHost;
  readonly eventSink: CanonicalHarnessEventSink;
};

interface EmbeddedHarnessAdapter {
  readonly manifest: EmbeddedHarnessManifest;
  construct(
    input: EmbeddedHarnessConstructionInput
  ): Promise<Result<EmbeddedHarnessRuntime, EmbeddedHarnessConstructionError>>;
}

interface EmbeddedHarnessRuntime {
  startTurn(
    input: readonly UserInput[]
  ): Result<EmbeddedHarnessActiveTurn, EmbeddedHarnessTurnError>;
  close(): void;
}

interface EmbeddedHarnessActiveTurn {
  steer(input: readonly UserInput[]): Promise<Result<null, EmbeddedHarnessSteerError>>;
  cancel(): Promise<Result<null, EmbeddedHarnessCancelError>>;
  complete(): Promise<Result<EmbeddedHarnessTurnResult, EmbeddedHarnessTurnError>>;
}
```

The exact names may follow existing repository conventions, but the boundary
must preserve these properties:

- fresh and restored construction are distinct valid states;
- the Actor passes a credential-free model-route lease rather than raw
  provider authentication;
- model transport details remain private to the backend adapter;
- tools enter through Satyr's authorized capability host;
- backend events become canonical harness events before Actor acceptance;
- turn completion returns an opaque checkpoint;
- steering and cancellation remain separate operations;
- close deterministically releases runtime, socket, and host registrations;
  and
- recoverable failures use exhaustive `Result` unions.

Use one exhaustive adapter selector with exactly one production branch during
the Picocodex work. NF1 initially wraps the current `nanocodex` backend. NF4
replaces that branch and identity with `picocodex`; it does not retain a
parallel legacy runtime. Do not add a plugin registry, dynamic module
discovery, optional callbacks, generic provider configuration, or placeholder
Pi branch.

### Checkpoint and lineage binding

The Actor treats every backend checkpoint as opaque bytes. Canonical
checkpoint metadata must bind:

- backend ID;
- harness build;
- checkpoint format;
- session and runtime generation;
- accepted event cursor;
- model-route digest;
- instruction digest;
- tool-catalog digest;
- workspace revision;
- policy revision; and
- checkpoint content digest.

Restore succeeds only when the selected adapter manifest matches the backend
ID, build, and checkpoint format. The adapter alone parses the plaintext
checkpoint after Satyr has authenticated, unsealed, and validated its semantic
binding.

A future backend change may keep the same durable `AgentSessionActor` and
attached surfaces, but it must start a new harness lineage. Satyr would first
commit accepted history and referenced artifacts, materialize a
provider-neutral conversation projection, accept a route-change event, and
construct a fresh checkpoint with the new adapter. A checkpoint created by one
backend never enters another backend.

### Canonical events and surfaces

Slack, the portal, future messaging surfaces, Cue scheduling, billing, and
execution placement consume only accepted Actor events. They must never branch
on Picocodex event payloads or Wasm types.

Keep backend-specific translation inside the adapter boundary. The canonical
contract covers:

- user input acceptance;
- assistant deltas and completed messages;
- tool-call start, progress, result, and failure;
- usage and model attribution;
- steering acceptance;
- cancellation;
- terminal completion or failure; and
- checkpoint availability.

This boundary permits a later Pi adapter while keeping Actor durability,
surface replay, tool authorization, accounting, and executor routing
unchanged.

## Recommended backend sequencing

The implementation order is:

1. Install the neutral embedded harness contract around the current pinned
   Nanocodex runtime and prove no behavior change.
2. Complete the Picocodex adapter and direct-tool path behind that contract.
3. Produce one successful Actor-embedded commerce canary without a Container.
4. Establish Picocodex as the first reference implementation of the neutral
   adapter.
5. Run a bounded Pi feasibility probe using:
   - `@earendil-works/pi-agent-core`;
   - `@earendil-works/pi-ai`;
   - the Anthropic provider only;
   - one direct Satyr tool;
   - workerd; and
   - no persistence.
6. Add checkpoint reconstruction and run the same Slack and Cue eval fixtures
   only when the first Pi probe passes.
7. Use eval quality, latency, operational complexity, and cost to classify Pi
   as:
   - the Claude backend;
   - the default commerce backend; or
   - an unsupported experiment.

The governing distinction is:

> **Picocodex first as the proven reference backend. Pi next as a challenger
> for the long-term default.**

The Pi probe excludes Pi's coding-agent CLI, TUI, local session authority,
bundled filesystem and shell tools, extensions, and product UI. Satyr supplies
the Actor lifecycle, canonical events, direct tool authorization, model route,
budgets, and execution placement.

The initial Picocodex adapter selector remains exhaustive with one production
branch. The first Pi probe uses fresh construction through a test-only
conformance entry point and cannot be selected by a tenant or production
session. A successful first probe permits the later persistent Pi adapter work;
it does not authorize rollout.

Reverse the order only when Claude or customer-selectable providers are
required for the initial Satyr launch. In that case, provider breadth becomes
a product requirement and Pi receives immediate priority.

## Retained Picocodex contents

Determine the final closure with `cargo metadata` for native and
`wasm32-unknown-unknown` targets before deleting files. The expected retained
set is:

- `crates/picocodex`;
- `crates/picocodex-agent`;
- `crates/picocodex-oai-api`;
- `crates/picocodex-tools`;
- `crates/picocodex-tools/macros` when required by the native build;
- `js/bindings` and the Wasm build inputs used by Satyr;
- focused lifecycle, transport, direct-tool, snapshot, and Wasm tests;
- the minimum build scripts and CI configuration for those targets;
- upstream license files;
- explicit source-import provenance and the import-to-pruned-tree patch; and
- a Picocodex README describing scope, provenance, and update policy.

Retain optional observability code only when Satyr's actual dependency graph
uses it. Keep test fixtures that protect the selected model and host contracts.

## Removed Picocodex contents

Remove the following from the Cargo workspace, package manifests, CI, and
source tree:

- `nanocodex-vm`, libkrun, KVM, OCI-to-rootfs, and retained VM tools;
- browser-agent and voice-agent experiments;
- Nanocodex CLI and TUI products;
- Python bindings;
- standalone web applications;
- payment and marketplace experiments;
- the PR #76 Cloudflare Worker example;
- the PR #76 Durable Object session and authentication classes;
- detachable Nanocodex REPL and browser clients;
- unrelated benchmarks, examples, deployment files, and release automation;
- generic self-update machinery; and
- dependencies reachable only from removed targets.

Deletion is complete when native and Wasm `cargo tree` output contains no KVM,
VM workspace, browser-agent, voice, TUI, Python, payment, or Cloudflare-example
dependency.

## Forward-only identity rename

Rename the complete retained source tree before initializing the new Git
repository. Picocodex's first commit must use its final identity throughout:

- Rust crates and dependency keys;
- Rust modules, public types, traits, functions, constants, tests, and feature
  names;
- JavaScript package names, exports, imports, types, tests, and generated
  bindings;
- Wasm modules, filenames, exports, and loader symbols;
- `globalThis.picocodexHost` and every host callback reference;
- environment variables and build configuration;
- harness IDs, build IDs, checkpoint formats, user-agent strings, trace
  targets, error codes, and event component IDs;
- artifact, cache, temporary-directory, and release names; and
- current documentation and examples retained in the repository.

Use `picocodex`, `Picocodex`, and `PICOCODEX` according to the target language's
naming convention. Do not add package aliases, crate aliases, legacy
environment variables, dual host globals, or checkpoint converters.

The original `nanocodex` name remains only where accurate attribution requires
it:

- `UPSTREAM.md`;
- `SOURCE_IMPORT.json`;
- upstream license or copyright text;
- the immutable source archive metadata; and
- the import-to-pruned-tree provenance patch.

Keep those files under an explicit `provenance/` boundary where practical. A
repository check must fail when a case-insensitive `nanocodex` match appears
outside the provenance allowlist.

Satyr receives the same forward-only rename during NF4. Current source files,
symbols, harness IDs, fixtures, generated artifacts, tests, trace names, and
operator-facing runtime labels become Picocodex. Historical evidence and
append-only persistence records retain the names that were true when written.

The production Picocodex cohort starts with a new harness build and checkpoint
lineage. Close any active Nanocodex lineage under its existing immutable build
before cutover. Do not resume its checkpoint under Picocodex.

## Required Picocodex extension: hybrid tool exposure

PR #75 exposes either direct tools or Code Mode for a host. Satyr needs a stable
catalog that can expose both in one session. Add an explicit Rust domain type:

```rust
enum HostedToolExposure {
    Code,
    Direct {
        tools: DirectToolSet,
    },
    Hybrid {
        direct_tools: DirectToolSet,
    },
}
```

`Hybrid` exposes:

- selected Satyr tools as normal typed function tools; and
- the Code Mode `exec` tool for orchestration that genuinely needs generated
  programs.

The host advertises a validated set of direct tool names. A model cannot turn
a Code Mode-only capability into a direct capability by changing its call
shape. Unknown and unadvertised names fail closed.

Tool exposure is stable for the lifetime of a checkpoint lineage. The
`AgentSessionActor` binds its digest to:

- harness build;
- instructions;
- model route;
- tool definitions;
- direct-tool set;
- workspace revision; and
- policy revision.

A catalog change starts a new compatible lineage or rejects restore at the
persistence boundary. Core execution will never accept a mismatched snapshot.

## Execution placement after Picocodex

Use the cheapest sufficient route:

| Work | Route |
| --- | --- |
| Read product, order, campaign, or workspace metadata | Direct typed tool |
| Call an authorized commerce or surface integration | Direct typed tool |
| Validate and accept a small semantic changeset | Direct typed tool |
| Combine, filter, or reduce many tool results | Dynamic Worker Code Mode |
| Run generated transformations | Dynamic Worker Code Mode |
| Use Linux binaries, browsers, media tools, or a materialized filesystem | Short-lived Container |

Direct tools execute through Satyr's existing authenticated gateway and Actor
authority. They receive validated tenant, session, tool, policy, and budget
state. Raw provider credentials remain inside the credential broker.

Code Mode retains network denial and receives typed capabilities. It cannot
read Actor bindings, credential storage, or provider secrets.

## PR #76 subscription-auth adoption

PR #76 demonstrates credential isolation and refresh behaviour. Translate that
behaviour into Satyr's existing boundaries.

### Model funding routes

Represent model funding as a discriminated union:

```ts
type ModelFundingRoute =
  | {
      readonly kind: "satyr_api";
      readonly budgetId: ModelBudgetId;
    }
  | {
      readonly kind: "tenant_api_key";
      readonly credentialBindingId: CredentialBindingId;
    }
  | {
      readonly kind: "chatgpt_subscription";
      readonly credentialBindingId: CredentialBindingId;
      readonly accountId: ChatGptAccountId;
    };
```

Each branch requires its complete identity and budget state. Raw tokens never
enter this domain object.

### Login and credential storage

1. Confirm OpenAI's supported flow for a hosted multi-tenant product.
2. Use an official browser or device-code Codex login flow.
3. Bind the approved account and workspace to one Satyr tenant.
4. Store access and refresh material in Satyr's credential vault.
5. Keep refresh-token rotation serialized under the credential authority.
6. Refresh before expiry and perform one revision-guarded retry after `401`.
7. Support explicit revoke, reconnect, disable, and audit operations.
8. Expose plan, quota, reset time, and non-secret health to the operator.

Production onboarding must not copy `~/.codex/auth.json`. That file-reading
approach remains useful only as local feasibility evidence.

### Model host boundary

Picocodex receives `host_managed` authorization. The Satyr Model Gateway opens
the authorized socket and supplies only the resulting transport handle and
non-secret connection metadata.

Access and refresh tokens are excluded from:

- Wasm memory;
- Picocodex snapshots;
- Actor event payloads;
- tool definitions and inputs;
- surface events;
- browser storage;
- logs and traces; and
- executor environments.

### Quota and fallback

Read subscription plan and rate-limit state where the supported OpenAI
interface exposes it. When a limit is reached, Satyr may pause until reset,
request a funding-route change, or use a previously approved fallback.

Satyr never silently changes from subscription-funded inference to
Satyr-funded API usage. Ambient and scheduled work requires a funding route
that explicitly permits unattended execution.

### Cloudflare egress

PR #76 observed a `403` when opening the ChatGPT Responses WebSocket directly
from Cloudflare. Treat this as a release gate:

1. test direct production Cloudflare egress with the supported login flow;
2. seek an OpenAI-supported allowlist or partner path where available;
3. otherwise route through a bounded Satyr-owned non-Cloudflare relay;
4. authenticate both ends and allowlist the upstream host and headers;
5. cap frames, queued bytes, connection duration, retries, and concurrency; and
6. meter relay use as part of the model route.

The relay carries model transport only. It owns no durable agent, tenant state,
tool authority, or refresh token.

## Implementation phases

### NF0 — Freeze evidence and compatibility baseline

- [ ] Record `a4ad251…`, PR #75 head, PR #76 head, PR #80 head, PR #74 head,
  and current upstream master.
- [ ] Save upstream license and commit provenance.
- [ ] Capture the current Satyr native and Wasm dependency graphs.
- [ ] Record optimized Wasm size, Worker bundle size, cold construction time,
  first-turn latency, restore latency, and memory observations.
- [ ] Preserve one deterministic fresh-turn snapshot and one restored-turn
  fixture from the current harness.
- [ ] Record the current harness build, checkpoint format, and tool-catalog
  digest rules.
- [ ] List upstream commits between `a4ad251…` and PR #75's base, classifying
  security, protocol, lifecycle, performance, and unrelated product changes.

NF0 ends with a written compatibility decision. Do not assume snapshots from
`a4ad251…` can resume under Picocodex.

### NF1 — Install the neutral embedded harness architecture

- [x] Define `EmbeddedHarnessBackendId` separately from the sandbox-oriented
  `HarnessId` and `HarnessSpec`.
- [x] Define the fresh and restore construction union, backend manifest,
  adapter, runtime, active-turn, completion, usage, and exhaustive error
  contracts.
- [x] Define one Actor-owned `HarnessToolHost` for direct capabilities, Code
  Mode placement, progress, cancellation, receipts, and settlement.
- [x] Define one `CanonicalHarnessEventSink`; only this sink may append
  backend output to accepted Actor history.
- [x] Add embedded backend ID to checkpoint records, sealed-object metadata,
  semantic binding, persistence validation, and restore validation.
- [x] Start a new checkpoint lineage for the neutral boundary. Close or
  explicitly restart older records instead of adding a checkpoint converter.
- [x] Implement `NanocodexEmbeddedAdapter` around the current pinned
  `a4ad251…` runtime.
- [x] Move Nanocodex construction, restore, model host, event bridge, tool
  mapping, turn execution, steering, cancellation, checkpoint extraction,
  cleanup, and backend error translation behind that adapter.
- [x] Rename Actor RPC methods, active-turn state, inputs, results, and errors
  to embedded-harness terminology.
- [x] Make `AgentSessionActor` depend only on the neutral adapter, tool host,
  event sink, and canonical checkpoint contracts.
- [x] Keep commerce instructions, policy, budget, model-route authority,
  workspace receipts, surface delivery, and execution placement owned by
  Satyr.
- [x] Add one exhaustive adapter selector with the current Nanocodex branch.
- [x] Keep Nanocodex names out of Actor lifecycle, surfaces, Cue processing,
  billing, placement, and canonical event contracts.
- [x] Prove fresh turn, event acceptance, direct tool, Code Mode, cancellation,
  checkpoint, close, restore, and continuation through the adapter operating
  test; keep steering explicit and return `embedded_harness_steer_unsupported`
  from the current backend.
- [x] Prove the existing Slack and Cue fixtures have unchanged accepted events,
  effects, terminal results, and placement decisions.
- [x] Add type fixtures rejecting cross-backend checkpoints, incomplete
  construction states, backend-specific event leakage, and invalid active-turn
  operations.
- [x] Add no Picocodex or Pi dependency in this phase.

NF1 completed on 2026-07-31. The selected adapter still wraps the pinned
Nanocodex runtime. Its steering operation returns the explicit
`embedded_harness_steer_unsupported` result supported by that build. Fresh and
restored turns, canonical events, direct tools, Code Mode, cancellation,
checkpoint persistence, cleanup, and continuation pass through the neutral
boundary. Checkpoint restore rejects backend, build, or format mismatches
before reading R2 or unsealing plaintext.

NF1 passes only when `AgentSessionActor` has no direct import of the
Nanocodex runtime, model host, event bridge, Code Mode adapter, input binder,
or backend error types. The current Nanocodex behavior remains available
through the sole adapter branch.

### NF2 — Import and prune Picocodex

- [x] Clone `gakonst/nanocodex` at `3d4548b…`, then apply the selected PR #75
  commits through `8516800…` in a temporary import repository.
- [x] Verify the checked-out tree and record its archive checksum.
- [ ] Store the immutable source archive in Satyr-controlled artifact storage.
- [x] Copy the license files and create `UPSTREAM.md` and
  `SOURCE_IMPORT.json`.
- [x] Remove the imported `.git` directory before initializing Picocodex.
- [x] Generate native and Wasm reachability reports with `cargo metadata` and
  `cargo tree`.
- [x] Remove unrelated workspace members, sources, manifests, CI jobs, and
  dependencies.
- [x] Rename every retained package, crate, module, symbol, host global,
  artifact, protocol identifier, fixture, and current document to Picocodex.
- [x] Regenerate bindings and lockfiles under the Picocodex identity.
- [x] Prove `nanocodex` appears only in the provenance allowlist.
- [x] Save the import-to-pruned-tree patch and its checksum.
- [x] Add a concise Picocodex README and upstream provenance record.
- [x] Build the retained Rust library for native and Wasm targets.
- [x] Build and package the retained JavaScript/Wasm binding.
- [x] Run retained upstream tests.
- [x] Initialize a new local Git repository after pruning and renaming pass.
- [x] Create the initial repository commit only after pruning and verification.
- [x] Create the independent `seams-tech/picocodex` GitHub repository.
- [x] Push the verified initial commit and make `main` the protected default
  branch.
- [x] Tag the first immutable Picocodex baseline.

The baseline commit contains the Picocodex identity, pruning, required build
repair, host-managed authentication, and Hybrid tool exposure. It contains no
compatibility identity or Satyr product state. The provenance files and binary
transformation patch keep its relationship to the exact imported upstream tree
reviewable without retaining upstream Git metadata.

NF2's local repository work completed on 2026-07-31 at root commit
`a621800`. The untouched upstream bundle, source archive, dependency reports,
and 31.9 MB binary transformation patch are preserved under
`~/Dev/rust/picocodex-upstream-preservation`. The independent public repository
is published at `seams-tech/picocodex`; `main` requires pull requests and one
approval, and tag `import-2026-07-31` identifies the verified import.
Satyr-controlled artifact-storage upload remains pending.

### NF3 — Add the Satyr host contract

- [x] Retain PR #75's asynchronous host-owned WebSocket upgrades.
- [x] Retain bearer and `host_managed` authentication as invalid-by-construction
  alternatives.
- [x] Retain direct hosted tool definitions and execution.
- [ ] Port PR #80's closed model identity into the retained Picocodex crates.
- [ ] Bind the selected model for the complete checkpoint lineage.
- [ ] Preserve model identity through turns, compaction, snapshots, restore,
  forks, prompt-cache keys, usage events, and telemetry.
- [ ] Reject unsupported models and restore-time model mismatches.
- [ ] Emit authoritative model, service tier, and token counts for Satyr
  accounting.
- [ ] Leave provider rates and monetary calculations in Satyr's versioned
  accounting boundary.
- [ ] Exclude PR #80's foreign-rollout fallbacks and removed product consumers.
- [x] Add `HostedToolExposure::Hybrid`.
- [ ] Reject duplicate, closed, invalid, and unauthorized socket upgrades.
- [ ] Reject unadvertised direct tool calls.
- [ ] Preserve typed tool-call and tool-result events.
- [ ] Preserve steering, cancellation, queueing, and safe historical forks.
- [x] Add deterministic tests for Code, Direct, and Hybrid exposure.
- [x] Add a Wasm regression proving a direct tool call uses no evaluator.
- [x] Add a Wasm regression proving Hybrid retains Code Mode.

Complete the remaining NF3 model and accounting work in a focused follow-up
commit. Tag the exact revision intended for Satyr.

### NF4 — Integrate Picocodex into Satyr

- [ ] Change the workspace dependency to Picocodex at
  `https://github.com/seams-tech/picocodex` at the exact NF3 commit.
- [ ] Regenerate `Cargo.lock`.
- [ ] Rebuild `crates/harness-wasm-gate`.
- [ ] Rename Satyr's Nanocodex files, modules, imports, types, constants,
  errors, fixtures, tests, globals, traces, and generated artifacts to
  Picocodex.
- [ ] Replace the `nanocodex` harness ID with `picocodex`.
- [ ] Prove current Satyr source contains no Nanocodex identity outside
  historical evidence and persistence provenance.
- [ ] Update the packaged harness-build identity.
- [ ] Update the checkpoint compatibility boundary.
- [ ] Add the required `picocodex` backend ID to the harness manifest,
  checkpoint metadata, semantic binding, and restore validation.
- [ ] Replace `NanocodexEmbeddedAdapter` with
  `PicocodexEmbeddedAdapter` behind the existing neutral contract.
- [ ] Replace the adapter selector's `nanocodex` branch with `picocodex`.
- [ ] Move Picocodex construction, restore, turn, steer, cancel, checkpoint,
  close, model-host, and error translation behind that adapter.
- [ ] Prove `AgentSessionActor` remains unchanged except for the selected
  backend manifest and lineage identity.
- [ ] Keep Picocodex-specific payloads out of surfaces, Cue processing,
  accounting, and executor placement.
- [ ] Bind Direct and Hybrid catalogs through
  `AgentSessionActor`'s existing tool-catalog digest.
- [ ] Route direct tool execution to the authenticated Satyr capability host.
- [ ] Keep Dynamic Worker Code Mode and Container escalation intact.
- [ ] Remove temporary adapters needed only by `a4ad251…`.

Do not add parallel normalization in TypeScript or Python. Native harness
normalization remains in `crates/harness-server`; the embedded Wasm path emits
the same canonical event contract.

### NF5 — Prove direct-tool economics

- [ ] Run one read-only commerce capability directly.
- [ ] Run one authorized provider read directly.
- [ ] Run one validated workspace changeset directly.
- [ ] Prove these calls create no Dynamic Worker reservation, allocation,
  invocation, or settlement record.
- [ ] Run the equivalent operations through the old Code Mode route in a
  controlled comparison.
- [ ] Record latency, model tokens, Actor CPU, Dynamic Worker count, startup
  CPU, execution CPU, and failure rate.
- [ ] Confirm Code Mode still handles a multi-tool reduction.
- [ ] Confirm a Container still handles a filesystem-heavy operation.

The direct route becomes the default for eligible typed capabilities after
this evidence is accepted.

### NF6 — Add ChatGPT subscription funding

- [ ] Complete the supported-use and Cloudflare-egress feasibility gates.
- [ ] Add the `chatgpt_subscription` funding-route boundary.
- [ ] Implement login, vault storage, refresh serialization, revocation, and
  health reporting.
- [ ] Add host-managed ChatGPT socket acquisition to the Model Gateway.
- [ ] Record plan and rate-limit state without exposing credentials.
- [ ] Implement explicit pause and approved-fallback behaviour.
- [ ] Test concurrent session reconnects against one rotating credential.
- [ ] Test revocation during an active session.
- [ ] Test tenant isolation across access, refresh, logs, snapshots, and
  surface delivery.
- [ ] Prove one interactive subscription-funded turn.
- [ ] Prove one eligible unattended turn only when the account and policy
  explicitly support it.

NF6 remains feature-gated independently from the Picocodex cutover.

### NF7 — Prove the Picocodex reference backend

- [ ] Run the deterministic AR1 fresh, checkpoint, replace, restore, and
  continue sequence.
- [ ] Run AR3 cancellation, stale-result, executor-failure, and sleep tests.
- [ ] Run the complete Slack start, stream, attachment, steer, cancel, restore,
  and delivery flow.
- [ ] Run Cue-triggered and scheduled sleeping-agent tests.
- [ ] Produce one successful Actor-embedded commerce canary using at least one
  direct Satyr tool.
- [ ] Prove the canary allocates no Container.
- [ ] Prove an eligible direct tool call allocates no Dynamic Worker.
- [ ] Record the accepted Actor history, tool receipt, checkpoint, model usage,
  terminal result, latency, and platform cost for the canary.
- [ ] Compare bundle size, cold start, turn latency, restore cost, and memory
  against NF0.
- [ ] Record Picocodex as the first conforming reference implementation of the
  neutral adapter.

NF7 is the gate for all Pi work. A successful harness turn without an accepted
commerce result does not satisfy the canary.

### NF8 — Run the bounded Pi challenger probe

- [ ] Pin exact versions or commits for
  `@earendil-works/pi-agent-core` and `@earendil-works/pi-ai`.
- [ ] Include only the Anthropic provider.
- [ ] Exclude Pi's coding-agent package, CLI, TUI, filesystem tools, shell
  tools, extensions, local authentication, local sessions, and product UI.
- [ ] Construct one fresh Pi agent through a test-only harness conformance
  entry point.
- [ ] Route model access through Satyr's credential-free Model Gateway
  boundary.
- [ ] Advertise and execute exactly one read-only direct Satyr commerce tool.
- [ ] Convert model and tool lifecycle events into Satyr's canonical harness
  events.
- [ ] Run the complete probe under workerd.
- [ ] Record bundle size, cold construction time, first-event latency,
  terminal latency, Actor CPU, memory, model usage, and platform cost.
- [ ] Prove no Container or Dynamic Worker is allocated.
- [ ] Add no checkpoint, restore, durable Pi lineage, production selector,
  tenant setting, provider fallback, or deployment.

NF8 passes only when workerd completes an accepted commerce result, canonical
event ordering is valid, credential isolation holds, and cleanup releases all
runtime resources deterministically. Failure closes the Pi experiment with
recorded evidence and leaves Picocodex as the reference backend.

### NF9 — Add Pi reconstruction and matched evals

Begin NF9 only after NF8 passes.

- [ ] Define a Satyr-versioned Pi checkpoint projection from Actor-authoritative
  accepted history, model-route identity, compaction state, instructions, tool
  catalog, workspace revision, and policy revision.
- [ ] Keep runtime functions, provider clients, credentials, and tool
  implementations outside the checkpoint.
- [ ] Reconstruct a fresh Pi runtime after Actor eviction.
- [ ] Prove continuation creates no duplicate model message, tool effect, or
  accepted event.
- [ ] Add `pi` to the internal backend identity only after fresh, restore, and
  mismatch-rejection tests pass.
- [ ] Run the same Slack start, stream, steer, cancel, restore, and delivery
  fixtures used by Picocodex.
- [ ] Run the same Cue-triggered and scheduled sleeping-agent fixtures used by
  Picocodex.
- [ ] Run matched commerce evals using equivalent instructions, direct tools,
  policy, artifacts, and budgets.
- [ ] Record eval quality, latency, token usage, Actor CPU, memory, checkpoint
  size, restore cost, executor allocations, operational failure rate, and
  total platform cost per successful task.

NF9 does not enable customer selection. It produces comparable evidence for
the backend decision.

### NF10 — Decide, cut over, and govern

- [ ] Classify Pi as the Claude backend, the default commerce backend, or an
  unsupported experiment.
- [ ] Record the decision using NF7 through NF9 evidence.
- [ ] Enable the selected backend configuration for an internal cohort.
- [ ] Observe model errors, checkpoint rejection, tool routing, executor
  allocation, credential health, and cost meters.
- [ ] Record owner acceptance before expanding the cohort.
- [ ] Keep backend identity immutable for each session lineage.
- [ ] Require a newly accepted lineage when changing backend.
- [ ] Complete or explicitly restart old checkpoint lineages.
- [ ] Remove the `a4ad251…` production path after the rollback window closes.
- [ ] Update Plan 14 to record the demonstrated source-import justification and
  final harness build.
- [ ] Update the AR0 feasibility evidence with the retained Picocodex contract.
- [ ] Document the exact upstream base, carried Satyr commits, and removed
  scope.
- [ ] Configure dependency and security scanning for retained crates.
- [ ] Review upstream before model-protocol changes and for relevant security
  fixes.
- [ ] Cherry-pick focused fixes only after differential tests.
- [ ] Avoid routine upstream merges.
- [ ] Delete compatibility code once all old lineages have closed.

Only one harness build is authoritative for a session lineage. A stale build
cannot append accepted Actor history.

## Verification

Run the narrow checks first:

```bash
cargo test --manifest-path crates/harness-server/Cargo.toml
cargo check --workspace
cargo test --workspace
pnpm --filter satyr-edge check:types
pnpm --filter satyr-edge check:test-types
pnpm --filter satyr-edge test
pnpm --filter satyr-edge test:onboarding-closure
python3 -m unittest discover -s services/sandbox -p 'test_*.py'
git diff --check
```

Add Picocodex CI for:

```bash
cargo check -p picocodex --target wasm32-unknown-unknown
cargo test -p picocodex
cargo test -p picocodex-agent
cargo test -p picocodex-tools
```

Add focused Satyr adapter checks for:

- fresh construction, one turn, checkpoint commit, close, restore, and
  continuation;
- steering and cancellation as separate operations;
- canonical event ordering and terminal boundaries;
- model, instruction, tool, workspace, policy, backend, build, and checkpoint
  format mismatch rejection;
- deterministic cleanup after construction and turn failures;
- one exhaustive Picocodex production selector before NF8;
- absence of Pi product routes, flags, credentials, persistence, and
  deployment during NF8; and
- matched Picocodex and Pi conformance fixtures during NF9.

Run the real-harness differential probes where the native adapter changes:

```bash
cargo test --manifest-path crates/harness-server/Cargo.toml \
  real_codex_long_streaming_uses_native_app_server_chunks \
  -- --ignored --nocapture

cargo test --manifest-path crates/harness-server/Cargo.toml \
  real_harnesses_basic_steer_and_resume \
  -- --ignored --nocapture
```

Inspect the emitted protocol. Passing exit codes alone are insufficient.

## Acceptance criteria

The neutral embedded harness architecture is accepted when:

- `AgentSessionActor` imports only neutral embedded harness contracts and the
  exhaustive adapter selector;
- backend construction, restore, event translation, model transport, tool
  mapping, checkpoints, steering, cancellation, and cleanup live behind the
  adapter;
- embedded backend identity is separate from sandbox `HarnessSpec`;
- Actor RPC methods, lifecycle state, surfaces, Cue processing, billing, and
  placement expose no backend-specific types;
- checkpoint records and sealed metadata bind backend ID, build, format, and
  semantic digests;
- a checkpoint from one backend is rejected before another backend sees its
  plaintext;
- current Nanocodex behavior passes through the sole NF1 adapter branch;
- accepted events, effects, terminal results, and placement decisions match
  the NF0 fixtures; and
- static fixtures reject invalid construction states and backend-specific
  leakage across the neutral boundary.

Picocodex is accepted as the reference backend when:

- Satyr consumes one immutable Picocodex revision;
- Picocodex's first commit and current source use the Picocodex identity
  throughout;
- `nanocodex` appears only in explicit upstream provenance and historical
  evidence;
- no legacy crate, package, environment, host-global, harness-ID, or
  checkpoint alias exists;
- the retained dependency graph contains no KVM or removed product surface;
- native and Worker Wasm builds pass;
- direct typed tools execute without Dynamic Worker allocation;
- Hybrid exposure supports direct tools and Code Mode in one stable catalog;
- snapshots reject mismatched backend, build, format, model, instruction, tool,
  workspace, or policy definitions;
- `AgentSessionActor` reaches Picocodex only through the embedded harness
  adapter;
- surfaces and Actor consumers receive only canonical harness events;
- Picocodex is the adapter selector's only production backend through NF8;
- steering, cancellation, restore, and canonical events retain their current
  semantics;
- the Actor remains the sole durable session authority;
- provider credentials remain in Satyr's brokered boundary;
- subscription credentials never cross into Wasm, snapshots, events, tools,
  surfaces, or executors;
- cost and latency observations are recorded;
- one Actor-embedded commerce canary succeeds without a Container; and
- eligible direct tool calls allocate no Dynamic Worker.

The bounded Pi probe is accepted when:

- only `pi-agent-core`, `pi-ai`, and the Anthropic provider enter the probe
  dependency closure;
- one read-only direct Satyr tool completes under workerd;
- the result becomes valid canonical Actor events;
- credentials remain inside Satyr's Model Gateway boundary;
- the run allocates no Container or Dynamic Worker;
- cleanup is deterministic; and
- no Pi persistence, restore, product route, tenant setting, or deployment
  exists.

The overall backend plan closes when:

- a passing Pi probe either completes NF9 or receives a recorded decision to
  stop;
- Picocodex and Pi use the same Slack, Cue, and commerce eval fixtures when
  NF9 runs;
- eval quality, latency, operational complexity, and cost evidence supports
  the recorded Pi classification;
- the selected internal production cohort passes;
- old checkpoint lineages are completed or explicitly restarted; and
- the old harness path and obsolete fixtures are deleted after cutover.

## Rollback

Keep the current `a4ad251…` build deployable until NF10 cohort acceptance.

Rollback changes the cohort's harness selection to the previous immutable
build. Sessions with checkpoints created by Picocodex remain fenced from
the old runtime. They resume only after returning to the compatible build or
start a newly accepted lineage through the persistence boundary.

Do not add bidirectional snapshot conversion or a permanent dual-runtime
compatibility layer.

## Explicit non-goals

- Building a general-purpose agent SDK.
- Shipping Nanocodex KVM or retained VM workspaces.
- Shipping a Nanocodex-owned Cloudflare agent platform.
- Replacing Satyr Actors, surfaces, credential vault, Model Gateway, or
  execution router.
- Shipping Pi before the bounded workerd probe and matched evals pass.
- Supporting Pi's coding-agent product, bundled tools, local session authority,
  or UI.
- Supporting customer-selectable backends before the NF10 decision.
- Allowing per-turn or in-place backend switching.
- Adding a dynamic harness plugin registry or placeholder backend branches.
- Making Picocodex internals provider-neutral.
- Publishing a general Nanocodex CLI, TUI, browser agent, or Python SDK.
- Mirroring every upstream feature or release.
- Supporting mutable shared canonical filesystems.
- Allowing direct tools to bypass policy, approval, budgets, or receipts.
- Silently charging Satyr API usage after subscription quota exhaustion.
- Maintaining the old and new Nanocodex architectures indefinitely.

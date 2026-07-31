import { createRequire } from "node:module";
import initWeb, { Picocodex as WebPicocodex } from "../pkg-web/picocodex.js";

import { agentActions } from "../actions/index.mjs";
import {
  activateHost,
  bindHostSession,
  createAgentClient,
  createEventChannel,
  defineRuntime,
  releaseHostSession,
  toWasmConfig,
} from "../internal.mjs";
import { createNodeHost } from "./host.mjs";

let initializedWeb;
let NodePicocodex;

export function create(options = {}) {
  const {
    thinking,
    reasoningMode,
    fastMode,
    instructions,
    sessionId,
    workspace,
    resume,
    apiKey,
    websocketUrl,
    apiBaseUrl,
    module,
    tools,
    toolMode,
  } = options;
  const events = createEventChannel();
  const host = createNodeHost({
    onEvent: events.emit,
    tools,
    toolMode,
    workspace: workspace ?? resume?.workspace,
  });
  activateHost(host);
  const runtime = defineRuntime({
    key: "node-wasm",
    name: "Picocodex Node WASM",
    type: "node",
    async create(config) {
      activateHost(host);
      const Picocodex = module === undefined
        ? loadNodePicocodex()
        : await loadWebPicocodex(module);
      activateHost(host);
      return new Picocodex(JSON.stringify(toWasmConfig({
        auth: { kind: "api_key", api_key: apiKey },
        websocketUrl,
        apiBaseUrl,
        ...config,
      })));
    },
    subscribe: events.subscribe,
    adopt: (raw) => bindHostSession(host, raw.sessionId),
    release: (raw) => releaseHostSession(host, raw.sessionId),
    decorate: (agent) => agent.extend(agentActions()),
  });
  return createAgentClient(runtime, {
    thinking,
    reasoningMode,
    fastMode,
    instructions,
    sessionId,
    workspace,
    resume,
  });
}

function loadNodePicocodex() {
  const require = createRequire(import.meta.url);
  NodePicocodex ||= require("../pkg-node/picocodex.js").Picocodex;
  return NodePicocodex;
}

async function loadWebPicocodex(module) {
  initializedWeb ||= initWeb({ module_or_path: module });
  await initializedWeb;
  return WebPicocodex;
}

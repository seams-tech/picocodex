import init, { Picocodex } from "../pkg-web/picocodex.js";

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
import { createBrowserHost } from "./host.mjs";

let initialized;

export function create(options = {}) {
  const {
    apiKey,
    hostAuth,
    websocketUrl,
    apiBaseUrl,
    module,
    thinking,
    reasoningMode,
    fastMode,
    instructions,
    sessionId,
    workspace,
    resume,
    WebSocketImpl,
    createWebSocket,
    tools,
    toolMode,
  } = options;
  if (hostAuth && apiKey !== undefined) {
    throw new TypeError("hostAuth is mutually exclusive with apiKey");
  }
  const events = createEventChannel();
  const host = createBrowserHost({
    WebSocketImpl,
    createWebSocket,
    hostAuth: hostAuth === true || apiKey === undefined,
    onEvent: events.emit,
    tools,
    toolMode,
  });
  activateHost(host);
  const runtime = defineRuntime({
    key: "browser-wasm",
    name: "Picocodex Browser WASM",
    type: "browser",
    async create(config) {
      activateHost(host);
      initialized ||= module === undefined ? init() : init({ module_or_path: module });
      await initialized;
      activateHost(host);
      return new Picocodex(JSON.stringify(toWasmConfig({
        auth: apiKey === undefined
          ? { kind: "host_managed" }
          : { kind: "api_key", api_key: apiKey },
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

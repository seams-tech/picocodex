import { Console } from "node:console";
import { createRequire } from "node:module";
import { resolve } from "node:path";
import WebSocket from "ws";
import packageMetadata from "../package.json" with { type: "json" };

import { createCodeRuntime } from "../runtime/code-runtime.mjs";

const RESPONSES_WEBSOCKETS_BETA = "responses_websockets=2026-02-06";
const USER_AGENT = `picocodex-wasm/${packageMetadata.version}`;
const DEFAULT_MAX_QUEUED_MESSAGES = 4_096;
const DEFAULT_MAX_QUEUED_BYTES = 32 * 1024 * 1024;
const DEFAULT_MAX_FRAME_BYTES = 16 * 1024 * 1024;

export function createNodeHost(options = {}) {
  const connections = new Map();
  const code = createCodeRuntime(options.tools, {
    require: createRequire(resolve(options.workspace ?? process.cwd(), ".picocodex-code-mode.cjs")),
    console: new Console({ stdout: process.stderr, stderr: process.stderr }),
  });
  const toolMode = options.toolMode ?? "code";
  if (toolMode !== "code" && toolMode !== "direct" && toolMode !== "hybrid") {
    throw new TypeError("toolMode must be code, direct, or hybrid");
  }
  const onEvent = options.onEvent || (() => {});
  const connectTimeoutMs = options.connectTimeoutMs ?? 30_000;
  const sendTimeoutMs = options.sendTimeoutMs ?? 30_000;
  const maxQueuedMessages = options.maxQueuedMessages ?? DEFAULT_MAX_QUEUED_MESSAGES;
  const maxQueuedBytes = options.maxQueuedBytes ?? DEFAULT_MAX_QUEUED_BYTES;
  const maxFrameBytes = options.maxFrameBytes ?? DEFAULT_MAX_FRAME_BYTES;
  let nextHandle = 1;

  function connect(endpoint, apiKey, sessionId, metadata = {}) {
    return new Promise((resolve, reject) => {
      let settled = false;
      let upgradeResponse;
      const headers = {
        Authorization: `Bearer ${apiKey}`,
        "OpenAI-Beta": RESPONSES_WEBSOCKETS_BETA,
        "x-openai-internal-codex-responses-lite": "true",
        "session-id": sessionId,
        "thread-id": sessionId,
        "x-client-request-id": sessionId,
        "x-responsesapi-include-timing-metrics": "true",
        "User-Agent": USER_AGENT,
      };
      if (metadata.accountId) headers["ChatGPT-Account-ID"] = metadata.accountId;
      if (metadata.fedramp) headers["X-OpenAI-Fedramp"] = "true";
      if (metadata.turnState) headers["x-codex-turn-state"] = metadata.turnState;
      const socket = new WebSocket(endpoint, {
        handshakeTimeout: connectTimeoutMs,
        maxPayload: maxFrameBytes,
        headers,
      });
      const handle = nextHandle++;
      const connection = queueState(socket);

      socket.on("upgrade", (response) => { upgradeResponse = response; });
      socket.on("unexpected-response", (_request, response) => {
        if (settled) return;
        settled = true;
        const chunks = [];
        response.on("data", (chunk) => chunks.push(Buffer.from(chunk)));
        response.on("end", () => {
          const error = new Error(`WebSocket handshake was rejected with HTTP ${response.statusCode}`);
          error.status = response.statusCode;
          error.body = chunks.length ? Buffer.concat(chunks).toString("utf8") : "empty response body";
          const retryAfter = Number(header(response.headers, "retry-after"));
          if (Number.isFinite(retryAfter) && retryAfter >= 0) error.retryAfter = retryAfter;
          reject(error);
        });
      });
      socket.on("open", () => {
        settled = true;
        connections.set(handle, connection);
        const headers = upgradeResponse?.headers || {};
        resolve(JSON.stringify({
          handle,
          status: upgradeResponse?.statusCode || 101,
          request_id: header(headers, "x-request-id"),
          server_model: header(headers, "openai-model"),
          reasoning_included: header(headers, "x-reasoning-included") !== undefined,
          turn_state: header(headers, "x-codex-turn-state"),
        }));
      });
      socket.on("message", (data, isBinary) => {
        enqueue(connection, isBinary
          ? { kind: "binary" }
          : { kind: "text", text: data.toString("utf8") });
      });
      socket.on("close", (status, reason) => {
        if (!connection.intentionallyClosed && !connection.overflowed) {
          const suffix = reason.length ? `: ${reason.toString("utf8")}` : "";
          enqueue(connection, { kind: "closed", detail: `with code ${status}${suffix}` });
        }
      });
      socket.on("error", (error) => {
        if (!settled) {
          settled = true;
          reject(error);
        } else {
          enqueue(connection, { kind: "error", detail: errorMessage(error) });
        }
      });
    });
  }

  function send(handle, message) {
    const connection = connections.get(handle);
    if (!connection || connection.socket.readyState !== WebSocket.OPEN) {
      return Promise.resolve(JSON.stringify({
        ok: false,
        reconnectable: true,
        error: "WebSocket is no longer open",
      }));
    }
    return new Promise((resolve) => {
      let completed = false;
      const timer = setTimeout(() => finish({
        ok: false,
        reconnectable: false,
        error: `sending a WebSocket frame exceeded ${sendTimeoutMs} milliseconds`,
      }), sendTimeoutMs);
      function finish(result) {
        if (completed) return;
        completed = true;
        clearTimeout(timer);
        resolve(JSON.stringify(result));
      }
      connection.socket.send(message, (error) => finish(error ? {
        ok: false,
        reconnectable: connection.socket.readyState !== WebSocket.OPEN,
        error: errorMessage(error),
      } : { ok: true }));
    });
  }

  function next(handle, timeoutMs) {
    const connection = connections.get(handle);
    if (!connection) {
      return Promise.resolve(JSON.stringify({ kind: "closed", detail: "before the next frame" }));
    }
    if (connection.queue.length) {
      const entry = connection.queue.shift();
      connection.queuedBytes -= entry.bytes;
      return Promise.resolve(JSON.stringify(entry.message));
    }
    if (connection.waiter) return Promise.reject(new Error("concurrent reads are unsupported"));
    return new Promise((resolve) => {
      const timer = setTimeout(() => {
        connection.waiter = undefined;
        resolve(JSON.stringify({ kind: "timeout" }));
      }, timeoutMs);
      connection.waiter = (message) => {
        clearTimeout(timer);
        connection.waiter = undefined;
        resolve(JSON.stringify(message));
      };
    });
  }

  function close(handle) {
    const connection = connections.get(handle);
    if (!connection) return;
    connections.delete(handle);
    connection.intentionallyClosed = true;
    connection.waiter?.({ kind: "closed", detail: "by the WASM runtime" });
    connection.socket.close();
  }

  function enqueue(connection, message) {
    if (connection.overflowed) return;
    if (connection.waiter) {
      connection.waiter(message);
      return;
    }
    const bytes = messageBytes(message);
    if (connection.queue.length >= maxQueuedMessages || connection.queuedBytes + bytes > maxQueuedBytes) {
      connection.queue.length = 0;
      connection.queuedBytes = 0;
      connection.overflowed = true;
      const error = {
        kind: "error",
        detail: `receive queue exceeded ${maxQueuedMessages} messages or ${maxQueuedBytes} bytes`,
      };
      connection.queue.push({ message: error, bytes: messageBytes(error) });
      if (typeof connection.socket.terminate === "function") connection.socket.terminate();
      else connection.socket.close(1009, "receive queue exceeded configured bounds");
      return;
    }
    connection.queue.push({ message, bytes });
    connection.queuedBytes += bytes;
  }

  return Object.freeze({
    connect,
    send,
    next,
    close,
    sleep: (milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds)),
    executeCode: code.executeCode,
    executeTool: code.executeTool,
    toolMode: () => toolMode,
    toolDefinitions: code.toolDefinitions,
    emitEvent: onEvent,
    reset: code.reset,
  });
}

function queueState(socket) {
  return {
    socket,
    queue: [],
    queuedBytes: 0,
    waiter: undefined,
    intentionallyClosed: false,
    overflowed: false,
  };
}

function header(headers, name) {
  const value = headers[name];
  return Array.isArray(value) ? value[0] : value;
}

function messageBytes(message) {
  return Buffer.byteLength(message.kind === "text" ? message.text : JSON.stringify(message), "utf8");
}

function errorMessage(error) {
  return error && (error.stack || error.message) || String(error);
}

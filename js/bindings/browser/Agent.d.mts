import type {
  AgentOptions,
  DefaultAgent,
  ToolMap,
} from "../types.mjs";
import type {
  BrowserWebSocketConnection,
  BrowserWebSocketRequest,
} from "./host.mjs";

export type Agent = DefaultAgent;

/** Creates a browser- or Worker-hosted Rust/WASM Agent. */
export function create(options?: create.Options): Promise<create.ReturnType>;
export declare namespace create {
  type Options = AgentOptions & (
    | { apiKey?: string | undefined; hostAuth?: never }
    | { apiKey?: never; hostAuth?: true }
  ) & {
    WebSocketImpl?: typeof WebSocket | undefined;
    apiBaseUrl?: string | undefined;
    createWebSocket?(
      endpoint: string,
      sessionId: string,
      request: BrowserWebSocketRequest,
    ): WebSocket | BrowserWebSocketConnection | Promise<WebSocket | BrowserWebSocketConnection>;
    module?: unknown;
    tools?: ToolMap | undefined;
    /** Direct dispatch is CSP-safe; Code Mode requires dynamic JavaScript evaluation. */
    toolMode?: "code" | "direct" | "hybrid" | undefined;
    websocketUrl?: string | undefined;
  };
  type ReturnType = Agent;
}

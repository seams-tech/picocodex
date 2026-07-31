import type {
  AgentOptions,
  DefaultAgent,
  ToolMap,
} from "../types.mjs";

export type Agent = DefaultAgent;

/** Creates a Node-hosted Rust/WASM Agent. */
export function create(options: create.Options): Promise<create.ReturnType>;
export declare namespace create {
  type Options = AgentOptions & {
    apiKey: string;
    apiBaseUrl?: string | undefined;
    module?: unknown;
    tools?: ToolMap | undefined;
    toolMode?: "code" | "direct" | "hybrid" | undefined;
    websocketUrl?: string | undefined;
  };
  type ReturnType = Agent;
}

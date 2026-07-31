# Picocodex Agent

The owned lifecycle for one headless `OpenAI` coding agent.

`picocodex-agent` composes the Tower-native Responses state machine from
`picocodex-oai-api` with the runtime from `picocodex-tools`. A normal consumer
builds one agent, receives a cheap cloneable [`Picocodex`] handle and an
independent [`AgentEvents`] stream, then submits ordered prompts.

## Quick start

```rust,no_run
use picocodex_agent::{Picocodex, OpenAi};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let openai = OpenAi::new(std::env::var("OPENAI_API_KEY")?)?;
let (agent, _events) = Picocodex::builder(openai)
    .instructions(
        "You are a Rust coding agent. Preserve unrelated work and run relevant tests.",
    )
    .workspace(std::env::current_dir()?)
    .build()?;

let result = agent
    .prompt("Explain the cause of the failing parser test.")
    .await?
    .await?;
println!("{}", result.final_message());
agent.shutdown().await?;
# Ok(())
# }
```

The first `await` means the private driver accepted and ordered the prompt.
[`Turn`] is both a per-turn event stream and a future for [`TurnResult`].
Awaiting the turn waits only for its result; event consumption is independent.

The private driver is the sole owner of mutable conversation, transport, tool,
and process state. Cloning [`Picocodex`] only clones its command capability;
[`Picocodex::spawn`] creates a clean sibling and [`Picocodex::fork`] creates an
independent branch from committed history.

## Typed events

[`AgentEvents`] is optional and independent from turn results. Its raw
JSONL-compatible envelope remains lossless, while
[`AgentEvent::data`](picocodex_agent::events::AgentEvent::data) provides a
normalized domain view:

```rust,no_run
use futures_util::StreamExt;
use picocodex_agent::{
    Picocodex, OpenAi,
    events::{AgentEventData, AssistantEvent},
};

# async fn run() -> Result<(), Box<dyn std::error::Error>> {
let openai = OpenAi::new(std::env::var("OPENAI_API_KEY")?)?;
let (agent, mut events) = Picocodex::builder(openai)
    .instructions("Answer concisely and preserve exact identifiers.")
    .build()?;
let turn = agent.prompt("Explain the identifier req_7f3.").await?;

while let Some(event) = events.next().await {
    if let AgentEventData::Assistant(AssistantEvent::Delta(delta)) = event.data()? {
        print!("{}", delta.text);
    }
    if event.kind.is_terminal() {
        break;
    }
}
let _result = turn.await?;
agent.shutdown().await?;
# Ok(())
# }
```

## Components

- [`events`](picocodex_agent::events) contains the complete typed lifecycle
  event taxonomy.
- [`input`](picocodex_agent::input) contains prompts and multimodal user input.
- [`session`](picocodex_agent::session) contains durable session identities and
  snapshots.
- [`usage`](picocodex_agent::usage) contains model-aware token accounting.
- [`transport`](picocodex_agent::transport) exposes advanced Responses and
  Tower configuration.
- [`tools`](picocodex_agent::tools) exposes the complete tool implementation
  surface.

OpenAI API-key and managed ChatGPT credentials belong to
[`picocodex_oai_api::auth`], independently of this lifecycle crate.

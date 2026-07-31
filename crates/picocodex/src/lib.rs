#![doc = include_str!("../README.md")]
#![deny(missing_docs, rustdoc::broken_intra_doc_links)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub use picocodex_agent::{
    AgentEvents, Picocodex, PicocodexBuilder, PicocodexError, PromptRoute, ServiceTier, Turn,
    TurnControl, TurnResult, TurnUsage,
};
pub use picocodex_oai_api::{Model, OpenAi, ReasoningMode, Thinking};
#[cfg(not(target_family = "wasm"))]
#[cfg_attr(docsrs, doc(cfg(not(target_family = "wasm"))))]
pub use picocodex_tools::tool;
pub use picocodex_tools::{Tool, Tools};

/// Owned agent lifecycle, builders, turns, branching, and snapshots.
///
/// Provider and tool-runtime APIs keep their canonical detailed paths under
/// [`crate::oai`] and [`crate::tools`].
pub mod agent {
    pub use picocodex_agent::{
        AgentEvents, AgentHandle, Picocodex, PicocodexBuilder, PicocodexError, PromptRoute, Result,
        ServiceTier, Turn, TurnControl, TurnResult, TurnUsage, events, input, session, usage,
    };
}

/// Tower-native OpenAI Responses client, sessions, protocol, and transport.
#[doc(inline)]
pub use picocodex_oai_api as oai;

/// Tool registry, built-ins, MCP, tool search, and Code Mode.
#[doc(inline)]
pub use picocodex_tools as tools;

/// Common imports for the golden owned-agent path.
pub mod prelude {
    #[cfg(not(target_family = "wasm"))]
    #[cfg_attr(docsrs, doc(cfg(not(target_family = "wasm"))))]
    pub use crate::tool;
    pub use crate::{Model, OpenAi, Picocodex, PicocodexBuilder, Tool, Tools};
}

#[cfg(not(target_family = "wasm"))]
#[doc(hidden)]
pub mod __private {
    pub use picocodex_tools::__private::*;
}

use std::{io, path::PathBuf, sync::Arc};

use picocodex_oai_api::ResponseError;
pub use picocodex_oai_api::transport::ResponsesError;

/// Error returned by the Picocodex library boundary.
#[derive(Debug, thiserror::Error)]
pub enum PicocodexError {
    /// Caller input or two configured policies are incompatible.
    #[error("invalid task request: {0}")]
    InvalidRequest(String),

    /// The configured workspace could not be resolved.
    #[error("failed to resolve task workspace {path}: {source}")]
    ResolveWorkspace {
        /// Workspace path supplied by the caller.
        path: PathBuf,
        /// Underlying filesystem failure.
        #[source]
        source: io::Error,
    },

    /// The resolved workspace exists but is not a directory.
    #[error("task workspace is not a directory: {path}")]
    WorkspaceNotDirectory {
        /// Resolved workspace path.
        path: PathBuf,
    },

    /// The resolved workspace cannot be represented as UTF-8.
    #[error("task workspace path is not valid UTF-8: {path}")]
    WorkspaceNotUtf8 {
        /// Resolved workspace path.
        path: PathBuf,
    },

    /// A follow-on prompt attempted to change an owned session's workspace.
    #[error("an active agent session cannot change workspace from {current} to {requested}")]
    WorkspaceChanged {
        /// Workspace already owned by the session.
        current: String,
        /// Conflicting workspace requested by the caller.
        requested: String,
    },

    /// A completed provider response violated an agent-loop invariant.
    #[error("malformed Responses API event: {detail}")]
    MalformedResponse {
        /// Stable invariant failure description.
        detail: &'static str,
    },

    /// A service returned an output for the wrong kind of attempt.
    #[error("invalid Responses attempt state: {detail}")]
    InvalidAttemptState {
        /// Stable invalid-state description.
        detail: &'static str,
    },

    /// The immutable request prefix could not be serialized for fingerprinting.
    #[error("failed to fingerprint the immutable prompt prefix: {0}")]
    SerializePromptPrefix(#[source] serde_json::Error),

    /// The private driver stopped before accepting a command.
    #[error("the agent stopped before accepting the command")]
    AgentStopped,

    /// The private driver stopped after accepting a turn but before delivering its result.
    #[error("the agent stopped before the turn completed")]
    TurnStopped,

    /// Shared cleanup failure returned to every caller of an idempotent
    /// shutdown.
    #[error(transparent)]
    Shutdown(Arc<Self>),

    /// Steering targeted a queued or terminal turn.
    #[error("the targeted turn is queued, completed, or otherwise not active for steering")]
    TurnNotSteerable,

    /// The active turn cannot accept more queued steering input.
    #[error("the active turn's steering queue is full")]
    SteerQueueFull,

    /// Cancellation targeted an already terminal turn.
    #[error("the targeted turn has already completed or been cancelled")]
    TurnNotCancellable,

    /// The targeted turn was cancelled after its resources stopped.
    #[error("the turn was cancelled")]
    TurnCancelled,

    /// A fork was requested before any safe committed boundary existed.
    #[error("the agent has no safe conversation boundary to fork")]
    ForkBeforeCompletedTurn,

    /// A historical result came from a different conversation lineage.
    #[error("the completed turn belongs to a different conversation lineage")]
    CheckpointLineageMismatch,

    /// A serialized session snapshot failed structural or policy validation.
    #[error("invalid session snapshot: {0}")]
    InvalidSessionSnapshot(String),

    /// Agent construction was attempted outside an active Tokio runtime.
    #[error("building an agent requires an active Tokio runtime")]
    TokioRuntimeUnavailable,

    /// Contractual agent event serialization failed.
    #[error(transparent)]
    Event(#[from] picocodex_oai_api::events::EventError),

    /// A complete Responses operation failed.
    #[error(transparent)]
    Response(#[from] ResponseError),

    /// The configured tool registry or runtime could not be built.
    #[cfg(not(target_family = "wasm"))]
    #[error("failed to build tools for an agent driver: {0}")]
    Tools(#[from] picocodex_tools::ToolsBuildError),
}

impl PicocodexError {
    /// Returns the underlying Responses transport/API error, including when a
    /// caller-provided Tower middleware boxed the standard service error.
    #[must_use]
    pub fn responses_error(&self) -> Option<&ResponsesError> {
        match self {
            Self::Response(error) => error.responses_error(),
            Self::Shutdown(error) => error.responses_error(),
            _ => None,
        }
    }
}

/// Result type returned by the owned agent lifecycle.
pub type Result<T> = std::result::Result<T, PicocodexError>;

#[cfg(test)]
mod tests {
    use super::{PicocodexError, ResponsesError};
    use picocodex_oai_api::{ResponseError, tower::ResponsesServiceError};

    #[test]
    fn response_error_is_the_single_provider_failure_boundary() {
        let service = PicocodexError::Response(ResponseError::from(ResponsesServiceError::from(
            ResponsesError::UnexpectedEnd,
        )));
        assert!(matches!(
            service.responses_error(),
            Some(ResponsesError::UnexpectedEnd)
        ));

        let service = ResponsesServiceError::from(ResponsesError::UnexpectedEnd);
        let error =
            PicocodexError::Response(ResponseError::from(Box::new(service) as tower::BoxError));
        assert!(matches!(
            error.responses_error(),
            Some(ResponsesError::UnexpectedEnd)
        ));
        assert_eq!(
            error.to_string(),
            "Responses WebSocket closed without a close frame"
        );
    }
}

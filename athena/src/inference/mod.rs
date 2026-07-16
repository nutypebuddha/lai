//! # Inference Backend — Capstone copilot layer
//!
//! Athena's capstone: a tiny local or remote LLM copilot that assists with
//! natural language routing, formula suggestion, and explanation generation.
//!
//! ## Architecture
//!
//! ```text
//!                    ┌─────────────────────┐
//!                    │   InferenceConfig    │
//!                    │  (TOML / env vars)   │
//!                    └──────┬──────────────┘
//!                           │
//! ┌──────────────────────────────────────────┐
//! │          InferenceBackend trait          │
//! │  fn generate(&mut self, req) -> Response │
//! │  fn health(&self) -> HealthStatus        │
//! └──────────┬──────────────────┬───────────┘
//!            │                  │
//! ┌──────────▼──────┐  ┌───────▼──────────┐
//! │   LocalBackend   │  │  RemoteBackend   │
//! │ (llama-gguf      │  │ (HTTP / ollama   │
//! │  Engine, qwen    │  │  llama.cpp svr)  │
//! │  0.5B GGUF)      │  │                  │
//! └─────────────────┘  └──────────────────┘
//! ```

pub mod config;
pub mod manifest;

#[cfg(feature = "llm")]
pub mod download;
#[cfg(feature = "llm")]
pub mod local;
#[cfg(feature = "llm")]
pub mod remote;

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Errors from inference operations.
#[derive(Error, Debug)]
pub enum InferenceError {
    #[error("model not loaded: {0}")]
    ModelNotLoaded(String),

    #[error("inference failed: {0}")]
    InferenceFailed(String),

    #[error("backend unavailable: {0}")]
    BackendUnavailable(String),

    #[error("config error: {0}")]
    ConfigError(String),

    #[error("health check failed: {0}")]
    HealthCheckFailed(String),

    #[error("not supported: {0}")]
    NotSupported(String),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Which inference backend to use.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub enum BackendKind {
    /// Local GGUF model via llama-gguf (Qwen2.5-0.5B recommended)
    #[default]
    Local,
    /// Remote HTTP API (ollama, llama.cpp server, OpenAI-compatible)
    Remote,
}

impl fmt::Display for BackendKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Local => write!(f, "local"),
            Self::Remote => write!(f, "remote"),
        }
    }
}

impl std::str::FromStr for BackendKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "remote" => Ok(Self::Remote),
            other => Err(format!(
                "unknown backend kind: '{other}'. Expected 'local' or 'remote'"
            )),
        }
    }
}

/// A request to the inference backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    /// The user prompt / input text
    pub prompt: String,
    /// Optional system prompt override
    pub system_prompt: Option<String>,
    /// Sampling temperature (0.0 = deterministic, 1.0 = creative)
    pub temperature: Option<f64>,
    /// Maximum tokens to generate
    pub max_tokens: Option<usize>,
    /// Stop sequences
    pub stop_sequences: Option<Vec<String>>,
    /// Whether to include usage stats in response
    pub include_usage: bool,
}

impl InferenceRequest {
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            system_prompt: None,
            temperature: None,
            max_tokens: None,
            stop_sequences: None,
            include_usage: false,
        }
    }

    pub fn with_system(mut self, system: impl Into<String>) -> Self {
        self.system_prompt = Some(system.into());
        self
    }

    pub fn with_temp(mut self, temp: f64) -> Self {
        self.temperature = Some(temp);
        self
    }
}

/// A response from the inference backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    /// The generated text
    pub text: String,
    /// Number of tokens generated
    pub tokens_generated: usize,
    /// Tokens per second
    pub tokens_per_second: f64,
    /// Why generation finished
    pub finish_reason: String,
    /// Raw model output (if available)
    pub raw: Option<String>,
}

/// Health status of a backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub healthy: bool,
    pub model_loaded: Option<String>,
    pub backend_kind: BackendKind,
    pub context_size: Option<usize>,
    pub message: String,
}

/// A capability advertised by a backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub version: Option<String>,
    pub supported_features: Vec<String>,
}

/// The core inference trait — implemented by LocalBackend and RemoteBackend.
///
/// # Determinism Guarantee
///
/// Athena's symbolic engine is purely deterministic. The copilot is an
/// advisory layer on top — its outputs are **never** used for core
/// computation, only for NL expansion, routing hints, and explanation.
pub trait InferenceBackend {
    /// Generate a response for the given request.
    fn generate(&mut self, request: InferenceRequest) -> Result<InferenceResponse, InferenceError>;

    /// Check backend health.
    fn health(&self) -> HealthStatus;

    /// List capabilities.
    fn capabilities(&self) -> Vec<Capability>;

    /// Get the backend kind.
    fn kind(&self) -> BackendKind;
}

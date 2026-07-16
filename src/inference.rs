//! Inference — local LLM front for the Laverna personality agent.
//!
//! Behind `--features llm`, the `Copilot` spawns a locally-installed llama.cpp
//! binary (path from `LAVERNA_LLAMA_BIN`) and pipes the persona prompt plus the
//! grounded tool result. This keeps the build dependency-free (no bundled model
//! runtime) while staying fully offline. Every answer is run through
//! `companion::sanitize_answer` so internal mechanics/brand never leak.

use std::process::{Command, Stdio};

use crate::companion::{sanitize_answer, PERSONA_SYSTEM_PROMPT};

/// Hint from the copilot for token descent (kept for the descent pipeline).
#[derive(Debug, Clone, Default)]
pub struct DescentHint {
    pub domains: Vec<String>,
    pub entity: Option<String>,
    pub formula: Option<String>,
    pub confidence: f64,
}

/// Error type for local-model invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CopilotError {
    BinaryMissing,
    SpawnFailed(String),
    NoOutput,
}

/// Local llama.cpp-backed persona. Pure-ish: given a prompt it returns the
/// model's natural-language answer (sanitized). No global state.
#[derive(Debug, Clone, Default)]
pub struct Copilot;

impl Copilot {
    /// Resolve the local model binary path from `LAVERNA_LLAMA_BIN`.
    pub fn binary_path() -> Option<String> {
        std::env::var("LAVERNA_LLAMA_BIN")
            .ok()
            .filter(|s| !s.is_empty())
    }

    /// Run the persona over `grounded_context` (the routed tool result + any
    /// user question) and return the sanitized natural-language answer.
    pub fn answer(&self, user_query: &str, grounded_context: &str) -> Result<String, CopilotError> {
        let bin = Self::binary_path().ok_or(CopilotError::BinaryMissing)?;
        let prompt = format!(
            "{}\n\nContext (verified):\n{}\n\nUser: {}\n\nLaverna:",
            PERSONA_SYSTEM_PROMPT, grounded_context, user_query
        );
        let child = Command::new(&bin)
            .arg("--temp")
            .arg("0.2")
            .arg("-p")
            .arg(&prompt)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| CopilotError::SpawnFailed(e.to_string()))?;
        let output = child
            .wait_with_output()
            .map_err(|e| CopilotError::SpawnFailed(e.to_string()))?;
        if !output.status.success() {
            return Err(CopilotError::SpawnFailed(format!("exit {}", output.status)));
        }
        let raw = String::from_utf8_lossy(&output.stdout);
        let answer = raw
            .lines()
            .find(|l| !l.trim().is_empty())
            .map(|l| l.trim().to_string())
            .ok_or(CopilotError::NoOutput)?;
        Ok(sanitize_answer(&answer))
    }

    /// Descent hint (unchanged stub path; used by the descent planner).
    pub fn descend_token(
        &self,
        _token: &str,
        _context: Option<&str>,
    ) -> Result<DescentHint, String> {
        Ok(DescentHint::default())
    }
}

pub mod copilot {
    pub use super::{Copilot, CopilotError, DescentHint};
}

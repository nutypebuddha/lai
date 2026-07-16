//! # Gates — validation gates
//!
//! Athena's validation gates evaluate reasoning chains across multiple dimensions:
//!
//! - **Math gate**: verifies numeric calculations and formula evaluations
//! - **Logic gate**: validates logical form and inference patterns
//! - **Fact gate**: validates claims via chain traversal (no static facts)
//! - **Confidence gate**: assigns confidence via composition analysis
//! - **Formal gate**: validates structural integrity of formulas and chains
//!
//! Each gate produces structured output with a pass/fail status, a confidence score,
//! and actionable feedback.

use serde::{Deserialize, Serialize};

pub(crate) mod confidence;
pub(crate) mod fact;
pub(crate) mod formal;
pub(crate) mod logic;
pub(crate) mod math;

pub use confidence::ConfidenceGate;
pub use fact::FactGate;
pub use formal::FormalGate;
pub use logic::LogicGate;
pub use math::MathGate;

/// The result of a single gate check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateOutput {
    /// The gate that produced this result.
    pub gate: String,
    /// Whether the check passed.
    pub passed: bool,
    /// Confidence score (0.0–1.0).
    pub confidence: f64,
    /// Human-readable description.
    pub message: String,
    /// Specific issues found (if any).
    pub issues: Vec<String>,
    /// Suggested fixes (if any).
    pub suggestions: Vec<String>,
}

/// The combined result of all gate checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GateResult {
    pub gates: Vec<GateOutput>,
    pub overall_passed: bool,
    pub overall_confidence: f64,
    pub summary: String,
}

impl GateResult {
    /// Create a new gate result from individual gate outputs.
    pub fn new(gates: Vec<GateOutput>) -> Self {
        let passed_count = gates.iter().filter(|g| g.passed).count();
        let total = gates.len();
        let overall_passed = passed_count == total;
        let overall_confidence = if total > 0 {
            gates.iter().map(|g| g.confidence).sum::<f64>() / total as f64
        } else {
            0.0
        };

        let summary = if overall_passed {
            format!(
                "Passed {}/{} gates (confidence: {:.2})",
                passed_count, total, overall_confidence
            )
        } else {
            let failed: Vec<&str> = gates
                .iter()
                .filter(|g| !g.passed)
                .map(|g| g.gate.as_str())
                .collect();
            format!(
                "Failed {}/{} gates: {} (confidence: {:.2})",
                total - passed_count,
                total,
                failed.join(", "),
                overall_confidence
            )
        };

        GateResult {
            gates,
            overall_passed,
            overall_confidence,
            summary,
        }
    }

    /// Run a set of gates against a target.
    pub fn run(gates: Vec<Box<dyn Gate>>, target: &str) -> Self {
        let outputs: Vec<GateOutput> = gates.iter().map(|g| g.check(target)).collect();
        Self::new(outputs)
    }
}

/// A single validation gate.
pub trait Gate {
    fn name(&self) -> &str;
    fn check(&self, target: &str) -> GateOutput;
}

/// A helper to build a GateOutput.
pub fn gate_output(
    gate: &str,
    passed: bool,
    confidence: f64,
    message: String,
    issues: Vec<String>,
    suggestions: Vec<String>,
) -> GateOutput {
    GateOutput {
        gate: gate.to_string(),
        passed,
        confidence,
        message,
        issues,
        suggestions,
    }
}

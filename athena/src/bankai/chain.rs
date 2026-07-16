//! # Chain — a complete reasoning chain with evaluation results
//!
//! A chain is the output of Bankai's chain execution. It records every step
//! of formula evaluation, including inputs, outputs, and errors.

use std::collections::HashMap;

use serde::Serialize;

use crate::wheel::Domain;

use super::Aspect;

/// The status of a single step in a chain.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ChainStatus {
    /// Successfully evaluated.
    Success,
    /// Evaluation failed.
    Failed,
    /// Step was skipped due to a prior failure.
    Skipped,
}

/// A single step in a reasoning chain.
#[derive(Debug, Clone, Serialize)]
pub struct ChainStep {
    pub formula_id: String,
    pub domain: Domain,
    pub inputs: HashMap<String, f64>,
    pub output_name: String,
    pub output_value: f64,
    pub status: ChainStatus,
    pub error: Option<String>,
}

impl ChainStep {
    /// Format this step as a readable line.
    pub fn format(&self) -> String {
        match &self.status {
            ChainStatus::Success => {
                let inputs_str: Vec<String> = self
                    .inputs
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect();
                format!(
                    "  ✓ {} ({}): {}({}) = {} = {:.6}",
                    self.formula_id,
                    self.domain.symbol(),
                    self.formula_id,
                    inputs_str.join(", "),
                    self.output_name,
                    self.output_value
                )
            }
            ChainStatus::Failed => {
                format!(
                    "  ✗ {} ({}): FAILED — {}",
                    self.formula_id,
                    self.domain.symbol(),
                    self.error.as_deref().unwrap_or("unknown error")
                )
            }
            ChainStatus::Skipped => {
                format!(
                    "  - {} ({}): SKIPPED",
                    self.formula_id,
                    self.domain.symbol()
                )
            }
        }
    }
}

/// The result of executing a chain of formulas.
#[derive(Debug, Clone, Serialize)]
pub struct ChainResult {
    pub steps: Vec<ChainStep>,
    pub total_domains: Vec<Domain>,
    pub total_aspects: Vec<Aspect>,
    pub success: bool,
    /// Confidence computed via semiring: compose (⊗) across steps.
    pub confidence: f64,
}

impl ChainResult {
    /// The final output value (if chain succeeded).
    pub fn final_output(&self) -> Option<f64> {
        self.steps
            .iter()
            .rev()
            .find(|s| matches!(s.status, ChainStatus::Success))
            .map(|s| s.output_value)
    }

    /// The number of successful steps.
    pub fn success_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| matches!(s.status, ChainStatus::Success))
            .count()
    }

    /// The number of failed steps.
    pub fn failure_count(&self) -> usize {
        self.steps
            .iter()
            .filter(|s| matches!(s.status, ChainStatus::Failed))
            .count()
    }

    /// Format the entire chain as a readable string.
    pub fn format(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("Chain: {} step(s)\n", self.steps.len()));
        for step in &self.steps {
            s.push_str(&step.format());
            s.push('\n');
        }
        s.push_str(&format!(
            "\nDomains: {}\n",
            self.total_domains
                .iter()
                .map(|d| format!("{}{}", d.symbol(), d.full_name()))
                .collect::<Vec<_>>()
                .join(" → ")
        ));
        s.push_str(&format!(
            "Status: {} (confidence: {:.2})\n",
            if self.success {
                "✓ SUCCESS"
            } else {
                "✗ FAILED"
            },
            self.confidence
        ));
        if let Some(final_val) = self.final_output() {
            s.push_str(&format!("Final value: {:.6}\n", final_val));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_step_format_success() {
        let mut args = HashMap::new();
        args.insert("x".to_string(), 5.0);
        let step = ChainStep {
            formula_id: "double".to_string(),
            domain: Domain::Mangala,
            inputs: args,
            output_name: "y".to_string(),
            output_value: 10.0,
            status: ChainStatus::Success,
            error: None,
        };
        let formatted = step.format();
        assert!(formatted.contains("✓"));
        assert!(formatted.contains("double"));
    }

    #[test]
    fn test_chain_step_format_failure() {
        let step = ChainStep {
            formula_id: "bad_formula".to_string(),
            domain: Domain::Shukra,
            inputs: HashMap::new(),
            output_name: "x".to_string(),
            output_value: 0.0,
            status: ChainStatus::Failed,
            error: Some("division by zero".to_string()),
        };
        let formatted = step.format();
        assert!(formatted.contains("✗"));
        assert!(formatted.contains("division by zero"));
    }

    #[test]
    fn test_chain_result_final_output() {
        let result = ChainResult {
            steps: vec![ChainStep {
                formula_id: "a".to_string(),
                domain: Domain::Mangala,
                inputs: HashMap::new(),
                output_name: "x".to_string(),
                output_value: 5.0,
                status: ChainStatus::Success,
                error: None,
            }],
            total_domains: vec![Domain::Mangala],
            total_aspects: vec![],
            success: true,
            confidence: 0.95,
        };
        assert_eq!(result.final_output(), Some(5.0));
    }

    #[test]
    fn test_chain_format() {
        let result = ChainResult {
            steps: vec![],
            total_domains: vec![],
            total_aspects: vec![],
            success: true,
            confidence: 1.0,
        };
        let formatted = result.format();
        assert!(formatted.contains("SUCCESS"));
    }
}

//! # EvalEngine — formula evaluation with Tanto
//!
//! Evaluates a formula expression against given input values.
//! Uses Tanto (pure Rust recursive-descent evaluator with exact rational
//! arithmetic) instead of a heavyweight JIT compiler — formulas are simple
//! arithmetic expressions like `mass * acceleration` or `0.5 * mass * velocity^2`.

use std::collections::HashMap;

use crate::formula::Formula;
use crate::tanto::{evaluate as tanto_eval, TantoEnv};

use super::BankaiError;

/// The evaluation engine using Tanto (pure Rust, zero allocation per eval).
///
/// Tanto provides:
/// - Deterministic evaluation (same input → same output, always)
/// - Exact rational arithmetic for fraction expressions
/// - Sandboxed — no filesystem, no network, no unsafe
/// - Built-in physical constants and unit conversions
#[derive(Debug, Clone)]
pub struct EvalEngine;

impl Default for EvalEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl EvalEngine {
    /// Create a new evaluation engine.
    pub fn new() -> Self {
        EvalEngine
    }

    /// Evaluate a formula with the given named arguments.
    pub fn evaluate(
        &self,
        formula: &Formula,
        args: &HashMap<String, f64>,
    ) -> Result<f64, BankaiError> {
        // Verify all required inputs are present
        for input in &formula.inputs {
            if !args.contains_key(input) {
                return Err(BankaiError::EvalError(format!(
                    "missing argument: '{}' for formula '{}'",
                    input, formula.id
                )));
            }
        }

        // Build Tanto environment from args
        let env = TantoEnv::from_map(args.clone());

        // Evaluate using Tanto
        let expr_str = preprocess_expression(&formula.expression);
        let result = tanto_eval(&expr_str, &env).ok_or_else(|| {
            BankaiError::EvalError(format!(
                "eval error in '{}': expression '{}'",
                formula.id, expr_str
            ))
        })?;

        if !result.is_finite() {
            return Err(BankaiError::EvalError(format!(
                "invalid result in '{}': {} — domain/range violation",
                formula.id,
                if result.is_nan() { "NaN" } else { "inf" }
            )));
        }

        Ok(result)
    }
}

/// Preprocess expression to handle syntax transitions from meval to Tanto.
///
/// Tanto natively supports:
/// - `**` for power (same as `^`)
/// - `log10`, `log2`, `log` (ln) as named functions
/// - All trig, rounding, and stats functions
///
/// Transformations applied:
/// - `.sqrt()` → `` (Rust-style method call to bare sqrt — legacy cleanup)
fn preprocess_expression(expr: &str) -> String {
    expr.replace(".sqrt()", "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formula::Formula;
    use crate::wheel::Domain;

    #[test]
    fn test_eval_simple() {
        let engine = EvalEngine::new();
        let formula = Formula::atomic("test", Domain::Mangala, vec!["x"], "y", "x * 2", "double");
        let mut args = HashMap::new();
        args.insert("x".to_string(), 5.0);
        let result = engine.evaluate(&formula, &args).unwrap();
        assert!((result - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_multi_arg() {
        let engine = EvalEngine::new();
        let formula = Formula::atomic("test", Domain::Mangala, vec!["a", "b"], "c", "a + b", "sum");
        let mut args = HashMap::new();
        args.insert("a".to_string(), 3.0);
        args.insert("b".to_string(), 4.0);
        let result = engine.evaluate(&formula, &args).unwrap();
        assert!((result - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_missing_arg_order_independent() {
        let engine = EvalEngine::new();
        let formula = Formula::atomic("test", Domain::Mangala, vec!["x", "y"], "z", "x + y", "sum");
        let mut args1 = HashMap::new();
        args1.insert("x".to_string(), 3.0);
        args1.insert("y".to_string(), 4.0);
        let mut args2 = HashMap::new();
        args2.insert("y".to_string(), 4.0);
        args2.insert("x".to_string(), 3.0);
        let result1 = engine.evaluate(&formula, &args1).unwrap();
        let result2 = engine.evaluate(&formula, &args2).unwrap();
        assert!((result1 - result2).abs() < 1e-10);
    }

    #[test]
    fn test_eval_missing_arg() {
        let engine = EvalEngine::new();
        let formula = Formula::atomic("test", Domain::Mangala, vec!["x"], "y", "x * 2", "double");
        let args = HashMap::new();
        let result = engine.evaluate(&formula, &args);
        assert!(result.is_err());
    }

    #[test]
    fn test_eval_complex_expression() {
        let engine = EvalEngine::new();
        let formula = Formula::atomic(
            "pythagorean",
            Domain::Mangala,
            vec!["a", "b"],
            "c",
            "sqrt(a^2 + b^2)",
            "hypotenuse",
        );
        let mut args = HashMap::new();
        args.insert("a".to_string(), 3.0);
        args.insert("b".to_string(), 4.0);
        let result = engine.evaluate(&formula, &args).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_log10() {
        let engine = EvalEngine::new();
        let formula = Formula::atomic(
            "entropy",
            Domain::Mangala,
            vec!["probability"],
            "bits",
            "-probability * log10(probability) / log10(2)",
            "Shannon entropy",
        );
        let mut args = HashMap::new();
        args.insert("probability".to_string(), 0.5);
        let result = engine.evaluate(&formula, &args).unwrap();
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_eval_log2() {
        let engine = EvalEngine::new();
        let formula = Formula::atomic(
            "log2_test",
            Domain::Mangala,
            vec!["x"],
            "y",
            "log2(x)",
            "log base 2",
        );
        let mut args = HashMap::new();
        args.insert("x".to_string(), 8.0);
        let result = engine.evaluate(&formula, &args).unwrap();
        println!("log2(8) = {}", result);
        assert!((result - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_eval_log_function() {
        let engine = EvalEngine::new();
        let mut args = HashMap::new();
        args.insert("x".to_string(), std::f64::consts::E);
        let formula = Formula::atomic(
            "log_natural",
            Domain::Mangala,
            vec!["x"],
            "y",
            "log(x)",
            "natural log",
        );
        let result = engine.evaluate(&formula, &args).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }
}

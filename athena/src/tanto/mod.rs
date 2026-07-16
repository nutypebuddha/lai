//! # Tanto — deterministic expression evaluator
//!
//! Pure Rust recursive-descent expression parser with exact rational arithmetic.
//! Replaces `meval` as Athena's formula evaluation backend.
//!
//! ## Design
//!
//! - **No external dependencies** — zero allocations during parsing, byte-level input
//! - **Two-phase evaluation**: try exact rational first, fallback to f64
//! - **Deterministic** — same input + same env → same output, always
//! - **Sandboxed** — no filesystem, no network, no unsafe, bounded recursion
//!
//! ## Supported Syntax
//!
//! | Category | Operators |
//! |----------|-----------|
//! | Arithmetic | `+`, `-`, `*`, `/`, `^`, `%` |
//! | Unary | `-`, `+` |
//! | Grouping | `(`, `)` |
//! | Functions | `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2` |
//! | Functions | `sqrt`, `abs`, `hypot`, `pow`, `exp`, `ln`, `log10`, `log2` |
//! | Functions | `round`, `floor`, `ceil`, `min`, `max`, `sum`, `avg`, `clamp` |
//! | Constants | `pi`, `e`, `c`, `g`, `G`, `h`, `hbar`, `kB`, `NA`, `R` |
//! | Units | `mi_to_km`, `km_to_mi`, `f_to_c`, `c_to_f`, `lb_to_kg`, etc. |
//!
//! ## Usage
//!
//! ```rust
//! let mut env = athena::tanto::TantoEnv::new();
//! env.set("mass", 5.0);
//! env.set("acceleration", 9.8);
//! let result = athena::tanto::evaluate("mass * acceleration", &env).unwrap();
//! assert!((result - 49.0).abs() < 1e-10);
//! ```

pub mod math;
pub mod parser;
pub mod rational;

use std::collections::HashMap;

/// Tanto environment: variable bindings + previous answer.
///
/// Deterministic by construction: no randomness, no system state.
#[derive(Debug, Clone)]
pub struct TantoEnv {
    /// Variable bindings (name → value).
    pub vars: HashMap<String, f64>,
    /// Previous answer (for chained `ans` references).
    pub ans: Option<f64>,
}

impl Default for TantoEnv {
    fn default() -> Self {
        Self::new()
    }
}

impl TantoEnv {
    /// Create a new empty environment.
    pub fn new() -> Self {
        TantoEnv {
            vars: HashMap::new(),
            ans: None,
        }
    }

    /// Create an environment from an existing variable map.
    pub fn from_map(vars: HashMap<String, f64>) -> Self {
        TantoEnv { vars, ans: None }
    }

    /// Get a variable value by name.
    /// Checks built-in constants first, then user variables.
    pub fn get(&self, name: &str) -> Option<f64> {
        if name == "ans" {
            return self.ans;
        }
        // Check built-in constants
        if let Some(val) = get_constant(name) {
            return Some(val);
        }
        self.vars.get(name).copied()
    }

    /// Set a variable value.
    pub fn set(&mut self, name: &str, val: f64) {
        self.vars.insert(name.to_string(), val);
    }

    /// Set the previous answer.
    pub fn set_ans(&mut self, val: f64) {
        self.ans = Some(val);
    }

    /// Number of variables in the environment.
    pub fn len(&self) -> usize {
        self.vars.len()
    }

    /// Whether the environment is empty.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
}

// ─── Constants ───────────────────────────────────────────────────────────────

/// Built-in physical constants.
#[allow(clippy::excessive_precision)]
pub fn get_constant(name: &str) -> Option<f64> {
    match name {
        "pi" => Some(std::f64::consts::PI),
        "e" => Some(std::f64::consts::E),
        "c" => Some(299_792_458.0),
        "c_squared" => Some(89_875_517_873_681_764.0),
        "R_air" => Some(287.058_694_911_492_55),
        "g" => Some(9.806_65),
        "G" => Some(6.674_30e-11),
        "h" => Some(6.626_070_15e-34),
        "hbar" => Some(1.054_571_817e-34),
        "kB" => Some(1.380_649e-23),
        "e_charge" => Some(1.602_176_634e-19),
        "me" => Some(9.109_383_701_5e-31),
        "mp" => Some(1.672_621_923_69e-27),
        "NA" => Some(6.022_140_76e23),
        "R" => Some(8.314_462_618),
        "atm" => Some(101_325.0),
        "Rearth" => Some(6_371_000.0),
        "GMearth" => Some(3.986_004_418e14),
        _ => None,
    }
}

// ─── Evaluation ──────────────────────────────────────────────────────────────

/// Evaluate a math expression string using Tanto's two-phase evaluator.
///
/// Phase 1: Try exact rational arithmetic (for fraction expressions like `1/3 + 1/6`)
/// Phase 2: Fall back to f64 floating-point evaluation
pub fn evaluate(expr: &str, env: &TantoEnv) -> Option<f64> {
    // Try exact rational arithmetic first (for fraction expressions)
    if let Some(rat) = rational::eval_rational(expr, env) {
        return Some(rat.to_f64());
    }
    parser::eval_math(expr.as_bytes(), env).or_else(|| parser::eval_op_format(expr.as_bytes(), env))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env_get_constant() {
        let env = TantoEnv::new();
        assert!((env.get("pi").unwrap() - std::f64::consts::PI).abs() < 1e-15);
        assert!((env.get("e").unwrap() - std::f64::consts::E).abs() < 1e-15);
        assert_eq!(env.get("c").unwrap(), 299_792_458.0);
    }

    #[test]
    fn test_env_set_and_get() {
        let mut env = TantoEnv::new();
        env.set("x", 42.0);
        assert!((env.get("x").unwrap() - 42.0).abs() < 1e-15);
    }

    #[test]
    fn test_env_unknown_var() {
        let env = TantoEnv::new();
        assert_eq!(env.get("nonexistent"), None);
    }

    #[test]
    fn test_env_ans() {
        let mut env = TantoEnv::new();
        env.set_ans(100.0);
        assert!((env.get("ans").unwrap() - 100.0).abs() < 1e-15);
    }

    #[test]
    fn test_evaluate_simple() {
        let env = TantoEnv::new();
        let result = evaluate("2 + 3", &env).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_with_vars() {
        let mut env = TantoEnv::new();
        env.set("mass", 5.0);
        env.set("acceleration", 9.8);
        let result = evaluate("mass * acceleration", &env).unwrap();
        assert!((result - 49.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_power() {
        let env = TantoEnv::new();
        let result = evaluate("2 ^ 3", &env).unwrap();
        println!("2 ^ 3 = {}", result);
        assert!((result - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_function() {
        let env = TantoEnv::new();
        let result = evaluate("sqrt(9) + abs(-5)", &env).unwrap();
        assert!((result - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_evaluate_rational() {
        let env = TantoEnv::new();
        // 1/3 + 1/6 = 1/2 via exact rational
        let result = evaluate("1/3 + 1/6", &env).unwrap();
        assert!((result - 0.5).abs() < 1e-15);
    }

    #[test]
    fn test_get_constant_all() {
        let constants = [
            "pi",
            "e",
            "c",
            "c_squared",
            "R_air",
            "g",
            "G",
            "h",
            "hbar",
            "kB",
            "e_charge",
            "me",
            "mp",
            "NA",
            "R",
            "atm",
            "Rearth",
            "GMearth",
        ];
        for name in &constants {
            assert!(get_constant(name).is_some(), "missing constant: {}", name);
        }
        assert_eq!(get_constant("nonexistent"), None);
    }
}

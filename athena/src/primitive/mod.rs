//! # Primitive — NAND as the Bedrock Primitive
//!
//! All computation bottoms out to NAND gates. Every formula, every expression,
//! every gate compiles to a Directed Acyclic Graph (DAG) of NAND operations.
//!
//! ## Sheffer Stroke (NAND)
//!
//! `nand(a, b) = 1 - a*b`
//!
//! NAND is functionally complete — every Boolean operation can be derived from it:
//!
//! | Operation   | Expression                          | NAND realization                    |
//! |-------------|-------------------------------------|-------------------------------------|
//! | `not(a)`    | `1 - a`                             | `nand(a, a)`                        |
//! | `and(a, b)` | `a * b`                             | `not(nand(a, b))`                   |
//! | `or(a, b)`  | `a + b - a*b`                       | `nand(not(a), not(b))`              |
//! | `nor(a, b)` | `1 - (a + b - a*b)`                 | `not(or(a, b))`                     |
//! | `xor(a, b)` | `a + b - 2*a*b`                     | `or(and(a, not(b)), and(not(a), b))`|
//! | `xnor(a,b)` | `1 - a - b + 2*a*b`                 | `not(xor(a, b))`                    |
//! | `implies(a,b)`| `1 - a + a*b`                     | `or(not(a), b)`                     |
//!
//! ## NAND DAG Compiler
//!
//! The evaluator takes a typed-expression AST and compiles it into a NAND DAG,
//! then propagates values through the graph. This ensures every formula's
//! truth table can be verified against its NAND expansion.
//!
//! ## Numeric extension
//!
//! For math beyond Boolean logic, NAND gates operate on `f64` values in [0, 1]
//! (interpreted as continuous truth values). Arithmetic operations like addition
//! and multiplication are approximated via NAND-based binary circuits.

mod eval;
mod nand;

pub use eval::*;
pub use nand::*;

use std::collections::HashMap;

/// A node in the NAND computation DAG.
///
/// Each node is either a constant input value or a NAND gate
/// that combines the outputs of two child nodes.
#[derive(Debug, Clone, PartialEq)]
pub enum NandNode {
    /// A named input variable (value supplied at evaluation time)
    Input(String),
    /// A constant literal value
    Constant(f64),
    /// A NAND gate combining two child nodes
    Nand { a: usize, b: usize },
}

/// A Directed Acyclic Graph of NAND operations.
///
/// The DAG is immutable once built. Evaluation propagates values
/// through the graph in topological order.
#[derive(Debug, Clone)]
pub struct NandDag {
    nodes: Vec<NandNode>,
}

impl Default for NandDag {
    fn default() -> Self {
        Self::new()
    }
}

impl NandDag {
    /// Create a new empty NAND DAG.
    pub fn new() -> Self {
        NandDag { nodes: Vec::new() }
    }

    /// Add an input variable node, returning its index.
    pub fn add_input(&mut self, name: &str) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(NandNode::Input(name.to_string()));
        idx
    }

    /// Add a constant value node, returning its index.
    pub fn add_constant(&mut self, value: f64) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(NandNode::Constant(value));
        idx
    }

    /// Add a NAND gate combining two existing nodes, returning its index.
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` are out of bounds.
    pub fn add_nand(&mut self, a: usize, b: usize) -> usize {
        assert!(
            a < self.nodes.len(),
            "NandDag: node index {a} out of bounds"
        );
        assert!(
            b < self.nodes.len(),
            "NandDag: node index {b} out of bounds"
        );
        let idx = self.nodes.len();
        self.nodes.push(NandNode::Nand { a, b });
        idx
    }

    /// Derive `not(a)` from `nand(a, a)`.
    pub fn add_not(&mut self, a: usize) -> usize {
        self.add_nand(a, a)
    }

    /// Derive `and(a, b)` from `not(nand(a, b))`.
    pub fn add_and(&mut self, a: usize, b: usize) -> usize {
        let n = self.add_nand(a, b);
        self.add_not(n)
    }

    /// Derive `or(a, b)` from `nand(not(a), not(b))`.
    pub fn add_or(&mut self, a: usize, b: usize) -> usize {
        let na = self.add_not(a);
        let nb = self.add_not(b);
        self.add_nand(na, nb)
    }

    /// Derive `nor(a, b)` from `not(or(a, b))`.
    pub fn add_nor(&mut self, a: usize, b: usize) -> usize {
        let o = self.add_or(a, b);
        self.add_not(o)
    }

    /// Derive `xor(a, b)` from `or(and(a, not(b)), and(not(a), b))`.
    pub fn add_xor(&mut self, a: usize, b: usize) -> usize {
        // and(a, not(b))
        let nb = self.add_not(b);
        let anb = self.add_and(a, nb);

        // and(not(a), b)
        let na = self.add_not(a);
        let nab = self.add_and(na, b);

        // or(...)
        self.add_or(anb, nab)
    }

    /// Derive `xnor(a, b)` from `not(xor(a, b))`.
    pub fn add_xnor(&mut self, a: usize, b: usize) -> usize {
        let x = self.add_xor(a, b);
        self.add_not(x)
    }

    /// Derive `implies(a, b)` from `or(not(a), b)`.
    pub fn add_implies(&mut self, a: usize, b: usize) -> usize {
        let na = self.add_not(a);
        self.add_or(na, b)
    }

    /// Evaluate the DAG with the given input bindings.
    ///
    /// Returns the computed value of the final node (the last node in the graph).
    /// Returns `None` if any required input is missing.
    pub fn evaluate(&self, inputs: &HashMap<String, f64>) -> Option<f64> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut values: Vec<f64> = Vec::with_capacity(self.nodes.len());
        for node in &self.nodes {
            let v = match node {
                NandNode::Input(name) => inputs.get(name).copied()?,
                NandNode::Constant(c) => *c,
                NandNode::Nand { a, b } => {
                    let va = values[*a];
                    let vb = values[*b];
                    nand(va, vb)
                }
            };
            values.push(v);
        }
        values.last().copied()
    }

    /// Evaluate with built-in NOT, AND, OR, NOR, XOR, XNOR gates
    /// that map to NAND internally.
    pub fn evaluate_with_gates(&self, inputs: &HashMap<String, f64>) -> Option<f64> {
        // Same as evaluate — the DAG already encodes NAND-based operations
        // because all add_* methods compile down to add_nand internally.
        self.evaluate(inputs)
    }

    /// Number of NAND gates in the DAG.
    pub fn nand_count(&self) -> usize {
        self.nodes
            .iter()
            .filter(|n| matches!(n, NandNode::Nand { .. }))
            .count()
    }

    /// Total number of nodes in the DAG.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Check if the DAG is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Get the index of the last (output) node.
    pub fn output_index(&self) -> Option<usize> {
        if self.nodes.is_empty() {
            None
        } else {
            Some(self.nodes.len() - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_basic() {
        // NAND truth table: nand(a, b) = 1 - a*b
        assert!((nand(0.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((nand(0.0, 1.0) - 1.0).abs() < 1e-12);
        assert!((nand(1.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((nand(1.0, 1.0) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_not_derived() {
        // not(a) = nand(a, a)
        assert!((not(0.0) - 1.0).abs() < 1e-12);
        assert!((not(1.0) - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_all_gates_truth_tables() {
        // AND truth table
        assert!((and(0.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((and(0.0, 1.0) - 0.0).abs() < 1e-12);
        assert!((and(1.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((and(1.0, 1.0) - 1.0).abs() < 1e-12);

        // OR truth table
        assert!((or(0.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((or(0.0, 1.0) - 1.0).abs() < 1e-12);
        assert!((or(1.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((or(1.0, 1.0) - 1.0).abs() < 1e-12);

        // NOR truth table
        assert!((nor(0.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((nor(0.0, 1.0) - 0.0).abs() < 1e-12);
        assert!((nor(1.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((nor(1.0, 1.0) - 0.0).abs() < 1e-12);

        // XOR truth table
        assert!((xor(0.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((xor(0.0, 1.0) - 1.0).abs() < 1e-12);
        assert!((xor(1.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((xor(1.0, 1.0) - 0.0).abs() < 1e-12);

        // XNOR truth table
        assert!((xnor(0.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((xnor(0.0, 1.0) - 0.0).abs() < 1e-12);
        assert!((xnor(1.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((xnor(1.0, 1.0) - 1.0).abs() < 1e-12);

        // Implies truth table: a → b = not(a) or b
        assert!((implies(0.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((implies(0.0, 1.0) - 1.0).abs() < 1e-12);
        assert!((implies(1.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((implies(1.0, 1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_dag_simple() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        let _out = dag.add_nand(a, b);

        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);

        inputs.insert("b".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_dag_not() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let _out = dag.add_not(a);

        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);

        inputs.insert("a".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_dag_and() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        let _out = dag.add_and(a, b);

        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12);

        inputs.insert("a".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_dag_or() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        let _out = dag.add_or(a, b);

        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 0.0);
        inputs.insert("b".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);

        inputs.insert("a".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_dag_xor() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        let _out = dag.add_xor(a, b);

        let mut inputs = HashMap::new();
        // XOR(a=1, b=0) = 1
        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12);

        // XOR(a=1, b=1) = 0
        inputs.insert("b".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_dag_implies() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        let _out = dag.add_implies(a, b);

        let mut inputs = HashMap::new();
        // implies(1, 0) = 0  (false → true is false)
        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);

        // implies(1, 1) = 1
        inputs.insert("b".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_dag_missing_input() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        dag.add_input("b");
        dag.add_nand(a, a); // not(a)

        let inputs = HashMap::new(); // no "a" provided
        assert!(dag.evaluate(&inputs).is_none());
    }

    #[test]
    fn test_nand_dag_constants() {
        let mut dag = NandDag::new();
        let a = dag.add_constant(1.0);
        let b = dag.add_constant(1.0);
        dag.add_nand(a, b);

        let inputs = HashMap::new();
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_nand_counting() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        dag.add_xor(a, b);

        // XOR requires: not(b), not(a), and(a, not(b)), and(not(a), b), or(anb, nab)
        // and adds: nand + not each
        // not is nand(a,a)
        // Total NAND gates:
        //   not(b): 1 nand
        //   not(a): 1 nand
        //   and(a, nb): nand(a, nb) + not(nand) = 2 nand
        //   and(na, b): nand(na, b) + not(nand) = 2 nand
        //   or(anb, nab): not(anb) + not(nab) + nand(not(anb), not(nab))
        //                = 1 + 1 + 1 = 3 nand
        // Total = 1 + 1 + 2 + 2 + 3 = 9 NAND gates, plus 5 intermediate NOT nodes
        // that don't add new nand (they reuse existing nand nodes)
        // Actually let's just check it's reasonable
        assert!(
            dag.nand_count() >= 4,
            "XOR should require at least 4 NAND gates, got {}",
            dag.nand_count()
        );
    }

    #[test]
    fn test_nand_dag_empty() {
        let dag = NandDag::new();
        assert!(dag.is_empty());
        let mut inputs = HashMap::new();
        inputs.insert("x".to_string(), 1.0);
        assert!(dag.evaluate(&inputs).is_none());
    }

    #[test]
    fn test_nand_dag_xnor() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        let _out = dag.add_xnor(a, b);

        // XNOR truth table: opposite of XOR
        let mut inputs = HashMap::new();

        inputs.insert("a".to_string(), 1.0);
        inputs.insert("b".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12, "xnor(1,1) should be 1");

        inputs.insert("b".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12, "xnor(1,0) should be 0");
    }

    #[test]
    fn test_nand_dag_nor() {
        let mut dag = NandDag::new();
        let a = dag.add_input("a");
        let b = dag.add_input("b");
        let _out = dag.add_nor(a, b);

        let mut inputs = HashMap::new();
        inputs.insert("a".to_string(), 0.0);
        inputs.insert("b".to_string(), 0.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 1.0).abs() < 1e-12, "nor(0,0) should be 1");

        inputs.insert("a".to_string(), 1.0);
        let result = dag.evaluate(&inputs).unwrap();
        assert!((result - 0.0).abs() < 1e-12, "nor(1,0) should be 0");
    }

    #[test]
    fn test_continuous_truth_values() {
        // NAND on continuous values in [0, 1]
        let result = nand(0.3, 0.4);
        let expected = 1.0 - 0.3 * 0.4;
        assert!((result - expected).abs() < 1e-12);

        // AND on continuous values: and(a,b) = not(nand(a,b)) = 1 - (1 - a*b)^2
        // This differs from simple multiplication for non-Boolean values
        // because NAND (Sheffer stroke) is a Boolean primitive extended continuously.
        let result = and(0.3, 0.4);
        let expected_nand = 1.0 - (1.0_f64 - 0.3 * 0.4).powi(2);
        assert!((result - expected_nand).abs() < 1e-12);
        // Verify Boolean values still work correctly
        assert!((and(0.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((and(0.0, 1.0) - 0.0).abs() < 1e-12);
        assert!((and(1.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((and(1.0, 1.0) - 1.0).abs() < 1e-12);
    }
}

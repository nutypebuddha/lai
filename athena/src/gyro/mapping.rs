//! # Primitive Mapping — sign → NAND primitive formulas
//!
//! Each zodiac sign maps to a set of atomic NAND-based primitives that fire
//! when the gyroscope aligns with that sign. This is the routing table:
//! sign determines which logical truth-functions are available.
//!
//! Note: Only logic gates (nand, not, and, or, nor, xor, xnor, implies)
//! are handled by the NandExpression compiler. Math primitives (add, mul, etc.)
//! are evaluated by the bankai/meval evaluator.

use crate::primitive::NandExpression;

/// A single primitive entry: name + compiled NAND expression
#[derive(Debug, Clone)]
pub struct PrimitiveEntry {
    pub name: &'static str,
    pub expression: NandExpression,
    pub description: &'static str,
    pub arity: usize, // number of inputs
}

/// Complete mapping of all 12 signs to their primitive formulas
#[derive(Debug, Clone)]
pub struct PrimitiveMapping {
    entries: [Vec<PrimitiveEntry>; 12],
}

impl Default for PrimitiveMapping {
    fn default() -> Self {
        // Build the mapping
        let mut entries: [Vec<PrimitiveEntry>; 12] = Default::default();

        // Aries (0) — Math & Logic: core NAND logic gates
        // These are the fundamental truth-functions
        entries[0] = vec![
            PrimitiveEntry {
                name: "nand",
                expression: NandExpression::compile("nand(a,b)").unwrap(),
                description: "Sheffer stroke — universal gate",
                arity: 2,
            },
            PrimitiveEntry {
                name: "not",
                expression: NandExpression::compile("not(a)").unwrap(),
                description: "Negation",
                arity: 1,
            },
            PrimitiveEntry {
                name: "and",
                expression: NandExpression::compile("and(a,b)").unwrap(),
                description: "Conjunction",
                arity: 2,
            },
            PrimitiveEntry {
                name: "or",
                expression: NandExpression::compile("or(a,b)").unwrap(),
                description: "Disjunction",
                arity: 2,
            },
            PrimitiveEntry {
                name: "xor",
                expression: NandExpression::compile("xor(a,b)").unwrap(),
                description: "Exclusive or",
                arity: 2,
            },
            PrimitiveEntry {
                name: "implies",
                expression: NandExpression::compile("implies(a,b)").unwrap(),
                description: "Material implication",
                arity: 2,
            },
            PrimitiveEntry {
                name: "xnor",
                expression: NandExpression::compile("xnor(a,b)").unwrap(),
                description: "Equivalence (NOT XOR)",
                arity: 2,
            },
            PrimitiveEntry {
                name: "nor",
                expression: NandExpression::compile("nor(a,b)").unwrap(),
                description: "NOT OR",
                arity: 2,
            },
        ];

        // Taurus (1) — Physics & Chemistry: logic for physical laws
        entries[1] = vec![
            PrimitiveEntry {
                name: "implies",
                expression: NandExpression::compile("implies(a,b)").unwrap(),
                description: "Causality: if force then acceleration",
                arity: 2,
            },
            PrimitiveEntry {
                name: "and",
                expression: NandExpression::compile("and(a,b)").unwrap(),
                description: "Conjunction of conditions",
                arity: 2,
            },
            PrimitiveEntry {
                name: "not",
                expression: NandExpression::compile("not(a)").unwrap(),
                description: "Negation (no force = no acceleration)",
                arity: 1,
            },
        ];

        // Gemini (2) — Astronomy & Cosmology
        entries[2] = vec![
            PrimitiveEntry {
                name: "or",
                expression: NandExpression::compile("or(a,b)").unwrap(),
                description: "Disjunction of observations",
                arity: 2,
            },
            PrimitiveEntry {
                name: "xor",
                expression: NandExpression::compile("xor(a,b)").unwrap(),
                description: "Mutually exclusive states",
                arity: 2,
            },
        ];

        // Cancer (3) — Earth & Environment
        entries[3] = vec![
            PrimitiveEntry {
                name: "nor",
                expression: NandExpression::compile("nor(a,b)").unwrap(),
                description: "Neither condition holds",
                arity: 2,
            },
            PrimitiveEntry {
                name: "implies",
                expression: NandExpression::compile("implies(a,b)").unwrap(),
                description: "Environmental causality",
                arity: 2,
            },
        ];

        // Leo (4) — Biology & Medicine
        entries[4] = vec![
            PrimitiveEntry {
                name: "and",
                expression: NandExpression::compile("and(a,b)").unwrap(),
                description: "Multiple symptoms required",
                arity: 2,
            },
            PrimitiveEntry {
                name: "or",
                expression: NandExpression::compile("or(a,b)").unwrap(),
                description: "Alternative pathways",
                arity: 2,
            },
        ];

        // Virgo (5) — Economics & Finance
        entries[5] = vec![
            PrimitiveEntry {
                name: "implies",
                expression: NandExpression::compile("implies(a,b)").unwrap(),
                description: "Market implication: if rate_up then bond_down",
                arity: 2,
            },
            PrimitiveEntry {
                name: "xor",
                expression: NandExpression::compile("xor(a,b)").unwrap(),
                description: "Mutually exclusive positions",
                arity: 2,
            },
        ];

        // Libra (6) — Engineering & Tech
        entries[6] = vec![
            PrimitiveEntry {
                name: "and",
                expression: NandExpression::compile("and(a,b)").unwrap(),
                description: "All conditions must be met",
                arity: 2,
            },
            PrimitiveEntry {
                name: "not",
                expression: NandExpression::compile("not(a)").unwrap(),
                description: "Failure mode analysis",
                arity: 1,
            },
        ];

        // Scorpio (7) — Computer Science & AI
        entries[7] = vec![
            PrimitiveEntry {
                name: "xor",
                expression: NandExpression::compile("xor(a,b)").unwrap(),
                description: "Bitwise XOR / difference",
                arity: 2,
            },
            PrimitiveEntry {
                name: "nand",
                expression: NandExpression::compile("nand(a,b)").unwrap(),
                description: "Universal gate for circuit synthesis",
                arity: 2,
            },
        ];

        // Sagittarius (8) — History & Anthropology
        entries[8] = vec![
            PrimitiveEntry {
                name: "or",
                expression: NandExpression::compile("or(a,b)").unwrap(),
                description: "Alternative historical paths",
                arity: 2,
            },
            PrimitiveEntry {
                name: "implies",
                expression: NandExpression::compile("implies(a,b)").unwrap(),
                description: "Historical causality",
                arity: 2,
            },
        ];

        // Capricorn (9) — Language & Linguistics
        entries[9] = vec![
            PrimitiveEntry {
                name: "and",
                expression: NandExpression::compile("and(a,b)").unwrap(),
                description: "Grammatical conjunction",
                arity: 2,
            },
            PrimitiveEntry {
                name: "not",
                expression: NandExpression::compile("not(a)").unwrap(),
                description: "Negation operator",
                arity: 1,
            },
        ];

        // Aquarius (10) — Philosophy & Ethics
        entries[10] = vec![
            PrimitiveEntry {
                name: "implies",
                expression: NandExpression::compile("implies(a,b)").unwrap(),
                description: "Logical implication",
                arity: 2,
            },
            PrimitiveEntry {
                name: "xnor",
                expression: NandExpression::compile("xnor(a,b)").unwrap(),
                description: "Equivalence / biconditional",
                arity: 2,
            },
        ];

        // Pisces (11) — Psychology & Neuroscience
        entries[11] = vec![
            PrimitiveEntry {
                name: "or",
                expression: NandExpression::compile("or(a,b)").unwrap(),
                description: "Associative activation",
                arity: 2,
            },
            PrimitiveEntry {
                name: "and",
                expression: NandExpression::compile("and(a,b)").unwrap(),
                description: "Binding of features",
                arity: 2,
            },
        ];

        PrimitiveMapping { entries }
    }
}

impl PrimitiveMapping {
    /// Get primitives for a sign
    pub fn for_sign(&self, sign: crate::astrology::Sign) -> &[PrimitiveEntry] {
        &self.entries[sign.index()]
    }

    /// Get all primitives across all signs
    pub fn all(&self) -> Vec<&PrimitiveEntry> {
        self.entries.iter().flatten().collect()
    }

    /// Get primitives matching a name pattern
    pub fn find(&self, name: &str) -> Vec<&PrimitiveEntry> {
        self.entries
            .iter()
            .flatten()
            .filter(|e| e.name.contains(name))
            .collect()
    }
}

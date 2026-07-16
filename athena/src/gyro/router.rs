//! # Gyro Router — routes tokens through aligned primitives to NAND core
//!
//! The router takes a classified token, determines which primitives are aligned
//! with the gyroscope's current orientation, and evaluates them as NAND expressions.

use crate::astrology::{ChangeSorter, Sign};
use crate::gyro::mapping::{PrimitiveEntry, PrimitiveMapping};
use crate::gyro::state::GyroState;
use crate::primitive::NandExpression;

/// Result of routing a token through the gyro
#[derive(Debug, Clone)]
pub struct RouteResult {
    /// The token's dominant sign
    pub sign: Sign,
    /// Primitives that fired (aligned with gyro)
    pub fired_primitives: Vec<FiredPrimitive>,
    /// The NAND expression for evaluation
    pub nand_expression: Option<NandExpression>,
    /// Gyro alignment confidence
    pub confidence: f64,
}

/// A primitive that fired with its inputs
#[derive(Debug, Clone)]
pub struct FiredPrimitive {
    pub entry: PrimitiveEntry,
    pub alignment: f64,
    pub inputs: Vec<f64>,
}

/// Gyro Router — main interface for token → NAND pipeline
#[derive(Debug, Clone)]
pub struct GyroRouter {
    gyro: GyroState,
    mapping: PrimitiveMapping,
    dynamics: crate::gyro::dynamics::GyroDynamics,
}

impl Default for GyroRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl GyroRouter {
    /// Create a new router with default gyroscope at Aries
    pub fn new() -> Self {
        GyroRouter {
            gyro: GyroState::new(),
            mapping: PrimitiveMapping::default(),
            dynamics: crate::gyro::dynamics::GyroDynamics::default(),
        }
    }

    /// Process a token through the full pipeline
    ///
    /// 1. Classify token → AtomClassification (7 axes)
    /// 2. Determine dominant sign
    /// 3. Apply torque to gyro (token mass = 1.0 by default)
    /// 4. Step gyro forward
    /// 5. Find aligned primitives
    /// 6. Return NAND expression for evaluation
    pub fn process_token(&mut self, token: &str, sorter: &ChangeSorter, mass: f64) -> RouteResult {
        // 1. Classify token
        let classification = sorter.classify_token(token);

        // 2. Get dominant sign
        let sign = classification.dominant_sign().unwrap_or(Sign::Aries);

        // 3. Apply impulse to gyro
        self.dynamics.apply_impulse(&mut self.gyro, sign, mass);

        // 4. Step gyro (1.0 second per token for now)
        self.dynamics.step(&mut self.gyro, 1.0);

        // 5. Find aligned primitives for this sign
        let primitives = self.mapping.for_sign(sign);
        let weights = self.dynamics.alignment(&self.gyro);

        let mut fired = Vec::new();
        for entry in primitives {
            let alignment = weights[sign.index()];
            if alignment > 0.1 {
                fired.push(FiredPrimitive {
                    entry: entry.clone(),
                    alignment,
                    inputs: vec![],
                });
            }
        }

        // 6. Get first aligned primitive's NAND expression
        let nand_expression = fired.first().map(|f| f.entry.expression.clone());

        // Confidence = alignment weight
        let confidence = weights[sign.index()];

        RouteResult {
            sign,
            fired_primitives: fired,
            nand_expression,
            confidence,
        }
    }

    /// Process a query string (multiple tokens)
    pub fn process_query(&mut self, query: &str, sorter: &ChangeSorter) -> Vec<RouteResult> {
        query
            .split_whitespace()
            .filter(|t| !t.is_empty())
            .map(|token| self.process_token(token, sorter, 1.0))
            .collect()
    }

    /// Get current gyro state
    pub fn gyro_state(&self) -> &GyroState {
        &self.gyro
    }

    /// Get mutable gyro state for manual control
    pub fn gyro_state_mut(&mut self) -> &mut GyroState {
        &mut self.gyro
    }

    /// Evaluate a NAND expression with given inputs
    pub fn evaluate(
        &self,
        expr: &NandExpression,
        inputs: &std::collections::HashMap<String, f64>,
    ) -> f64 {
        expr.evaluate(inputs).unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = GyroRouter::new();
        assert_eq!(router.gyro.orientation.0, 0.0);
    }

    #[test]
    fn test_process_token() {
        let mut router = GyroRouter::new();
        let sorter = ChangeSorter::new();
        let result = router.process_token("force", &sorter, 1.0);
        assert!(result.confidence > 0.0);
    }

    #[test]
    fn test_process_query() {
        let mut router = GyroRouter::new();
        let sorter = ChangeSorter::new();
        let results = router.process_query("force mass acceleration", &sorter);
        assert_eq!(results.len(), 3);
    }
}

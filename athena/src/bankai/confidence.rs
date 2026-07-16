//! Confidence semiring — algebraic confidence composition.
//!
//! Provides a formal semiring for confidence values, where:
//! - `combine` (⊕) merges evidence from alternative derivations (max)
//! - `compose` (⊗) chains confidence across sequential dependencies (multiply)
//!
//! This follows real semiring laws: associativity, identity, distributivity.

/// A value that can be combined (⊕) across alternative derivations
/// and composed (⊗) across sequential dependency, following semiring laws:
/// associativity of both ops, identity elements, and ⊗ distributing over ⊕.
pub trait ConfidenceSemiring: Clone + PartialEq {
    /// Identity for ⊕ — "no support for this conclusion yet"
    fn zero() -> Self;
    /// Identity for ⊗ — "fully certain, contributes no discount"
    fn one() -> Self;
    /// Combine confidence from alternative/independent derivations (⊕).
    fn combine(&self, other: &Self) -> Self;
    /// Compose confidence across a dependency chain (⊗).
    fn compose(&self, other: &Self) -> Self;
}

/// Bounded confidence in [0.0, 1.0].
///
/// `compose = multiplication` (independence assumption across steps).
/// `combine = max` (take the best-supported derivation).
///
/// This is a documented simplification. A future version could swap in
/// Dempster-Shafer combination or interval bounds without changing call
/// sites, because they'd all implement the same trait.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct BoundedConfidence(pub f64);

impl ConfidenceSemiring for BoundedConfidence {
    fn zero() -> Self {
        BoundedConfidence(0.0)
    }
    fn one() -> Self {
        BoundedConfidence(1.0)
    }
    fn combine(&self, other: &Self) -> Self {
        BoundedConfidence(self.0.max(other.0))
    }
    fn compose(&self, other: &Self) -> Self {
        BoundedConfidence((self.0 * other.0).clamp(0.0, 1.0))
    }
}

impl From<BoundedConfidence> for f64 {
    fn from(c: BoundedConfidence) -> Self {
        c.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_confidence_identities() {
        let c = BoundedConfidence(0.7);
        assert_eq!(c.compose(&BoundedConfidence::one()), c);
        assert_eq!(c.combine(&BoundedConfidence::zero()), c);
    }

    #[test]
    fn test_bounded_confidence_compose_reduces() {
        let a = BoundedConfidence(0.9);
        let b = BoundedConfidence(0.8);
        let composed = a.compose(&b);
        assert!((composed.0 - 0.72).abs() < 1e-10);
    }

    #[test]
    fn test_bounded_confidence_combine_takes_max() {
        let a = BoundedConfidence(0.9);
        let b = BoundedConfidence(0.5);
        assert_eq!(a.combine(&b), a);
        assert_eq!(b.combine(&a), a);
    }

    #[test]
    fn test_bounded_confidence_clamps() {
        let c = BoundedConfidence(2.0);
        let d = BoundedConfidence(0.5);
        let composed = c.compose(&d);
        assert!(composed.0 <= 1.0);
    }
}

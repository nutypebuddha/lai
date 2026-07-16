//! # NAND Gate — the single primitive operation
//!
//! All logic gates derive from the Sheffer stroke (NAND):
//!
//! ```text
//! nand(a, b) = 1 - a * b
//! ```
//!
//! Inputs are `f64` values interpreted as continuous truth values in [0, 1].
//! For Boolean logic, values should be exactly 0.0 or 1.0 — but the gates
//! work correctly for any value in [0, 1], enabling fuzzy and continuous
//! truth computations.

/// The Sheffer stroke — functionally complete Boolean primitive.
///
/// `nand(a, b) = 1 - a * b`
///
/// For Boolean inputs {0, 1}, the output is:
/// - nand(0, 0) = 1
/// - nand(0, 1) = 1
/// - nand(1, 0) = 1
/// - nand(1, 1) = 0
///
/// For continuous inputs in [0, 1], the output is `1 - a*b`,
/// which smoothly interpolates between Boolean values.
#[inline]
pub fn nand(a: f64, b: f64) -> f64 {
    1.0 - a * b
}

/// NOT: `not(a) = nand(a, a) = 1 - a`
#[inline]
pub fn not(a: f64) -> f64 {
    nand(a, a)
}

/// AND: `and(a, b) = not(nand(a, b)) = a * b`
#[inline]
pub fn and(a: f64, b: f64) -> f64 {
    let n = nand(a, b);
    not(n)
}

/// OR: `or(a, b) = nand(not(a), not(b)) = a + b - a*b`
#[inline]
pub fn or(a: f64, b: f64) -> f64 {
    let na = not(a);
    let nb = not(b);
    nand(na, nb)
}

/// NOR: `nor(a, b) = not(or(a, b)) = 1 - (a + b - a*b)`
#[inline]
pub fn nor(a: f64, b: f64) -> f64 {
    let o = or(a, b);
    not(o)
}

/// XOR: `xor(a, b) = or(and(a, not(b)), and(not(a), b)) = a + b - 2*a*b`
#[inline]
pub fn xor(a: f64, b: f64) -> f64 {
    let nb = not(b);
    let anb = and(a, nb);
    let na = not(a);
    let nab = and(na, b);
    or(anb, nab)
}

/// XNOR: `xnor(a, b) = not(xor(a, b)) = 1 - a - b + 2*a*b`
#[inline]
pub fn xnor(a: f64, b: f64) -> f64 {
    let x = xor(a, b);
    not(x)
}

/// Implies: `implies(a, b) = or(not(a), b) = 1 - a + a*b`
///
/// Logical implication: "if a then b". Only false when a is true and b is false.
#[inline]
pub fn implies(a: f64, b: f64) -> f64 {
    let na = not(a);
    or(na, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nand_identity() {
        // nand(a, 1) = 1 - a  (inverter when one input is 1)
        assert!((nand(0.0, 1.0) - 1.0).abs() < 1e-12);
        assert!((nand(1.0, 1.0) - 0.0).abs() < 1e-12);

        // nand(a, 0) = 1 (constant 1 when one input is 0)
        assert!((nand(0.0, 0.0) - 1.0).abs() < 1e-12);
        assert!((nand(1.0, 0.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_not_involution() {
        // not(not(a)) = a
        assert!((not(not(0.0)) - 0.0).abs() < 1e-12);
        assert!((not(not(1.0)) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_and_idempotent() {
        // a AND a = a
        assert!((and(0.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((and(1.0, 1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_or_idempotent() {
        // a OR a = a
        assert!((or(0.0, 0.0) - 0.0).abs() < 1e-12);
        assert!((or(1.0, 1.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_xor_commutative() {
        // xor(a, b) = xor(b, a)
        assert!((xor(0.3, 0.7) - xor(0.7, 0.3)).abs() < 1e-12);
    }

    #[test]
    fn test_de_morgan_boolean() {
        // De Morgan's laws hold exactly for Boolean values {0, 1}
        // (not(a AND b) = not(a) OR not(b))
        // Not guaranteed for continuous values because NAND-based gates
        // extend non-linearly outside {0, 1}.
        for a in [0.0, 1.0] {
            for b in [0.0, 1.0] {
                let lhs = not(and(a, b));
                let rhs = or(not(a), not(b));
                assert!(
                    (lhs - rhs).abs() < 1e-12,
                    "De Morgan LHS: {lhs}, RHS: {rhs} for a={a}, b={b}"
                );

                let lhs2 = not(or(a, b));
                let rhs2 = and(not(a), not(b));
                assert!(
                    (lhs2 - rhs2).abs() < 1e-12,
                    "De Morgan LHS2: {lhs2}, RHS2: {rhs2} for a={a}, b={b}"
                );
            }
        }
    }

    #[test]
    fn test_implies_definition() {
        // a → b  =  not(a) OR b
        for a in [0.0, 0.5, 1.0] {
            for b in [0.0, 0.5, 1.0] {
                let direct = implies(a, b);
                let derived = or(not(a), b);
                assert!(
                    (direct - derived).abs() < 1e-12,
                    "implies({a}, {b}) = {direct} != {derived}"
                );
            }
        }
    }

    #[test]
    fn test_all_gates_from_nand_only() {
        // Verify all gates produce the same result as their NAND-only implementations
        let vals: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];
        for a in vals {
            for b in vals {
                // NOT: should equal nand(a, a)
                assert!((not(a) - nand(a, a)).abs() < 1e-12);

                // AND: should equal not(nand(a, b))
                assert!((and(a, b) - not(nand(a, b))).abs() < 1e-12);

                // OR: should equal nand(not(a), not(b))
                assert!((or(a, b) - nand(not(a), not(b))).abs() < 1e-12);
            }
        }
    }
}

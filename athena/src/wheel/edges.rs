//! # Edges — aspects and relationships between domains
//!
//! Each edge on the wheel has an *aspect* that defines the nature of the relationship
//! between two domains. Aspects determine how formulas compose across domains.
//!
//! # Vedic Wheel (9 Grahas)
//!
//! The wheel consists of 9 Vedic grahas at 40° intervals:
//!
//! | Steps | Arc    | Aspect (Western) | Aspect (Vedic)    |
//! |-------|--------|-------------------|-------------------|
//! | 0     | 0°     | Conjunction       | Yuti              |
//! | 1     | 40°    | Sextile           | Sama (adjacent)   |
//! | 2     | 80°    | Square            | Varga (division)  |
//! | 3     | 120°   | Trine             | Trikona           |
//! | 4     | 160°   | Opposition        | Saptama (full)    |

use serde::{Deserialize, Serialize};
use std::fmt;

use super::Domain;

/// The aspect (relationship type) between two domains on the wheel.
///
/// Aspects are derived from the arc distance between two nodes on the 9-node wheel.
/// They define how naturally formulas in one domain compose with formulas in another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aspect {
    /// Same domain — trivial composition.
    /// Arc distance: 0.
    /// Surya → Surya (internal formula chain).
    Conjunction,

    /// Adjacent domains — direct flow, sequential reasoning.
    /// Arc distance: 1 step (40°).
    /// Surya → Chandra, Ketu → Surya.
    Sextile,

    /// Harmonious domains — formulas naturally compose across these domains.
    /// Arc distance: 3 steps (120°).
    /// Surya → Budha, Surya → Shani.
    Trine,

    /// Tension between domains — requires conversion/translation.
    /// Arc distance: 2 steps (80°).
    /// Surya → Mangala, Chandra → Budha.
    Square,

    /// Complementary inverses — opposing but completing domains.
    /// Arc distance: 4 steps (160°) — closest to 180° on a 9-node wheel.
    /// Surya → Brihaspati, Chandra → Shukra.
    Opposition,
}

/// Precomputed 9×9 lookup table for `Aspect::between(a, b)`.
/// Indexed by `[a.index()][b.index()]`, computed at compile time via const fn.
/// Provides O(1) aspect lookup instead of arithmetic with branching.
///
/// On a 9-node wheel (40° per step):
/// - 0 steps → Conjunction
/// - 1 step  → Sextile (adjacent)
/// - 2 steps → Square (Varga/tension)
/// - 3 steps → Trine (Trikona/harmonious)
/// - 4 steps → Opposition (Saptama/full aspect)
const ASPECT_TABLE: [[Aspect; 9]; 9] = {
    let mut table = [[Aspect::Conjunction; 9]; 9];
    let mut i: usize = 0;
    while i < 9 {
        let mut j: usize = 0;
        while j < 9 {
            let diff = i.abs_diff(j);
            let min_diff = if diff < 9 - diff { diff } else { 9 - diff };
            table[i][j] = match min_diff {
                0 => Aspect::Conjunction,
                1 => Aspect::Sextile,
                2 => Aspect::Square,
                3 => Aspect::Trine,
                4 => Aspect::Opposition,
                _ => Aspect::Conjunction, // unreachable for 0..=4, but const fn requires exhaustive
            };
            j += 1;
        }
        i += 1;
    }
    table
};

impl Aspect {
    /// Determine the aspect between two domains on the wheel.
    ///
    /// Uses a precomputed 9×9 lookup table for O(1) constant-time access.
    #[inline]
    pub fn between(a: Domain, b: Domain) -> Aspect {
        ASPECT_TABLE[a.index()][b.index()]
    }

    /// The arc distance (minimum steps) between the two domains.
    #[inline]
    pub fn arc_distance(self) -> usize {
        match self {
            Aspect::Conjunction => 0,
            Aspect::Sextile => 1,
            Aspect::Trine => 3,
            Aspect::Square => 2,
            Aspect::Opposition => 4,
        }
    }

    /// Whether formulas compose directly across this aspect.
    /// Conjunction, Trine, and Sextile are direct; Square and Opposition require mediation.
    #[inline]
    pub fn is_direct(self) -> bool {
        matches!(self, Aspect::Conjunction | Aspect::Sextile | Aspect::Trine)
    }

    /// Base confidence for this aspect relationship.
    ///
    /// Used by composition confidence, entity relationship display, and anywhere
    /// an aspect needs a scalar confidence value. Single source of truth.
    #[inline]
    pub fn confidence(self) -> f64 {
        match self {
            Aspect::Conjunction => 1.00,
            Aspect::Sextile => 0.95,
            Aspect::Trine => 0.90,
            Aspect::Square => 0.75,
            Aspect::Opposition => 0.60,
        }
    }

    /// Whether this aspect represents tension or inversion.
    #[inline]
    pub fn is_tension(self) -> bool {
        matches!(self, Aspect::Square | Aspect::Opposition)
    }
}

/// A typed relationship between two domains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub from: Domain,
    pub to: Domain,
    pub aspect: Aspect,
    /// Arc distance in steps (1–6).
    pub distance: usize,
    /// Whether this is a forward or reverse traversal.
    pub direction: Direction,
}

/// The direction of traversal along the edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Forward,
    Reverse,
}

impl Relationship {
    /// Create a relationship between two domains.
    pub fn new(from: Domain, to: Domain) -> Self {
        let aspect = Aspect::between(from, to);
        let distance = aspect.arc_distance();
        let (from_idx, to_idx) = (from.index(), to.index());
        let forward = ((to_idx as isize - from_idx as isize).rem_euclid(9)) <= 4;
        let direction = if forward {
            Direction::Forward
        } else {
            Direction::Reverse
        };
        Relationship {
            from,
            to,
            aspect,
            distance,
            direction,
        }
    }

    /// Reverse the direction of this relationship.
    pub fn reverse(&self) -> Self {
        Relationship {
            from: self.to,
            to: self.from,
            aspect: self.aspect,
            distance: self.distance,
            direction: match self.direction {
                Direction::Forward => Direction::Reverse,
                Direction::Reverse => Direction::Forward,
            },
        }
    }
}

impl fmt::Display for Aspect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Aspect::Conjunction => write!(f, "Conjunction (☌)"),
            Aspect::Sextile => write!(f, "Sextile (⚹)"),
            Aspect::Trine => write!(f, "Trine (△)"),
            Aspect::Square => write!(f, "Square (□)"),
            Aspect::Opposition => write!(f, "Opposition (☍)"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_between_self() {
        assert_eq!(
            Aspect::between(Domain::Mangala, Domain::Mangala),
            Aspect::Conjunction
        );
    }

    #[test]
    fn test_aspect_adjacent() {
        // Surya (0) and Chandra (1) are adjacent (1 step)
        assert_eq!(
            Aspect::between(Domain::Surya, Domain::Chandra),
            Aspect::Sextile
        );
        // Ketu (8) and Surya (0) wrap around — adjacent (1 step)
        assert_eq!(
            Aspect::between(Domain::Ketu, Domain::Surya),
            Aspect::Sextile
        );
    }

    #[test]
    fn test_aspect_opposition() {
        // On 9-node wheel, opposition is 4 steps (160°)
        // Surya (0) → Brihaspati (4) = 4 steps
        assert_eq!(
            Aspect::between(Domain::Surya, Domain::Brihaspati),
            Aspect::Opposition
        );
        assert_eq!(
            Aspect::between(Domain::Brihaspati, Domain::Surya),
            Aspect::Opposition
        );
    }

    #[test]
    fn test_aspect_trine() {
        // On 9-node wheel, trine is 3 steps (120°)
        // Surya (0) → Budha (3) = 3 steps
        let a = Domain::Surya;
        assert_eq!(Aspect::between(a, Domain::Budha), Aspect::Trine);
        // Surya (0) → Shani (6) = 3 steps (wrapping: 9-6=3)
        assert_eq!(Aspect::between(a, Domain::Shani), Aspect::Trine);
    }

    #[test]
    fn test_aspect_square() {
        // On 9-node wheel, square is 2 steps (80°)
        // Surya (0) → Mangala (2) = 2 steps
        let a = Domain::Surya;
        assert_eq!(Aspect::between(a, Domain::Mangala), Aspect::Square);
        // Surya (0) → Rahu (7) = 2 steps (wrapping: 9-7=2)
        assert_eq!(Aspect::between(a, Domain::Rahu), Aspect::Square);
    }

    #[test]
    fn test_relationship_creation() {
        // Surya (0) and Chandra (1) are adjacent (sextile)
        let r = Relationship::new(Domain::Surya, Domain::Chandra);
        assert_eq!(r.aspect, Aspect::Sextile);
        assert_eq!(r.distance, 1);
    }

    #[test]
    fn test_direct_and_tension() {
        assert!(Aspect::Conjunction.is_direct());
        assert!(Aspect::Sextile.is_direct());
        assert!(!Aspect::Square.is_direct());
        assert!(!Aspect::Opposition.is_direct());
        assert!(Aspect::Square.is_tension());
        assert!(Aspect::Opposition.is_tension());
    }
}

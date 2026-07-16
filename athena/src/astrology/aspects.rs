//! # Aspects — dynamic angular relationships
//!
//! Aspects represent the angular relationship between two points on the
//! zodiac wheel. In the gyroscopic engine, aspects are computed dynamically
//! based on the wheel's current orientation, not from a static table.
//!
//! This module provides the static Ptolemaic aspect definitions as a
//! baseline. The dynamic gyroscopic computation lives in `src/gyro/`.

use serde::{Deserialize, Serialize};

/// The 5 Ptolemaic aspects (major aspects in Western astrology).
///
/// Each aspect has an angular separation on the wheel and a quality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aspect {
    /// 0° separation — identity, unity, fusion
    Conjunction = 0,
    /// 30° or 60° — opportunity, flow, ease
    Sextile = 1,
    /// 120° — harmony, talent, luck
    Trine = 2,
    /// 90° — tension, challenge, growth
    Square = 3,
    /// 180° — polarity, awareness, balance
    Opposition = 4,
}

impl Aspect {
    /// Number of aspect types.
    pub const COUNT: usize = 5;

    /// Get the index of this aspect.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Create an Aspect from a 0-based index.
    pub fn from_index(i: usize) -> Self {
        match i % 5 {
            0 => Aspect::Conjunction,
            1 => Aspect::Sextile,
            2 => Aspect::Trine,
            3 => Aspect::Square,
            4 => Aspect::Opposition,
            _ => unreachable!(),
        }
    }

    /// The arc distance on the wheel (in zodiac steps, 30° each).
    pub fn arc_distance(self) -> usize {
        match self {
            Aspect::Conjunction => 0,
            Aspect::Sextile => 1,
            Aspect::Trine => 4,
            Aspect::Square => 3,
            Aspect::Opposition => 6,
        }
    }

    /// The quality or nature of this aspect.
    pub fn quality(self) -> &'static str {
        match self {
            Aspect::Conjunction => "Unity, fusion, intensification",
            Aspect::Sextile => "Opportunity, flow, cooperation",
            Aspect::Trine => "Harmony, talent, effortless expression",
            Aspect::Square => "Tension, challenge, motivation for growth",
            Aspect::Opposition => "Polarity, awareness, balance through integration",
        }
    }

    /// Whether this aspect is considered "hard" (challenging).
    pub fn is_hard(self) -> bool {
        matches!(self, Aspect::Square | Aspect::Opposition)
    }

    /// Whether this aspect is considered "soft" (harmonious).
    pub fn is_soft(self) -> bool {
        matches!(self, Aspect::Sextile | Aspect::Trine)
    }

    /// Whether this aspect is neutral.
    pub fn is_neutral(self) -> bool {
        matches!(self, Aspect::Conjunction)
    }

    /// Unicode symbol for this aspect.
    pub fn symbol(self) -> &'static str {
        match self {
            Aspect::Conjunction => "☌",
            Aspect::Sextile => "⚹",
            Aspect::Trine => "△",
            Aspect::Square => "□",
            Aspect::Opposition => "☍",
        }
    }

    /// All aspects as an array.
    pub fn all() -> [Aspect; 5] {
        [
            Aspect::Conjunction,
            Aspect::Sextile,
            Aspect::Trine,
            Aspect::Square,
            Aspect::Opposition,
        ]
    }

    /// Compute the static aspect between two zodiac sign indices (0–11).
    /// Uses the Ptolemaic system: arc distances 0, 1, 3, 4, 6.
    ///
    /// Returns `None` for non-aspect distances (2, 5).
    pub fn between_sign_indices(a: usize, b: usize) -> Option<Aspect> {
        let diff = a.abs_diff(b);
        let dist = diff.min(12 - diff);
        match dist {
            0 => Some(Aspect::Conjunction),
            1 => Some(Aspect::Sextile),
            3 => Some(Aspect::Square),
            4 => Some(Aspect::Trine),
            6 => Some(Aspect::Opposition),
            _ => None,
        }
    }

    /// Compute the arc distance between two sign indices (0–11).
    pub fn arc_distance_between(a: usize, b: usize) -> usize {
        let diff = a.abs_diff(b);
        diff.min(12 - diff)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_index_roundtrip() {
        for i in 0..5 {
            let a = Aspect::from_index(i);
            assert_eq!(a.index(), i);
        }
    }

    #[test]
    fn test_aspect_arc_distances() {
        assert_eq!(Aspect::Conjunction.arc_distance(), 0);
        assert_eq!(Aspect::Sextile.arc_distance(), 1);
        assert_eq!(Aspect::Square.arc_distance(), 3);
        assert_eq!(Aspect::Trine.arc_distance(), 4);
        assert_eq!(Aspect::Opposition.arc_distance(), 6);
    }

    #[test]
    fn test_between_sign_indices() {
        // Aries (0) and Aries (0) — conjunction
        assert_eq!(
            Aspect::between_sign_indices(0, 0),
            Some(Aspect::Conjunction)
        );
        // Aries (0) and Taurus (1) — sextile
        assert_eq!(Aspect::between_sign_indices(0, 1), Some(Aspect::Sextile));
        // Aries (0) and Cancer (3) — square
        assert_eq!(Aspect::between_sign_indices(0, 3), Some(Aspect::Square));
        // Aries (0) and Leo (4) — trine
        assert_eq!(Aspect::between_sign_indices(0, 4), Some(Aspect::Trine));
        // Aries (0) and Libra (6) — opposition
        assert_eq!(Aspect::between_sign_indices(0, 6), Some(Aspect::Opposition));
        // Aries (0) and Gemini (2) — no aspect (distance 2)
        assert_eq!(Aspect::between_sign_indices(0, 2), None);
        // Aries (0) and Virgo (5) — no aspect (distance 5)
        assert_eq!(Aspect::between_sign_indices(0, 5), None);
    }

    #[test]
    fn test_aspect_qualities() {
        assert!(Aspect::Square.is_hard());
        assert!(Aspect::Opposition.is_hard());
        assert!(!Aspect::Trine.is_hard());
        assert!(Aspect::Trine.is_soft());
        assert!(Aspect::Sextile.is_soft());
        assert!(Aspect::Conjunction.is_neutral());
    }

    #[test]
    fn test_aspect_symbols() {
        for a in Aspect::all() {
            assert!(!a.symbol().is_empty());
        }
    }

    #[test]
    fn test_arc_distance_between() {
        assert_eq!(Aspect::arc_distance_between(0, 0), 0);
        assert_eq!(Aspect::arc_distance_between(0, 1), 1);
        assert_eq!(Aspect::arc_distance_between(0, 6), 6);
        assert_eq!(Aspect::arc_distance_between(0, 11), 1); // Pisces, adjacent to Aries
    }
}

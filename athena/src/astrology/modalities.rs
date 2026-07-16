//! # Modalities — the 3 modes of expression
//!
//! Cardinal (initiation), Fixed (stabilization), Mutable (adaptation).
//! Each modality groups 4 zodiac signs into quadruplicities.

use serde::{Deserialize, Serialize};

/// The 3 modalities (also called quadruplicities).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modality {
    Cardinal = 0,
    Fixed = 1,
    Mutable = 2,
}

impl Modality {
    /// Number of modalities.
    pub const COUNT: usize = 3;

    /// Get the index of this modality.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Create a Modality from a 0-based index.
    pub fn from_index(i: usize) -> Self {
        match i % 3 {
            0 => Modality::Cardinal,
            1 => Modality::Fixed,
            2 => Modality::Mutable,
            _ => unreachable!(),
        }
    }

    /// Unicode symbol for this modality.
    pub fn symbol(self) -> &'static str {
        match self {
            Modality::Cardinal => "🅲",
            Modality::Fixed => "🅵",
            Modality::Mutable => "🅼",
        }
    }

    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Modality::Cardinal => "Cardinal",
            Modality::Fixed => "Fixed",
            Modality::Mutable => "Mutable",
        }
    }

    /// Core quality description.
    pub fn quality(self) -> &'static str {
        match self {
            Modality::Cardinal => "Initiation, leadership",
            Modality::Fixed => "Stabilization, endurance",
            Modality::Mutable => "Adaptation, flexibility",
        }
    }

    /// All 3 modalities as an array.
    pub fn all() -> [Modality; 3] {
        [Modality::Cardinal, Modality::Fixed, Modality::Mutable]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modality_index_roundtrip() {
        for i in 0..3 {
            let m = Modality::from_index(i);
            assert_eq!(m.index(), i);
        }
    }

    #[test]
    fn test_modality_has_symbol() {
        for m in Modality::all() {
            assert!(!m.symbol().is_empty());
        }
    }

    #[test]
    fn test_modality_names() {
        assert_eq!(Modality::Cardinal.name(), "Cardinal");
        assert_eq!(Modality::Fixed.name(), "Fixed");
        assert_eq!(Modality::Mutable.name(), "Mutable");
    }
}

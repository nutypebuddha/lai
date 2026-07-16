//! # Elements — the 4 classical elements
//!
//! Fire, Earth, Air, Water — the fundamental temperaments that group
//! the 12 zodiac signs into triplicities.

use serde::{Deserialize, Serialize};

/// The 4 classical elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Fire = 0,
    Earth = 1,
    Air = 2,
    Water = 3,
}

impl Element {
    /// Number of elements.
    pub const COUNT: usize = 4;

    /// Get the index of this element.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Create an Element from a 0-based index.
    pub fn from_index(i: usize) -> Self {
        match i % 4 {
            0 => Element::Fire,
            1 => Element::Earth,
            2 => Element::Air,
            3 => Element::Water,
            _ => unreachable!(),
        }
    }

    /// Unicode symbol for this element.
    pub fn symbol(self) -> &'static str {
        match self {
            Element::Fire => "🔥",
            Element::Earth => "🜁",
            Element::Air => "🜄",
            Element::Water => "🜂",
        }
    }

    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Element::Fire => "Fire",
            Element::Earth => "Earth",
            Element::Air => "Air",
            Element::Water => "Water",
        }
    }

    /// Core quality description.
    pub fn quality(self) -> &'static str {
        match self {
            Element::Fire => "Action, transformation",
            Element::Earth => "Substance, structure",
            Element::Air => "Relation, information",
            Element::Water => "Emotion, integration",
        }
    }

    /// All 4 elements as an array.
    pub fn all() -> [Element; 4] {
        [Element::Fire, Element::Earth, Element::Air, Element::Water]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element_index_roundtrip() {
        for i in 0..4 {
            let elem = Element::from_index(i);
            assert_eq!(elem.index(), i);
        }
    }

    #[test]
    fn test_element_has_symbol() {
        for elem in Element::all() {
            assert!(!elem.symbol().is_empty());
        }
    }

    #[test]
    fn test_element_has_name() {
        assert_eq!(Element::Fire.name(), "Fire");
        assert_eq!(Element::Earth.name(), "Earth");
        assert_eq!(Element::Air.name(), "Air");
        assert_eq!(Element::Water.name(), "Water");
    }

    #[test]
    fn test_element_has_quality() {
        assert!(!Element::Fire.quality().is_empty());
    }
}

//! # Houses — the 12 astrological houses
//!
//! The houses represent different areas of life experience. Each house
//! has a natural sign association and a domain focus.

use serde::{Deserialize, Serialize};

/// The 12 astrological houses (1st through 12th).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum House {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
    Ninth = 8,
    Tenth = 9,
    Eleventh = 10,
    Twelfth = 11,
}

impl House {
    /// Number of houses.
    pub const COUNT: usize = 12;

    /// Get the index of this house (0-based, 1st = 0).
    pub fn index(self) -> usize {
        self as usize
    }

    /// Create a House from a 0-based index.
    pub fn from_index(i: usize) -> Self {
        match i % 12 {
            0 => House::First,
            1 => House::Second,
            2 => House::Third,
            3 => House::Fourth,
            4 => House::Fifth,
            5 => House::Sixth,
            6 => House::Seventh,
            7 => House::Eighth,
            8 => House::Ninth,
            9 => House::Tenth,
            10 => House::Eleventh,
            11 => House::Twelfth,
            _ => unreachable!(),
        }
    }

    /// Get the house number (1-based, for display).
    pub fn number(self) -> usize {
        self.index() + 1
    }

    /// The natural sign association for this house.
    pub fn natural_sign(self) -> crate::astrology::Sign {
        use crate::astrology::Sign;
        match self {
            House::First => Sign::Aries,
            House::Second => Sign::Taurus,
            House::Third => Sign::Gemini,
            House::Fourth => Sign::Cancer,
            House::Fifth => Sign::Leo,
            House::Sixth => Sign::Virgo,
            House::Seventh => Sign::Libra,
            House::Eighth => Sign::Scorpio,
            House::Ninth => Sign::Sagittarius,
            House::Tenth => Sign::Capricorn,
            House::Eleventh => Sign::Aquarius,
            House::Twelfth => Sign::Pisces,
        }
    }

    /// Area of life represented by this house.
    pub fn domain(self) -> &'static str {
        match self {
            House::First => "Self, identity, appearance",
            House::Second => "Values, possessions, finances",
            House::Third => "Communication, learning, siblings",
            House::Fourth => "Home, family, roots",
            House::Fifth => "Creativity, romance, children",
            House::Sixth => "Health, work, service",
            House::Seventh => "Partnerships, marriage, contracts",
            House::Eighth => "Transformation, shared resources, depth",
            House::Ninth => "Philosophy, travel, higher learning",
            House::Tenth => "Career, reputation, public life",
            House::Eleventh => "Friendships, community, aspirations",
            House::Twelfth => "Subconscious, solitude, spirituality",
        }
    }

    /// All 12 houses as an array.
    pub fn all() -> [House; 12] {
        [
            House::First,
            House::Second,
            House::Third,
            House::Fourth,
            House::Fifth,
            House::Sixth,
            House::Seventh,
            House::Eighth,
            House::Ninth,
            House::Tenth,
            House::Eleventh,
            House::Twelfth,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_house_index_roundtrip() {
        for i in 0..12 {
            let h = House::from_index(i);
            assert_eq!(h.index(), i);
        }
    }

    #[test]
    fn test_house_number() {
        assert_eq!(House::First.number(), 1);
        assert_eq!(House::Twelfth.number(), 12);
    }

    #[test]
    fn test_house_natural_sign() {
        assert_eq!(House::First.natural_sign(), crate::astrology::Sign::Aries);
        assert_eq!(House::Seventh.natural_sign(), crate::astrology::Sign::Libra);
    }

    #[test]
    fn test_house_has_domain() {
        for h in House::all() {
            assert!(!h.domain().is_empty());
        }
    }

    #[test]
    fn test_twelve_houses() {
        assert_eq!(House::all().len(), 12);
    }
}

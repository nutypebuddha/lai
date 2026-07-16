//! # Planetary Rulers — the 7 classical planets
//!
//! Each zodiac sign is ruled by a planet (or luminary). The 7 classical
//! planets (Sun through Saturn) govern the 12 signs, with some planets
//! ruling two signs.

use serde::{Deserialize, Serialize};

/// The 7 classical planetary rulers (including Sun and Moon as luminaries).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetaryRuler {
    Sun = 0,
    Moon = 1,
    Mercury = 2,
    Venus = 3,
    Mars = 4,
    Jupiter = 5,
    Saturn = 6,
}

impl PlanetaryRuler {
    /// Number of rulers.
    pub const COUNT: usize = 7;

    /// Get the index of this ruler.
    pub fn index(self) -> usize {
        self as usize
    }

    /// Create a PlanetaryRuler from a 0-based index.
    pub fn from_index(i: usize) -> Self {
        match i % 7 {
            0 => PlanetaryRuler::Sun,
            1 => PlanetaryRuler::Moon,
            2 => PlanetaryRuler::Mercury,
            3 => PlanetaryRuler::Venus,
            4 => PlanetaryRuler::Mars,
            5 => PlanetaryRuler::Jupiter,
            6 => PlanetaryRuler::Saturn,
            _ => unreachable!(),
        }
    }

    /// Astrological symbol for this planet.
    pub fn symbol(self) -> &'static str {
        match self {
            PlanetaryRuler::Sun => "☉",
            PlanetaryRuler::Moon => "☽",
            PlanetaryRuler::Mercury => "☿",
            PlanetaryRuler::Venus => "♀",
            PlanetaryRuler::Mars => "♂",
            PlanetaryRuler::Jupiter => "♃",
            PlanetaryRuler::Saturn => "♄",
        }
    }

    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            PlanetaryRuler::Sun => "Sun",
            PlanetaryRuler::Moon => "Moon",
            PlanetaryRuler::Mercury => "Mercury",
            PlanetaryRuler::Venus => "Venus",
            PlanetaryRuler::Mars => "Mars",
            PlanetaryRuler::Jupiter => "Jupiter",
            PlanetaryRuler::Saturn => "Saturn",
        }
    }

    /// The signs this planet rules (domicile).
    pub fn domicile_signs(self) -> &'static [crate::astrology::Sign] {
        use crate::astrology::Sign;
        match self {
            PlanetaryRuler::Sun => &[Sign::Leo],
            PlanetaryRuler::Moon => &[Sign::Cancer],
            PlanetaryRuler::Mercury => &[Sign::Gemini, Sign::Virgo],
            PlanetaryRuler::Venus => &[Sign::Taurus, Sign::Libra],
            PlanetaryRuler::Mars => &[Sign::Aries, Sign::Scorpio],
            PlanetaryRuler::Jupiter => &[Sign::Sagittarius, Sign::Pisces],
            PlanetaryRuler::Saturn => &[Sign::Capricorn, Sign::Aquarius],
        }
    }

    /// All 7 rulers as an array.
    pub fn all() -> [PlanetaryRuler; 7] {
        [
            PlanetaryRuler::Sun,
            PlanetaryRuler::Moon,
            PlanetaryRuler::Mercury,
            PlanetaryRuler::Venus,
            PlanetaryRuler::Mars,
            PlanetaryRuler::Jupiter,
            PlanetaryRuler::Saturn,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ruler_index_roundtrip() {
        for i in 0..7 {
            let r = PlanetaryRuler::from_index(i);
            assert_eq!(r.index(), i);
        }
    }

    #[test]
    fn test_ruler_has_symbol() {
        for r in PlanetaryRuler::all() {
            assert!(!r.symbol().is_empty());
        }
    }

    #[test]
    fn test_ruler_names() {
        assert_eq!(PlanetaryRuler::Sun.name(), "Sun");
        assert_eq!(PlanetaryRuler::Moon.name(), "Moon");
        assert_eq!(PlanetaryRuler::Mercury.name(), "Mercury");
        assert_eq!(PlanetaryRuler::Venus.name(), "Venus");
        assert_eq!(PlanetaryRuler::Mars.name(), "Mars");
        assert_eq!(PlanetaryRuler::Jupiter.name(), "Jupiter");
        assert_eq!(PlanetaryRuler::Saturn.name(), "Saturn");
    }

    #[test]
    fn test_mars_rules_aries_and_scorpio() {
        let signs = PlanetaryRuler::Mars.domicile_signs();
        assert!(signs.contains(&crate::astrology::Sign::Aries));
        assert!(signs.contains(&crate::astrology::Sign::Scorpio));
        assert_eq!(signs.len(), 2);
    }

    #[test]
    fn test_sun_rules_leo_only() {
        let signs = PlanetaryRuler::Sun.domicile_signs();
        assert_eq!(signs, &[crate::astrology::Sign::Leo]);
    }
}

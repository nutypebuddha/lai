//! # Zodiac Signs — the 12 astrological archetypes
//!
//! Each sign maps to a domain on the zodiac wheel, with associated
//! element, modality, planetary ruler, house, and polarity.

use serde::{Deserialize, Serialize};

/// The 12 zodiac signs, ordered from Aries (0) to Pisces (11).
///
/// Each sign represents a domain of knowledge on the wheel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

impl Sign {
    /// Number of signs.
    pub const COUNT: usize = 12;

    /// Get the index of this sign (0-based, Aries = 0).
    pub fn index(self) -> usize {
        self as usize
    }

    /// Create a Sign from a 0-based index.
    pub fn from_index(i: usize) -> Self {
        match i % 12 {
            0 => Sign::Aries,
            1 => Sign::Taurus,
            2 => Sign::Gemini,
            3 => Sign::Cancer,
            4 => Sign::Leo,
            5 => Sign::Virgo,
            6 => Sign::Libra,
            7 => Sign::Scorpio,
            8 => Sign::Sagittarius,
            9 => Sign::Capricorn,
            10 => Sign::Aquarius,
            11 => Sign::Pisces,
            _ => unreachable!(),
        }
    }

    /// Unicode zodiac symbol for this sign.
    pub fn symbol(self) -> &'static str {
        match self {
            Sign::Aries => "♈",
            Sign::Taurus => "♉",
            Sign::Gemini => "♊",
            Sign::Cancer => "♋",
            Sign::Leo => "♌",
            Sign::Virgo => "♍",
            Sign::Libra => "♎",
            Sign::Scorpio => "♏",
            Sign::Sagittarius => "♐",
            Sign::Capricorn => "♑",
            Sign::Aquarius => "♒",
            Sign::Pisces => "♓",
        }
    }

    /// The element associated with this sign.
    pub fn element(self) -> super::Element {
        use super::Element;
        match self {
            Sign::Aries | Sign::Leo | Sign::Sagittarius => Element::Fire,
            Sign::Taurus | Sign::Virgo | Sign::Capricorn => Element::Earth,
            Sign::Gemini | Sign::Libra | Sign::Aquarius => Element::Air,
            Sign::Cancer | Sign::Scorpio | Sign::Pisces => Element::Water,
        }
    }

    /// The modality associated with this sign.
    pub fn modality(self) -> super::Modality {
        use super::Modality;
        match self {
            Sign::Aries | Sign::Cancer | Sign::Libra | Sign::Capricorn => Modality::Cardinal,
            Sign::Taurus | Sign::Leo | Sign::Scorpio | Sign::Aquarius => Modality::Fixed,
            Sign::Gemini | Sign::Virgo | Sign::Sagittarius | Sign::Pisces => Modality::Mutable,
        }
    }

    /// The planetary ruler of this sign.
    pub fn ruler(self) -> super::PlanetaryRuler {
        use super::PlanetaryRuler;
        match self {
            Sign::Aries => PlanetaryRuler::Mars,
            Sign::Taurus => PlanetaryRuler::Venus,
            Sign::Gemini => PlanetaryRuler::Mercury,
            Sign::Cancer => PlanetaryRuler::Moon,
            Sign::Leo => PlanetaryRuler::Sun,
            Sign::Virgo => PlanetaryRuler::Mercury,
            Sign::Libra => PlanetaryRuler::Venus,
            Sign::Scorpio => PlanetaryRuler::Mars,
            Sign::Sagittarius => PlanetaryRuler::Jupiter,
            Sign::Capricorn => PlanetaryRuler::Saturn,
            Sign::Aquarius => PlanetaryRuler::Saturn,
            Sign::Pisces => PlanetaryRuler::Jupiter,
        }
    }

    /// The polarity (yang/yin) of this sign.
    pub fn polarity(self) -> f64 {
        match self {
            Sign::Aries
            | Sign::Gemini
            | Sign::Leo
            | Sign::Libra
            | Sign::Sagittarius
            | Sign::Aquarius => 0.0, // Yang
            Sign::Taurus
            | Sign::Cancer
            | Sign::Virgo
            | Sign::Scorpio
            | Sign::Capricorn
            | Sign::Pisces => 1.0, // Yin
        }
    }

    /// All 12 signs as an array.
    pub fn all() -> [Sign; 12] {
        [
            Sign::Aries,
            Sign::Taurus,
            Sign::Gemini,
            Sign::Cancer,
            Sign::Leo,
            Sign::Virgo,
            Sign::Libra,
            Sign::Scorpio,
            Sign::Sagittarius,
            Sign::Capricorn,
            Sign::Aquarius,
            Sign::Pisces,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_index_roundtrip() {
        for i in 0..12 {
            let sign = Sign::from_index(i);
            assert_eq!(sign.index(), i);
        }
    }

    #[test]
    fn test_sign_has_symbol() {
        for sign in Sign::all() {
            assert!(!sign.symbol().is_empty());
        }
    }

    #[test]
    fn test_aries_properties() {
        assert_eq!(Sign::Aries.element().symbol(), "🔥");
        assert_eq!(Sign::Aries.modality().symbol(), "🅲");
        assert_eq!(Sign::Aries.ruler().symbol(), "♂");
        assert!((Sign::Aries.polarity() - 0.0).abs() < 1e-12); // yang
    }

    #[test]
    fn test_taurus_properties() {
        assert_eq!(Sign::Taurus.element().symbol(), "🜁");
        assert_eq!(Sign::Taurus.modality().symbol(), "🅵");
        assert_eq!(Sign::Taurus.ruler().symbol(), "♀");
        assert!((Sign::Taurus.polarity() - 1.0).abs() < 1e-12); // yin
    }

    #[test]
    fn test_sign_element_groupings() {
        // Fire signs
        for sign in [Sign::Aries, Sign::Leo, Sign::Sagittarius] {
            assert_eq!(sign.element(), crate::astrology::Element::Fire);
        }
        // Earth signs
        for sign in [Sign::Taurus, Sign::Virgo, Sign::Capricorn] {
            assert_eq!(sign.element(), crate::astrology::Element::Earth);
        }
        // Air signs
        for sign in [Sign::Gemini, Sign::Libra, Sign::Aquarius] {
            assert_eq!(sign.element(), crate::astrology::Element::Air);
        }
        // Water signs
        for sign in [Sign::Cancer, Sign::Scorpio, Sign::Pisces] {
            assert_eq!(sign.element(), crate::astrology::Element::Water);
        }
    }

    #[test]
    fn test_sign_modality_groupings() {
        for sign in [Sign::Aries, Sign::Cancer, Sign::Libra, Sign::Capricorn] {
            assert_eq!(sign.modality(), crate::astrology::Modality::Cardinal);
        }
        for sign in [Sign::Taurus, Sign::Leo, Sign::Scorpio, Sign::Aquarius] {
            assert_eq!(sign.modality(), crate::astrology::Modality::Fixed);
        }
        for sign in [Sign::Gemini, Sign::Virgo, Sign::Sagittarius, Sign::Pisces] {
            assert_eq!(sign.modality(), crate::astrology::Modality::Mutable);
        }
    }

    #[test]
    fn test_twelve_signs() {
        assert_eq!(Sign::all().len(), 12);
    }
}

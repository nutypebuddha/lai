//! # K-12 Spiral Understanding Axis
//!
//! Each domain has its own K-12 progression track, repeated at increasing depth cycles.
//!
//! ## Structure
//!
//! | Concept | Meaning |
//! |---------|---------|
//! | Level 0 | Kindergarten — foundational concepts, exposure |
//! | Level 1 | Grade 1 — basic operations |
//! | Level 2 | Grade 2 — elementary connections |
//! | Level 3 | Grade 3 — intermediate patterns |
//! | Level 4 | Grade 4 — applied computation |
//! | Level 5 | Grade 5 — cross-domain links |
//! | Level 6 | Grade 6 — algebra & abstraction |
//! | Level 7 | Grade 7 — formal reasoning |
//! | Level 8 | Grade 8 — systems thinking |
//! | Level 9 | Grade 9 — specialization |
//! | Level 10 | Grade 10 — advanced theory |
//! | Level 11 | Grade 11 — synthesis |
//! | Level 12 | Grade 12 — mastery |
//!
//! After Grade 12 (level 12), instead of graduating, the learner loops to
//! Kindergarten at the next cycle depth — a spiral, not a ladder.
//!
//! ## The Understanding Axis
//!
//! The axis value `depth = cycle × 13 + level` gives a unique linear ordering
//! while preserving the spiral structure. Two formulas at the same level but
//! different cycles are related (same band, deeper treatment).
//!
//! ## Per-Subject Tracks
//!
//! Each of the 12 zodiac domains is a subject track. A single Athena instance
//! might be at Math (Aries) level 7, Grammar (Capricorn) level 4, and
//! Code (Scorpio) level 9 independently — reflecting different levels of
//! internalized knowledge per domain.

use serde::{Deserialize, Serialize};

/// The four Bleach layers mapped to K-12 understanding levels.
///
/// Each layer represents a stage of mastery along the understanding axis:
///
/// | Layer      | Level | State    | Meaning                          |
/// |------------|-------|----------|----------------------------------|
/// | `Asauchi`  | 0     | Unknown  | No identity, public only         |
/// | `Zanpakuto`| 3     | Aware    | Named identity, basic access     |
/// | `Shikai`   | 6     | Learning | First release, intermediate power|
/// | `Bankai`   | 12    | Known    | Full release, complete mastery   |
///
/// This mirrors the Bleach power progression: an Asauchi is a nameless sealed sword.
/// Giving it a name awakens it as a Zanpakuto. Speaking its release command activates
/// Shikai. Mastering it completely unlocks Bankai.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BleachLayer {
    /// Level 0: No identity, public only — the nameless blade.
    Asauchi,
    /// Level 3: Named identity, aware of existence — the named blade.
    Zanpakuto,
    /// Level 6: Active learning and practice — the released blade.
    Shikai,
    /// Level 12: Complete mastery — the fully realized blade.
    Bankai,
}

impl BleachLayer {
    /// The K-12 level corresponding to this Bleach layer.
    pub fn level(self) -> u8 {
        match self {
            BleachLayer::Asauchi => 0,
            BleachLayer::Zanpakuto => 3,
            BleachLayer::Shikai => 6,
            BleachLayer::Bankai => 12,
        }
    }

    /// The human-readable state name for this layer.
    pub fn state(self) -> &'static str {
        match self {
            BleachLayer::Asauchi => "Unknown",
            BleachLayer::Zanpakuto => "Aware",
            BleachLayer::Shikai => "Learning",
            BleachLayer::Bankai => "Known",
        }
    }

    /// Derive the Bleach layer from a K-12 level (rounds down to nearest threshold).
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => BleachLayer::Asauchi,
            1..=3 => BleachLayer::Zanpakuto,
            4..=6 => BleachLayer::Shikai,
            _ => BleachLayer::Bankai, // 7..=12 maps to Bankai
        }
    }
}

impl std::fmt::Display for BleachLayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.state(), self.level())
    }
}

/// Maximum level number (Grade 12).
pub const MAX_LEVEL: u8 = 12;

/// Total levels per cycle (K + 12 grades).
pub const LEVELS_PER_CYCLE: u16 = 13;

/// The K-12 Spiral Understanding Axis for a single domain.
///
/// Tracks progression through the K-12 curriculum across multiple
/// depth cycles for one subject (domain).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UnderstandingAxis {
    /// K-12 level: 0 (K/kindergarten) through 12 (Grade 12).
    pub level: u8,
    /// Spiral cycle: 0 = first pass (foundational), 1 = deeper, etc.
    pub cycle: u8,
}

impl UnderstandingAxis {
    /// Create a new axis at the given level and cycle.
    ///
    /// Level is clamped to 0..=12. Cycle is free.
    pub fn new(level: u8, cycle: u8) -> Self {
        UnderstandingAxis {
            level: level.min(MAX_LEVEL),
            cycle,
        }
    }

    /// Kindergarten — the starting point (level 0, cycle 0).
    pub const fn kindergarten() -> Self {
        UnderstandingAxis { level: 0, cycle: 0 }
    }

    /// Linear depth value along the understanding axis.
    ///
    /// `depth = cycle × 13 + level`
    ///
    /// This creates a unique ordering: level 12 cycle 0 = 12, level 0 cycle 1 = 13.
    pub fn depth(self) -> u16 {
        self.cycle as u16 * LEVELS_PER_CYCLE + self.level as u16
    }

    /// Advance one level. If at level 12, loop to level 0 at next cycle.
    pub fn advance(&mut self) {
        if self.level >= MAX_LEVEL {
            self.level = 0;
            self.cycle += 1;
        } else {
            self.level += 1;
        }
    }

    /// Advance multiple levels.
    pub fn advance_by(&mut self, steps: u8) {
        for _ in 0..steps {
            self.advance();
        }
    }

    /// Get the next axis state without mutating.
    pub fn next(self) -> Self {
        let mut next = self;
        next.advance();
        next
    }

    /// Human-readable level name.
    pub fn level_name(self) -> &'static str {
        match self.level {
            0 => "Kindergarten",
            1 => "Grade 1",
            2 => "Grade 2",
            3 => "Grade 3",
            4 => "Grade 4",
            5 => "Grade 5",
            6 => "Grade 6",
            7 => "Grade 7",
            8 => "Grade 8",
            9 => "Grade 9",
            10 => "Grade 10",
            11 => "Grade 11",
            12 => "Grade 12",
            _ => "Beyond",
        }
    }

    /// Band name for the current level.
    pub fn band(self) -> &'static str {
        match self.level {
            0 => "Foundation",
            1..=3 => "Elementary",
            4..=6 => "Intermediate",
            7..=9 => "Advanced",
            10..=12 => "Mastery",
            _ => "Transcendent",
        }
    }

    /// The Bleach layer corresponding to this understanding level.
    ///
    /// Maps the K-12 level to the 4 Bleach power tiers:
    /// - Level 0  → Asauchi  (Unknown)
    /// - Level 3  → Zanpakuto (Aware)
    /// - Level 6  → Shikai   (Learning)
    /// - Level 12 → Bankai   (Known)
    pub fn bleach_layer(self) -> BleachLayer {
        BleachLayer::from_level(self.level)
    }

    /// Full description: "Cycle 0 Grade 3 (Elementary) — Zanpakuto (Aware)" etc.
    pub fn describe(self) -> String {
        format!(
            "Cycle {} {} ({}) — {} ({})",
            self.cycle,
            self.level_name(),
            self.band(),
            self.bleach_layer().state(),
            self.bleach_layer().level(),
        )
    }

    /// Shorter description: "Asauchi (Unknown)" / "Bankai (Known)" etc.
    pub fn describe_bleach(self) -> String {
        let layer = self.bleach_layer();
        format!(
            "{} ({}) — Level {}/{}",
            layer.state(),
            layer.level(),
            self.level,
            MAX_LEVEL
        )
    }
}

impl Default for UnderstandingAxis {
    fn default() -> Self {
        UnderstandingAxis::kindergarten()
    }
}

impl std::fmt::Display for UnderstandingAxis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}

// ─── Band helper ────────────────────────────────────────────────────────────

/// Curriculum bands — broader groupings of K-12 levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurriculumBand {
    /// Level 0: First exposure, no prerequisites
    Foundation,
    /// Levels 1–3: Basic operations and patterns
    Elementary,
    /// Levels 4–6: Applied reasoning and cross-links
    Intermediate,
    /// Levels 7–9: Formal systems and specialization
    Advanced,
    /// Levels 10–12: Synthesis and mastery
    Mastery,
}

impl CurriculumBand {
    /// The levels covered by this band.
    pub fn levels(self) -> std::ops::RangeInclusive<u8> {
        match self {
            CurriculumBand::Foundation => 0..=0,
            CurriculumBand::Elementary => 1..=3,
            CurriculumBand::Intermediate => 4..=6,
            CurriculumBand::Advanced => 7..=9,
            CurriculumBand::Mastery => 10..=12,
        }
    }

    /// Detect the band from a level value.
    pub fn from_level(level: u8) -> Self {
        match level {
            0 => CurriculumBand::Foundation,
            1..=3 => CurriculumBand::Elementary,
            4..=6 => CurriculumBand::Intermediate,
            7..=9 => CurriculumBand::Advanced,
            _ => CurriculumBand::Mastery,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kindergarten_default() {
        let axis = UnderstandingAxis::kindergarten();
        assert_eq!(axis.level, 0);
        assert_eq!(axis.cycle, 0);
        assert_eq!(axis.depth(), 0);
    }

    #[test]
    fn test_level_name() {
        assert_eq!(UnderstandingAxis::new(0, 0).level_name(), "Kindergarten");
        assert_eq!(UnderstandingAxis::new(6, 0).level_name(), "Grade 6");
        assert_eq!(UnderstandingAxis::new(12, 0).level_name(), "Grade 12");
    }

    #[test]
    fn test_band_detection() {
        assert_eq!(UnderstandingAxis::new(0, 0).band(), "Foundation");
        assert_eq!(UnderstandingAxis::new(2, 0).band(), "Elementary");
        assert_eq!(UnderstandingAxis::new(5, 0).band(), "Intermediate");
        assert_eq!(UnderstandingAxis::new(8, 0).band(), "Advanced");
        assert_eq!(UnderstandingAxis::new(11, 0).band(), "Mastery");
    }

    #[test]
    fn test_advance_loops_to_next_cycle() {
        let mut axis = UnderstandingAxis::new(12, 0);
        assert_eq!(axis.depth(), 12);
        axis.advance();
        assert_eq!(axis.level, 0);
        assert_eq!(axis.cycle, 1);
        assert_eq!(axis.depth(), 13);
    }

    #[test]
    fn test_advance_by_multiple_steps() {
        let mut axis = UnderstandingAxis::new(11, 0);
        axis.advance_by(3);
        // 11 → 12 → (loop) 0 cycle 1 → 1 cycle 1
        assert_eq!(axis.level, 1);
        assert_eq!(axis.cycle, 1);
        assert_eq!(axis.depth(), 14);
    }

    #[test]
    fn test_next_without_mutation() {
        let axis = UnderstandingAxis::new(5, 0);
        let next = axis.next();
        assert_eq!(axis.level, 5); // original unchanged
        assert_eq!(next.level, 6);
    }

    #[test]
    fn test_depth_ordering() {
        // level 12 cycle 0 should be before level 0 cycle 1
        let l12c0 = UnderstandingAxis::new(12, 0).depth();
        let l0c1 = UnderstandingAxis::new(0, 1).depth();
        assert!(l12c0 < l0c1);
        // 12 < 13
    }

    #[test]
    fn test_band_level_ranges() {
        assert!(CurriculumBand::Foundation.levels().contains(&0));
        assert!(CurriculumBand::Elementary.levels().contains(&2));
        assert!(CurriculumBand::Intermediate.levels().contains(&5));
        assert!(CurriculumBand::Advanced.levels().contains(&8));
        assert!(CurriculumBand::Mastery.levels().contains(&12));
    }

    #[test]
    fn test_describe() {
        let axis = UnderstandingAxis::new(0, 0);
        assert_eq!(
            axis.describe(),
            "Cycle 0 Kindergarten (Foundation) — Unknown (0)"
        );
        let axis = UnderstandingAxis::new(7, 2);
        assert_eq!(axis.describe(), "Cycle 2 Grade 7 (Advanced) — Known (12)");
    }

    #[test]
    fn test_bleach_layer_mapping() {
        assert_eq!(
            UnderstandingAxis::new(0, 0).bleach_layer(),
            BleachLayer::Asauchi
        );
        assert_eq!(
            UnderstandingAxis::new(3, 0).bleach_layer(),
            BleachLayer::Zanpakuto
        );
        assert_eq!(
            UnderstandingAxis::new(6, 0).bleach_layer(),
            BleachLayer::Shikai
        );
        assert_eq!(
            UnderstandingAxis::new(12, 0).bleach_layer(),
            BleachLayer::Bankai
        );
        // Levels beyond 12 still map to Bankai
        assert_eq!(BleachLayer::from_level(10), BleachLayer::Bankai);
    }

    #[test]
    fn test_bleach_layer_state() {
        assert_eq!(BleachLayer::Asauchi.state(), "Unknown");
        assert_eq!(BleachLayer::Zanpakuto.state(), "Aware");
        assert_eq!(BleachLayer::Shikai.state(), "Learning");
        assert_eq!(BleachLayer::Bankai.state(), "Known");
    }

    #[test]
    fn test_bleach_layer_levels() {
        assert_eq!(BleachLayer::Asauchi.level(), 0);
        assert_eq!(BleachLayer::Zanpakuto.level(), 3);
        assert_eq!(BleachLayer::Shikai.level(), 6);
        assert_eq!(BleachLayer::Bankai.level(), 12);
    }

    #[test]
    fn test_describe_bleach() {
        let axis = UnderstandingAxis::new(6, 0);
        assert_eq!(axis.describe_bleach(), "Learning (6) — Level 6/12");
        let axis = UnderstandingAxis::new(12, 0);
        assert_eq!(axis.describe_bleach(), "Known (12) — Level 12/12");
    }
}

# Vedic Alignment — Athena v0.2.0 Vision

> **From Western zodiac symbolism to Vedic (Jyotisha) architecture — a complete re-grounding of Athena's 4-layer reasoning engine in authentic Vedic astrology categories, with a reimagined Shikai-as-KB and Bankai-as-solved-form progression.**

---

## Table of Contents

1. [Philosophy: Why Vedic?](#1-philosophy-why-vedic)
2. [Overview of Changes](#2-overview-of-changes)
3. [9 Graha Wheel (Primary Domains)](#3-9-graha-wheel-primary-domains)
4. [12 Rashis (Secondary Classification)](#4-12-rashis-secondary-classification)
5. [Vedic Aspect System](#5-vedic-aspect-system)
6. [7-Layer Descent (Vedic)](#6-7-layer-descent-vedic)
7. [Gyro: 9-Node Precession](#7-gyro-9-node-precession)
8. [AtomClassification: Vedic-First](#8-atomclassification-vedic-first)
9. [Entity System Re-grounding](#9-entity-system-re-grounding)
10. [Formula Re-alignment](#10-formula-re-alignment)
11. [4-Layer Architecture Refactor](#11-4-layer-architecture-refactor)
12. [Asauchi — Unchanged Portal](#12-asauchi--unchanged-portal)
13. [Zanpakuto — Expanded NLP + Identity Engine](#13-zanpakuto--expanded-nlp--identity-engine)
14. [Shikai — Dynamic User Knowledge Base (NEW)](#14-shikai--dynamic-user-knowledge-base-new)
15. [Bankai — Solved Atomic KB Form (NEW)](#15-bankai--solved-atomic-kb-form-new)
16. [Shikai → Bankai Progression](#16-shikai--bankai-progression)
17. [User Journey: From Asauchi to Bankai](#17-user-journey-from-asauchi-to-bankai)
18. [Implementation Phases](#18-implementation-phases)
19. [Migration Path](#19-migration-path)
20. [appendix: Vedic Category Reference](#20-appendix-vedic-category-reference)

---

## 1. Philosophy: Why Vedic?

The current Athena uses Western zodiac signs (Aries→Pisces) as **mnemonic labels** for 12 knowledge domains. While functional, this is a thin veneer over a fundamentally Western-category system.

Vedic astrology (Jyotisha = "science of light") offers a richer, more structurally sound foundation for a *reasoning engine* because:

| Feature | Western (current) | Vedic (proposed) |
|---------|------------------|-----------------|
| Primary divisions | 12 equal signs | 9 Grahas (planets) + 12 Rashis (signs) |
| Element system | 4 (Fire, Earth, Air, Water) | 5 (with Akasha/Ether as substrate) |
| Quality system | 3 modalities (Cardinal, Fixed, Mutable) | 3 Gunas (Sattva, Rajas, Tamas) + Purusharthas |
| Lunar subdivision | None | 27 Nakshatras — fine-grained 13°20' divisions |
| Node system | None | Rahu + Ketu — karmic axis, innovation/detachment |
| Goal orientation | None | 4 Purusharthas: Dharma, Artha, Kama, Moksha |
| Calculus framework | Dominant/western | Bija (seed), Nadi, Dasha timing systems |

### Key Insight: 9 Grahas as Cognitive Functions

The 9 grahas (planets) map naturally to distinct *cognitive and knowledge domains*:

| Graha | Cognitive Function | Knowledge Domain | Archetype |
|-------|-------------------|-----------------|-----------|
| Surya (Sun) | **Identity & Authority** | Self, Leadership, Governance | The Sovereign |
| Chandra (Moon) | **Mind & Emotion** | Psychology, Mental Processing | The Reflector |
| Mangala (Mars) | **Action & Energy** | Engineering, Force, Drive | The Warrior |
| Budha (Mercury) | **Intellect & Communication** | Logic, Language, Commerce | The Messenger |
| Brihaspati (Jupiter) | **Wisdom & Expansion** | Philosophy, Law, Dharma | The Guru |
| Shukra (Venus) | **Beauty & Value** | Arts, Wealth, Relationships | The Sage |
| Shani (Saturn) | **Structure & Discipline** | Time, Karma, Form | The Ascetic |
| Rahu (North Node) | **Innovation & Ambition** | Technology, Foreign, Materialism | The Seeker |
| Ketu (South Node) | **Spirituality & Liberation** | Deep Science, Detachment | The Liberated |

---

## 2. Overview of Changes

### Current → Proposed Mapping

| Layer | Current (Western) | Proposed (Vedic) |
|-------|-------------------|-----------------|
| **Wheel nodes** | 12 Domains (Aries→Pisces) | 9 Grahas (Surya→Ketu) |
| **Wheel aspects** | 5 (Conjunction→Opposition) | 6 Vedic aspects + graha relationships |
| **Classification axes** | 7 Western | 7 Vedic (Grahas, Rashis, Tattvas, Gunas, Nakshatras, Houses, Purusharthas) |
| **Descent layers** | Macro→NAND (7 layers) | Unmanifest→Brahman (7 layers, Vedic-named) |
| **Gyro** | 12-sign, 30° intervals | 9-graha, 40° intervals |
| **Asauchi** | Public CLI | Unchanged (is already neutral) |
| **Zanpakuto** | Access + basic NLP | Full NLP pipeline + identity mastery |
| **Shikai** | Query parser | **User-constructed dynamic knowledge base** |
| **Bankai** | Solve engine | **Solved atomic form of user's Shikai KB** |
| **Entities** | Western-grounded | Vedic-grounded (Navagraha, Nakshatra deities, etc.) |
| **Formulas** | Domain-agnostic (current labeling) | Vedic-framed (graha-specific proven formulas) |
| **CLI commands** | shikai, solve, eval, etc. | shikai-build, shikai-status, bankai-release |

### What Stays

- **4-layer architecture** (Asauchi→Zanpakuto→Shikai→Bankai) — the Bleach metaphor gets *stronger*, not weaker
- **Rust implementation** — no language change
- **TOML data files** — format stays, content regrounds
- **MCP server** — 12 tools, now KB-aware
- **All 5 validation gates** — they become aspects of the Vedic descent
- **NAND as bedrock** — NAND is universal; the name stays
- **Build/tooling** — unchanged

---

## 3. 9 Graha Wheel (Primary Domains)

### The Domain Enum

```rust
/// The 9 Vedic grahas (planets) as knowledge domains, arranged in wheel order.
///
/// Order follows the Vedic weekday (starting with Surya at 0°) and proceeds
/// through decreasing orbital period: Surya → Chandra → Mangala → Budha →
/// Brihaspati → Shukra → Shani → Rahu → Ketu.
///
/// Each graha governs 40° of the 360° wheel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Graha {
    /// ☉ Surya (Sun) — Identity, Self, Leadership, Governance
    /// Tattva: Fire | Guna: Sattva | Rashi: Simha
    Surya,
    /// ☽ Chandra (Moon) — Mind, Emotion, Psychology, Nurturing
    /// Tattva: Water | Guna: Sattva | Rashi: Karka
    Chandra,
    /// ♂ Mangala (Mars) — Action, Energy, Engineering, Courage
    /// Tattva: Fire | Guna: Rajas | Rashi: Mesha, Vrishchika
    Mangala,
    /// ☿ Budha (Mercury) — Communication, Logic, Intellect, Commerce
    /// Tattva: Earth | Guna: Rajas | Rashi: Mithuna, Kanya
    Budha,
    /// ♃ Brihaspati (Jupiter) — Wisdom, Law, Philosophy, Teaching
    /// Tattva: Ether | Guna: Sattva | Rashi: Dhanu, Meena
    Brihaspati,
    /// ♀ Shukra (Venus) — Beauty, Arts, Wealth, Relationships
    /// Tattva: Water | Guna: Sattva | Rashi: Vrishabha, Tula
    Shukra,
    /// ♄ Shani (Saturn) — Structure, Discipline, Time, Karma
    /// Tattva: Air | Guna: Tamas | Rashi: Makara, Kumbha
    Shani,
    /// ☊ Rahu (North Lunar Node) — Innovation, Ambition, Technology, Foreign
    /// Tattva: Air | Guna: Tamas | Rashi: Not a ruler (shadow graha)
    Rahu,
    /// ☋ Ketu (South Lunar Node) — Spirituality, Liberation, Deep Science, Detachment
    /// Tattva: Ether | Guna: Tamas | Rashi: Not a ruler (shadow graha)
    Ketu,
}
```

### Wheel Order (40° each)

```
0°    Surya (Sun)
40°   Chandra (Moon)
80°   Mangala (Mars)
120°  Budha (Mercury)
160°  Brihaspati (Jupiter)
200°  Shukra (Venus)
240°  Shani (Saturn)
280°  Rahu (North Node)
320°  Ketu (South Node) → wraps to 360°/0° Surya
```

### Knowledge Domain Assignment

```rust
impl Graha {
    pub fn knowledge_domain(self) -> &'static str {
        match self {
            Graha::Surya => "Self & Leadership — identity, authority, governance",
            Graha::Chandra => "Mind & Emotion — psychology, nurturing, mental processing",
            Graha::Mangala => "Action & Energy — engineering, force, drive, courage",
            Graha::Budha => "Logic & Communication — intellect, commerce, language",
            Graha::Brihaspati => "Wisdom & Law — philosophy, dharma, teaching, expansion",
            Graha::Shukra => "Arts & Value — beauty, wealth, creativity, relationships",
            Graha::Shani => "Structure & Time — discipline, karma, limitation, longevity",
            Graha::Rahu => "Innovation & Ambition — technology, foreign, materialism",
            Graha::Ketu => "Spirituality & Science — liberation, deep research, detachment",
        }
    }
    
    /// The 360° wheel angle for this graha.
    pub fn wheel_angle(self) -> f64 {
        self.index() as f64 * 40.0
    }
}
```

### Graha Relationships: Mitra (Friend), Shatru (Enemy), Sama (Neutral)

Vedic astrology classifies graha relationships into three types, which map naturally to aspect tension:

| Relationship | Meaning | Aspect Quality |
|-------------|---------|----------------|
| **Mitra** (Friend) | Natural affinity, cooperation | Direct flow (like Sextile/Trine) |
| **Shatru** (Enemy) | Natural tension, competition | Tension (like Square) |
| **Sama** (Neutral) | Neither friend nor foe | Neutral |
| **Adhi Mitra** (Best Friend) | Deep natural harmony | Strong flow |
| **Adhi Shatru** (Bitter Enemy) | Deep natural conflict | Strong tension |

### Alternative: 12 Rashis as Domains

If 9 nodes feels too constraining, an alternative is keeping 12 domains but using the Vedic Rashis:

| Index | Sanskrit | Devanagari | Western | Lord | Tattva | Guna | Domain |
|-------|----------|------------|---------|------|--------|------|--------|
| 0 | Mesha | मेष | Aries | Mangala | Fire (Tejas) | Rajas | Mathematics & Logic |
| 1 | Vrishabha | वृषभ | Taurus | Shukra | Earth (Prithvi) | Tamas | Physics & Matter |
| 2 | Mithuna | मिथुन | Gemini | Budha | Air (Vayu) | Rajas | Communication & Networks |
| 3 | Karka | कर्क | Cancer | Chandra | Water (Apas) | Sattva | Psychology & Nurturing |
| 4 | Simha | सिंह | Leo | Surya | Fire (Tejas) | Sattva | Biology & Medicine |
| 5 | Kanya | कन्या | Virgo | Budha | Earth (Prithvi) | Tamas | Economics & Analysis |
| 6 | Tula | तुला | Libra | Shukra | Air (Vayu) | Rajas | Design & Engineering |
| 7 | Vrishchika | वृश्चिक | Scorpio | Mangala | Water (Apas) | Tamas | Computer Science & AI |
| 8 | Dhanu | धनु | Sagittarius | Brihaspati | Fire (Tejas) | Sattva | History & Anthropology |
| 9 | Makara | मकर | Capricorn | Shani | Earth (Prithvi) | Tamas | Language & Linguistics |
| 10 | Kumbha | कुम्भ | Aquarius | Shani | Air (Vayu) | Sattva | Philosophy & Ethics |
| 11 | Meena | मीन | Pisces | Brihaspati | Water (Apas) | Sattva | Neuroscience & Consciousness |

**Recommendation**: Use **9 Grahas** as the primary wheel (more authentically Vedic, elegant 3×3 structure) and retain **12 Rashis** as a secondary classification axis within the AtomClassification system.

---

## 4. 12 Rashis (Secondary Classification)

The 12 rashis remain as a secondary classification axis within `AtomClassification`:

```rust
/// 12 Vedic rashis (sidereal signs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rashi {
    Mesha,      // 0 — मेष — Aries (Fire, Rajas)
    Vrishabha,  // 1 — वृषभ — Taurus (Earth, Tamas)
    Mithuna,    // 2 — मिथुन — Gemini (Air, Rajas)
    Karka,      // 3 — कर्क — Cancer (Water, Sattva)
    Simha,      // 4 — सिंह — Leo (Fire, Sattva)
    Kanya,      // 5 — कन्या — Virgo (Earth, Tamas)
    Tula,       // 6 — तुला — Libra (Air, Rajas)
    Vrishchika, // 7 — वृश्चिक — Scorpio (Water, Tamas)
    Dhanu,      // 8 — धनु — Sagittarius (Fire, Sattva)
    Makara,     // 9 — मकर — Capricorn (Earth, Tamas)
    Kumbha,     // 10 — कुम्भ — Aquarius (Air, Sattva)
    Meena,      // 11 — मीन — Pisces (Water, Sattva)
}
```

Each Rashi maps to a Graha lord, a Tattva (element), a Guna, and a knowledge domain — forming a 3-dimensional classification tensor.

### The Graha × Rashi × Nakshatra Tensor

```
Graha (9) × Rashi (12) × Nakshatra (27) = 2,916 classification buckets

Each entity token settles into one or more buckets with weighted confidence.
```

---

## 5. Vedic Aspect System

Aspects in Vedic are more nuanced than the Western 5-aspect system. The primary aspects are:

### Pada (Footstep) Aspects — based on position

| Aspect | Arc Distance | Graha Steps | Quality |
|--------|-------------|-------------|---------|
| **Yuti** (Conjunction) | 0° | 0 | Unity — same graha |
| **Sama** (Equal) | 40° | 1 | Adjacent — sequential |
| **Varga** (Division) | 80° | 2 | Secondary relationship |
| **Trikona** (Trine) | 120° | 3 | **Harmonious** — dharma/kama/moksha |
| **Kona** (Quadrant) | 160° | 4 | 7th house aspect from |
| **Ardhakona** (Semi-quad) | 200° | 5 | 5th house aspect (Jupiter special) |
| **Saptama** (Opposition) | 160° (shortest) | 4 or 5 | **Full aspect** — all grahas |

### Graha-Specific Special Aspects (Visesha Drishti)

In Vedic, certain grahas have *additional* aspect powers beyond the universal 7th-house aspect:

| Graha | Special Aspects | Meaning |
|-------|----------------|---------|
| **Mangala** (Mars) | 4th, 7th, 8th | Action, transformation, hidden power |
| **Brihaspati** (Jupiter) | 5th, 7th, 9th | Wisdom, expansion, dharma trine |
| **Shani** (Saturn) | 3rd, 7th, 10th | Discipline, karma, career |

### Mitra-Shatru Aspects (Friendship/Enmity)

Beyond positional aspects, each graha pair has an inherent relationship:

```rust
pub enum GrahaRelationship {
    AdhiMitra,    // Best friend — strongest flow
    Mitra,        // Friend — direct flow
    Sama,         // Neutral — moderate flow
    Shatru,       // Enemy — tension
    AdhiShatru,   // Bitter enemy — strongest tension
}
```

Precomputed 9×9 lookup:

| | Surya | Chandra | Mangala | Budha | Brihaspati | Shukra | Shani | Rahu | Ketu |
|-|-------|---------|---------|-------|------------|--------|-------|------|------|
| **Surya** | Self | Mitra | Shatru | Mitra | Mitra | Mitra | Shatru | Sama | Sama |
| **Chandra** | Mitra | Self | Mitra | Mitra | Mitra | Mitra | Sama | Sama | Sama |
| **Mangala** | Shatru | Mitra | Self | Mitra | Mitra | Shatru | Sama | Sama | Sama |
| **Budha** | Mitra | Mitra | Mitra | Self | Sama | Sama | Sama | Sama | Sama |
| **Brihaspati** | Mitra | Mitra | Mitra | Sama | Self | Mitra | Sama | Sama | Sama |
| **Shukra** | Mitra | Mitra | Shatru | Sama | Mitra | Self | Sama | Sama | Sama |
| **Shani** | Shatru | Sama | Sama | Sama | Sama | Sama | Self | Sama | Sama |
| **Rahu** | Sama | Sama | Sama | Sama | Sama | Sama | Sama | Self | Mitra |
| **Ketu** | Sama | Sama | Sama | Sama | Sama | Sama | Sama | Mitra | Self |

---

## 6. 7-Layer Descent (Vedic)

The descent system gets renamed from Western-coded layers to Vedic-coded layers, keeping the same 7-depth structure:

```rust
/// Vedic descent layers — a token descends from pure potential to atomic truth.
///
/// ```text
/// 0: Akasha (Ether)    — Unmanifest potential, pure Akasha
/// 1: Tattva (Element)  — Five great elements (Fire, Earth, Air, Water, Ether)
/// 2: Guna (Quality)    — Three gunas: Sattva, Rajas, Tamas
/// 3: Nakshatra (Mansion) — 27 lunar mansions, finer resolution
/// 4: Rashi (Sign)      — 12 sidereal rashis
/// 5: Graha (Planet)    — 9 grahas, cognitive function
/// 6: Brahman (Absolute) — NAND gate — provably true at gate level
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum VedicLayer {
    /// Akasha (Ether) — unmanifest potential, floating token (depth 0)
    Akasha = 0,
    /// Tattva — element classification (depth 1)
    Tattva = 1,
    /// Guna — quality classification (depth 2)
    Guna = 2,
    /// Nakshatra — lunar mansion (depth 3)
    Nakshatra = 3,
    /// Rashi — zodiac sign (depth 4)
    Rashi = 4,
    /// Graha — planetary domain (depth 5)
    Graha = 5,
    /// Brahman — absolute truth, NAND gate resolution (depth 6)
    Brahman = 6,
}
```

### Descent Path

```
Token "force"
  → Akasha:     unmanifest potential [0.0]
  → Tattva:     Fire (Tejas) [0.8] — force transforms
  → Guna:       Rajas [0.7] — force drives action
  → Nakshatra:  Magha [0.6] — force of authority/throne
  → Rashi:      Mesha [0.7] or Simha [0.6] — fiery signs
  → Graha:      Mangala [0.85] — Mars, action, energy
  → Brahman:    resolved to NAND gates [true]
```

---

## 7. Gyro: 9-Node Precession

The gyroscopic wheel changes from 12×30° to 9×40°:

```rust
pub struct GyroState {
    /// Current orientation on the 9-graha wheel (degrees, 0 = Surya)
    pub orientation: Orientation,
    /// Angular velocity (degrees/second)
    pub angular_velocity: f64,
    /// Mass distribution around the wheel (9 grahas)
    pub mass_distribution: [f64; 9],  // was [f64; 12]
}
```

Each graha occupies 40°:
- Surya: 0°–40°
- Chandra: 40°–80°
- Mangala: 80°–120°
- Budha: 120°–160°
- Brihaspati: 160°–200°
- Shukra: 200°–240°
- Shani: 240°–280°
- Rahu: 280°–320°
- Ketu: 320°–360°

Alignment weights use cosine falloff over 40° intervals instead of 30°.

---

## 8. AtomClassification: Vedic-First

The 7-axis classification is re-ordered to be Vedic-first, Western-second:

```rust
pub struct VedicAtomClassification {
    // ─── Primary Vedic axes ───────────────────────────────────────
    /// 9 Grahas (planets) — the primary cognitive domains
    pub grahas: [f64; 9],
    /// 12 Rashis (sidereal signs) — secondary positional axis
    pub rashis: [f64; 12],
    /// 27 Nakshatras (lunar mansions) — fine-grained resolution
    pub nakshatras: [f64; 27],
    /// 5 Tattvas (elements + Akasha) — material substrate
    pub tattvas: [f64; 5],
    /// 3 Gunas (primordial qualities) — fundamental nature
    pub gunas: [f64; 3],
    /// 4 Purusharthas (life goals) — purpose/orientation
    pub purusharthas: [f64; 4],
    
    // ─── Secondary (for cross-reference) ───────────────────────────
    /// Aspect affinities (conjunction, sextile, etc. — computed)
    pub aspects: [f64; 5],
    /// Polarity (0 = Purusha/active, 1 = Prakriti/receptive)
    pub polarity: f64,
}
```

### Purusharthas — 4 Life Goals

```rust
pub enum Purushartha {
    Dharma  = 0,  // Righteousness, duty, cosmic order
    Artha   = 1,  // Prosperity, wealth, means of life
    Kama    = 2,  // Desire, pleasure, creative expression
    Moksha  = 3,  // Liberation, enlightenment, release
}
```

---

## 9. Entity System Re-grounding

Entities are regrounded in Vedic archetypes. Current planetary entities (Mars, Venus, etc.) remain but are re-mapped to their Vedic equivalents.

### The Navagraha Entity Set

```toml
# entities/navagraha.toml
# The 9 Grahas as primary entities

[[entity]]
id = "surya"
name = "Surya"
vedic_name = "आदित्य"
domain = "surya"
description = "Sun — soul, authority, vitality. Ruler of Simha (Leo). Atmakaraka (soul significator)."
graha = "surya"
rashi = "simha"
tattva = "fire"
guna = "sattva"
purushartha = "dharma"
properties = {
  vitality = 0.95, authority = 0.9, leadership = 0.88,
  self_expression = 0.9, sovereignty = 0.85
}
mantra = "ॐ ह्रां ह्रीं ह्रौं सः सूर्याय नमः"
bija = "ह्रां"
day = "sunday"
constants = { mass_kg = 1.9885e30, radius_m = 6.957e8, luminosity_w = 3.828e26 }

[[entity]]
id = "chandra"
name = "Chandra"
vedic_name = "चन्द्र"
domain = "chandra"
description = "Moon — mind, emotion, nurturing. Ruler of Karka (Cancer). Manokaraka (mind significator)."
graha = "chandra"
rashi = "karka"
tattva = "water"
guna = "sattva"
purushartha = "kama"
properties = {
  emotional_depth = 0.9, intuition = 0.88, nurture = 0.85,
  receptivity = 0.85, mental_fluidity = 0.8
}
mantra = "ॐ सों सोमाय नमः"
bija = "सों"
day = "monday"
constants = { mass_kg = 7.342e22, radius_m = 1.7374e6, orbital_period_days = 27.32 }

# ... remaining 7 grahas follow same pattern
```

### Entity → Nakshatra Connections

```toml
# Each graha entity gains nakshatra connections with shakti (power) descriptions

[[entity]]
id = "ketu"
name = "Ketu"
vedic_name = "केतु"
domain = "ketu"
description = "South Lunar Node — detachment, spirituality, moksha, past-life wisdom."
graha = "ketu"
tattva = "ether"
guna = "tamas"
purushartha = "moksha"
properties = {
  spirituality = 0.9, detachment = 0.85, wisdom = 0.8,
  liberation = 0.88, deep_research = 0.85
}
mantra = "ॐ कें केतवे नमः"
bija = "कें"
# Nakshatras ruled by Ketu: Ashwini, Magha, Mula
ruled_nakshatras = ["ashwini", "magha", "mula"]
```

### Seed Entity Toml Structure (Migrated)

```toml
[[entity]]
id = "gravity"
name = "Gravity"
domain = "mangala"  # was "taurus" (Physics)
description = "Universal attraction — Mangala: the force that draws matter together."
tags = ["force", "physics", "attraction", "mangala"]
vedic_classification = {
  grahas = [0.0, 0.0, 0.8, 0.0, 0.0, 0.0, 0.3, 0.0, 0.0],  # Mangala primary, Shani secondary
  tattvas = [0.8, 0.3, 0.2, 0.1, 0.0],  # Fire dominant
  gunas = [0.3, 0.7, 0.2],              # Rajas dominant
}
properties = { strength = 0.9, range = 1.0, universality = 0.95 }
```

### Entity to Entity Relationships

Beyond single-entity files, add explicit relationship graphs:

```toml
# entities/relationships.toml

[[relationship]]
from = "surya"
to = "chandra"
aspect = "saptama"  # Opposition (full aspect)
graha_relation = "mitra"  # Friends
description = "Sun and Moon are friends — consciousness reflects on itself"
confidence = 0.95

[[relationship]]
from = "mangala"
to = "shukra"
aspect = "visesha"  # Mars has special aspect on Venus
graha_relation = "shatru"  # Enemies
description = "Mars and Venus are enemies — action vs. harmony, tension of drive and desire"
confidence = 0.90
```

---

## 10. Formula Re-alignment

Formulas get regrounded in Vedic concepts. The formula TOML structure stays the same, but the domain assignments, tags, descriptions, and evidence fields change.

### Atomic Formula Example (Surya — Self & Leadership)

```toml
# formulas/atomic/surya_self.toml
# ☉ Surya — Self, Identity, Leadership — Atomic Formulas
# Fire (Tejas) | Sattva | Rashi: Simha
# Purushartha: Dharma — the sovereign, dharma as cosmic order
# Anatomical: Heart, Spine — center of vitality

[[formula]]
id = "identity_function"
tags = ["algebra", "surya", "self", "identity", "sovereign"]
domain = "surya"
level = 0
inputs = ["x"]
output = "x"
expression = "x"
description = "Identity: id(x) = x — Surya: pure self, the sovereign 'I am'"
evidence = "Category theory: id_A : A → A — the self-morphism as atman (self)"

[[formula]]
id = "self_reference"
tags = ["logic", "surya", "self", "fixed_point", "sovereign"]
domain = "surya"
level = 6
inputs = ["f_x", "x"]
output = "is_fixed"
expression = "f_x - x"
description = "Fixed point: f(x) = x — Surya: self-recognition, the sovereign who sees itself"
evidence = "Gödel's fixed-point lemma: self-reference as the foundation of mathematics"
```

### Atomic Formula Example (Mangala — Action & Engineering)

```toml
# formulas/atomic/mangala_action.toml
# ♂ Mangala — Action, Energy, Engineering
# Fire (Tejas) | Rajas | Rashi: Mesha, Vrishchika
# Purushartha: Artha — material accomplishment

[[formula]]
id = "force_mass_acceleration"
tags = ["physics", "mangala", "force", "action", "mesha"]
domain = "mangala"
level = 3
inputs = ["mass", "acceleration"]
output = "force"
expression = "mass * acceleration"
description = "Newton's second law: F = ma — Mangala: action through mass and acceleration"
evidence = "Newton's Principia: force is proportional to the change in momentum"

[[formula]]
id = "kinetic_energy"
tags = ["physics", "mangala", "energy", "motion", "ardha"]
domain = "mangala"
level = 6
inputs = ["mass", "velocity"]
output = "ke"
expression = "0.5 * mass * velocity^2"
description = "Kinetic energy: K = ½mv² — Mangala: the energy of action"
evidence = "Work-energy theorem: work done = change in kinetic energy"
```

### Nakshatra Formula Connections

Formulas can now reference nakshatra energies for fine-grained resolution:

```toml
[[formula]]
id = "healing_efficacy"
tags = ["biology", "ashwini", "ketu", "healing", "speed"]
domain = "ketu"
nakshatra = "ashwini"  # Ashwini = healing, speed (ruled by Ketu)
level = 9
inputs = ["treatment_potency", "patient_vitality"]
output = "healing_rate"
expression = "treatment_potency * patient_vitality^0.5"
description = "Ashwini healing: healing rate ∝ treatment × √vitality — the horse-star's swift restoration"
evidence = "Clinical recovery models: exponential decay of illness with treatment"
```

---

## 11. 4-Layer Architecture Refactor

Here is the complete refactored architecture:

```
┌─────────────────────────────────────────────────────────┐
│                     A S A U C H I                        │
│              Nameless blade — public CLI                  │
│    info, version, ping, public_validate                   │
│     "What everyone sees first"                            │
└──────────────────────┬──────────────────────────────────┘
                       │ Raw query string
                       ▼
┌─────────────────────────────────────────────────────────┐
│                    Z A N P A K U T O                     │
│           Named identity — full NLP preprocessing         │
│                                                           │
│  ┌──────────────────────────────────────────────────┐    │
│  │ Identity Engine                                   │    │
│  │ • register(name) → Identity                       │    │
│  │ • authenticate(session) → AccessTier              │    │
│  │ • track_query_count() → mastery_level             │    │
│  └──────────────┬───────────────────────────────────┘    │
│                 │                                        │
│  ┌──────────────▼───────────────────────────────────┐    │
│  │ NLP Engine (expanded — absorbs old Shikai parse)   │    │
│  │ • tokenize() → tokens                              │    │
│  │ • remove_stopwords() → significant_tokens         │    │
│  │ • stem() → stems                                   │    │
│  │ • classify_intent() → Intent                       │    │
│  │ • classify_domain() → Graha[]                      │    │
│  │ • resolve_entities() → Entity[]                    │    │
│  │ • detect_formulas() → Formula[]                    │    │
│  │ • extract_arguments() → Args                       │    │
│  │ • build_nlp_context() → NlpContext                 │    │
│  └──────────────────────────────────────────────────┘    │
│                                                           │
│  Output: NlpContext (fully parsed, no Shikai work left)   │
└──────────────────────┬──────────────────────────────────┘
                       │ NlpContext (fully parsed query)
                       ▼
┌─────────────────────────────────────────────────────────┐
│                      S H I K A I                        │
│         Your released blade — YOUR knowledge base        │
│                                                           │
│  ┌──────────────────────────────────────────────────┐    │
│  │ Dynamic KB Engine (NEW)                           │    │
│  │                                                   │    │
│  │ Manages user's personal knowledge graph:          │    │
│  │ • add_entity(id, name, domain, properties)        │    │
│  │ • add_formula(id, expression, inputs, output)     │    │
│  │ • add_relationship(from, to, aspect, confidence)  │    │
│  │ • add_nakshatra_connection(entity, nakshatra)     │    │
│  │ • remove(id)                                      │    │
│  │ • update(id, changes)                             │    │
│  │ • search(query) → KB results                      │    │
│  │ • graph() → KB graph visualization                │    │
│  │ • status() → KB completeness report               │    │
│  │ • export() → KB snapshot file                      │    │
│  │ • import(path) → load KB                          │    │
│  └──────────────────────────────────────────────────┘    │
│                                                           │
│  ┌──────────────────────────────────────────────────┐    │
│  │ KB Completeness Monitor                           │    │
│  │ • grounded_formulas() → count                     │    │
│  │ • floating_formulas() → count                     │    │
│  │ • connected_components() → graph clusters         │    │
│  │ • resolution_percentage() → 0.0..1.0             │    │
│  │ • nand_coverage() → fraction of KB as NAND        │    │
│  │ • can_release_bankai() → bool                     │    │
│  └──────────────────────────────────────────────────┘    │
│                                                           │
│  Output: ShikaiQuery (KB-grounded, ready for solve)       │
└──────────────────────┬──────────────────────────────────┘
                       │ ShikaiQuery (with KB context)
                       ▼
┌─────────────────────────────────────────────────────────┐
│                     B A N K A I                         │
│         The solved atomic form — released KB             │
│                                                           │
│  ┌──────────────────────────────────────────────────┐    │
│  │ KB Compiler (NEW)                                 │    │
│  │ • compile() — transforms KB into NAND DAG        │    │
│  │ • validate() — checks KB consistency             │    │
│  │ • optimize() — reduces redundant paths           │    │
│  │ • verify() — proves all KB paths terminate       │    │
│  └──────────────────────────────────────────────────┘    │
│                                                           │
│  ┌──────────────────────────────────────────────────┐    │
│  │ Solve Engine (existing, adapted)                  │    │
│  │ • eval() — evaluates single formula               │    │
│  │ • compose() — chains formulas                     │    │
│  │ • traverse() — follows wheel paths                │    │
│  │ • reason() — BFS from have→want                   │    │
│  │ • validate() — runs 5 gates                       │    │
│  └──────────────────────────────────────────────────┘    │
│                                                           │
│  ┌──────────────────────────────────────────────────┐    │
│  │ BankaiSolve (output)                              │    │
│  │ • solved_chain: Vec<Step>                         │    │
│  │ • nand_dag: NandDag (compiled KB)                 │    │
│  │ • confidence: f64 (overall solve confidence)      │    │
│  │ • kb_hash: String (fingerprint of KB state)       │    │
│  └──────────────────────────────────────────────────┘    │
│                                                           │
│  Output: BankaiSolve — final, atomic, verified            │
└─────────────────────────────────────────────────────────┘
```

---

## 12. Asauchi — Unchanged Portal

Asauchi stays as-is. It is already neutral — it presents the system identity, version, and basic capabilities. No Vedic concepts needed at this layer.

**Changes:**
- `capabilities` list updates to reflect Vedic wheel:
  ```rust
  capabilities: vec![
      "vedic_wheel:9_grahas",
      "rashis:12_sidereal_signs",
      "nakshatras:27_lunar_mansions",
      "tattvas:5_elements_with_akasha",
      "gunas:3_primordial_qualities",
      "purusharthas:4_life_goals",
      "primitive_formulas:22_gates",
      "vedic_entity_system",
      "dynamic_knowledge_base",
      "validation_gates:math_logic_confidence_formal",
      "cross_domain_traversal",
      "mcp_server",
      "cli_interface",
  ]
  ```

---

## 13. Zanpakuto — Expanded NLP + Identity Engine

Zanpakuto becomes the **complete preprocessing layer** — absorbing all parsing work that currently lives in Shikai.

### Identity Engine (New Module)

```rust
/// Tracks user progression through the 4 tiers
pub struct IdentityEngine {
    identities: HashMap<String, Identity>,
    session_store: HashMap<String, SessionData>,
}

pub struct SessionData {
    pub query_count: u64,
    pub kb_entities: usize,
    pub kb_formulas: usize,
    pub kb_relationships: usize,
    pub kb_completeness: f64,
    pub tier: AccessTier,
}

impl IdentityEngine {
    pub fn register(&mut self, name: &str) -> Identity;
    pub fn authenticate(&self, session: &str) -> Result<Identity>;
    pub fn track_query(&mut self, session: &str);
    pub fn update_progress(&mut self, session: &str, kb_status: &KBStatus);
    pub fn check_tier_eligibility(&self, session: &str) -> Option<AccessTier>;
}
```

### NLP Engine (Expanded)

Current NLP has tokenization, stemming, stopwords, entity recognition, intent scoring, domain scoring. It needs:

**New capabilities:**
1. **Formula detection** — match raw text to formula IDs in the registry (move from Shikai)
2. **Argument extraction** — parse `key=value` pairs and numeric literals
3. **Relationship parsing** — detect "from X to Y", "X → Y" patterns
4. **Nakshatra detection** — recognize nakshatra names and their symbols
5. **Intent classification** — fully move Intent enum from Shikai to Zanpakuto
6. **K-12 level detection** — move level/cycle detection from Shikai

```rust
impl NlpEngine {
    /// Full pipeline — replaces Shikai::process_query parsing
    pub fn process(&self, raw: &str, identity: &Identity) -> NlpContext {
        // 1. Tokenize
        let tokens = self.tokenize(raw);
        // 2. Preprocess
        let significant = self.remove_stopwords(&tokens);
        let stems = self.stem(&significant);
        // 3. Classify
        let intent = self.classify_intent(&tokens, &stems);
        let domain_scores = self.classify_graha_domains(&tokens, &stems);
        // 4. Resolve
        let entity_matches = self.resolve_entities(&tokens, identity);
        let formula_matches = self.detect_formulas(&tokens, &stems);
        let args = self.extract_arguments(raw);
        // 5. Level/cycle detection (moved from Shikai)
        let level = self.detect_level(&tokens);
        let cycle = self.detect_cycle(&tokens);
        // 6. Build context
        NlpContext {
            original: raw.into(),
            tokens,
            significant_tokens: significant,
            stems,
            intent,
            graha_scores: domain_scores,  // was domain_scores
            entity_matches,
            formula_matches,
            args,
            level,
            cycle,
            likely_graha: domain_scores.first().map(|(g, _)| *g),
            likely_entity: entity_matches.first().map(|(id, _, _)| id.clone()),
            identity_name: identity.name.clone(),
            access_tier: identity.tier,
        }
    }
}
```

### Zanpakuto's Output Contract

The `NlpContext` is now a **complete parse** — Shikai receives a fully preprocessed query and needs only to check the user's KB and route to Bankai.

---

## 14. Shikai — Dynamic User Knowledge Base (NEW)

This is the most transformative change. Shikai is no longer a query parser — it becomes the **user's personal knowledge base engine**.

### KB Data Structures

```rust
/// A user's personal Shikai knowledge base
pub struct KnowledgeBase {
    /// KB owner identity
    pub owner: String,
    /// Unique KB version hash
    pub version: String,
    /// Entities in this KB
    pub entities: HashMap<String, KBEntity>,
    /// Formulas in this KB
    pub formulas: HashMap<String, KBFormula>,
    /// Cross-entity/formula relationships
    pub relationships: Vec<KBRelationship>,
    /// Nakshatra connections
    pub nakshatra_connections: Vec<NakshatraConnection>,
    /// KB metadata
    pub metadata: KBMetadata,
}

pub struct KBEntity {
    pub id: String,
    pub name: String,
    pub domain: Graha,      // Primary graha domain
    pub rashis: Vec<Rashi>, // Secondary rashi associations
    pub nakshatras: Vec<Nakshatra>,
    pub tattvas: Vec<VedicElement>,
    pub gunas: Vec<Guna>,
    pub properties: HashMap<String, f64>,
    pub description: String,
    pub tags: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
    pub grounded: bool,      // Entity is connected to at least one formula
}

pub struct KBFormula {
    pub id: String,
    pub expression: String,  // meval-compatible
    pub inputs: Vec<String>,
    pub output: String,
    pub domain: Graha,
    pub tags: Vec<String>,
    pub description: String,
    pub evidence: String,
    pub level: u8,
    pub grounded: bool,      // All inputs have entity/formula mappings
    pub nand_eligible: bool, // Can be compiled to NAND
}

pub struct KBRelationship {
    pub from_id: String,
    pub to_id: String,
    pub relationship_type: GrahaRelationship,
    pub aspect: VedicAspect,
    pub confidence: f64,
    pub description: String,
}

pub struct KBMetadata {
    pub total_entities: usize,
    pub total_formulas: usize,
    pub total_relationships: usize,
    pub grounded_entities: usize,
    pub grounded_formulas: usize,
    pub floating_entities: usize,   // Entities with no connections
    pub floating_formulas: usize,   // Formulas with ungrounded inputs
    pub nand_ready_formulas: usize, // Formulas that compile to NAND
    pub resolution_score: f64,      // 0.0 = all floating, 1.0 = all grounded
    pub nand_coverage: f64,         // 0.0 = none NAND-ready, 1.0 = all NAND-ready
    pub graph_density: f64,         // Relationship count / possible connections
}
```

### KB Building Commands

```rust
impl KnowledgeBase {
    /// Add a new entity to the KB
    pub fn add_entity(&mut self, entity: KBEntity) -> Result<(), KBError>;
    
    /// Add a new formula to the KB
    pub fn add_formula(&mut self, formula: KBFormula) -> Result<(), KBError>;
    
    /// Connect two KB nodes with a relationship
    pub fn add_relationship(&mut self, rel: KBRelationship) -> Result<(), KBError>;
    
    /// Connect an entity to a nakshatra
    pub fn add_nakshatra_connection(&mut self, entity_id: &str, nak: Nakshatra, weight: f64);
    
    /// Remove an entity/formula and all its connections
    pub fn remove(&mut self, id: &str) -> Result<(), KBError>;
    
    /// Search the KB by keyword, domain, nakshatra, etc.
    pub fn search(&self, query: &SearchQuery) -> Vec<SearchResult>;
    
    /// Get KB completeness status
    pub fn status(&self) -> KBStatus;
    
    /// Export KB to TOML file
    pub fn export(&self) -> String;
    
    /// Import KB from TOML
    pub fn import(toml_str: &str) -> Result<Self, KBError>;
    
    /// Render KB as a graph (for visualization)
    pub fn graph(&self) -> KBGraph;
}
```

### KB Status Report

```rust
pub struct KBStatus {
    pub owner: String,
    pub version: String,
    pub resolution_score: f64,         // Overall resolution
    pub nand_coverage: f64,            // NAND compilation readiness
    pub can_release_bankai: bool,      // Threshold met
    pub reason: Vec<String>,           // Why Bankai can/can't be released
    
    pub entities: EntityStats,
    pub formulas: FormulaStats,
    pub relationships: RelationshipStats,
}

pub struct EntityStats {
    pub total: usize,
    pub grounded: usize,
    pub floating: usize,
    pub by_domain: HashMap<Graha, usize>,
    pub by_nakshatra: HashMap<Nakshatra, usize>,
    pub by_rashi: HashMap<Rashi, usize>,
}

pub struct FormulaStats {
    pub total: usize,
    pub grounded: usize,
    pub floating: usize,
    pub nand_ready: usize,
    pub by_domain: HashMap<Graha, usize>,
    pub by_level: HashMap<u8, usize>,
}
```

### Bankai Release Conditions

A KB is eligible for Bankai release when:

```
1. resolution_score >= 0.85    (85% of entities/formulas are grounded)
2. nand_coverage >= 0.70       (70% of formulas are NAND-ready)
3. connected_components <= 3    (KB is mostly a single graph, not 20 disconnected islands)
4. no_circular_deps == true     (formula dependency graph is acyclic)
5. entity_count >= 5            (minimum 5 entities)
6. formula_count >= 5           (minimum 5 formulas)
7. relationship_count >= 3      (minimum 3 relationships)
```

---

## 15. Bankai — Solved Atomic KB Form (NEW)

Bankai becomes the **compiled, solved form of the user's Shikai KB**. When a user's KB is sufficiently complete, they can "release Bankai" — the KB compiles to a NAND DAG and enters a solved state.

### KB Compiler

```rust
/// Compiles a user's Knowledge Base into a NAND DAG
pub struct KBCompiler {
    /// The source KB
    kb: KnowledgeBase,
    /// Compiled NAND DAG
    nand_dag: Option<NandDag>,
    /// Compilation artifacts
    artifacts: CompilationArtifacts,
}

impl KBCompiler {
    /// Compile the KB to NAND DAG
    pub fn compile(&mut self) -> Result<NandDag, CompileError> {
        // 1. Validate KB consistency
        self.validate()?;
        // 2. Build dependency graph of formulas
        let deps = self.build_dependency_graph();
        // 3. Topologically sort (detect cycles)
        let sorted = self.topological_sort(&deps)?;
        // 4. Compile each formula to NAND gates
        for formula_id in sorted {
            let nand_ops = self.compile_to_nand(formula_id)?;
            self.nand_dag.add_operations(nand_ops);
        }
        // 5. Verify compilation
        self.verify()?;
        Ok(self.nand_dag.clone().unwrap())
    }
    
    /// Verify KB consistency (no missing inputs, no type mismatches)
    pub fn validate(&self) -> Result<(), CompileError>;
    
    /// Verify the compiled NAND DAG is sound
    pub fn verify(&self) -> Result<(), CompileError>;
}
```

### BankaiSolve (Updated)

```rust
pub struct BankaiSolve {
    /// Original query
    pub query: String,
    /// Identity that requested
    pub identity: String,
    
    // ─── KB Info ──────────────────────────────────────────────
    /// KB version at solve time
    pub kb_version: String,
    /// KB hash (fingerprint)
    pub kb_hash: String,
    
    // ─── Solve Results ────────────────────────────────────────
    /// Chain result (if applicable)
    pub chain: Option<ChainResult>,
    /// Composition (if applicable)
    pub composition: Option<Composition>,
    /// Traversal (if applicable)
    pub traversal: Option<Traversal>,
    
    // ─── NAND DAG ─────────────────────────────────────────────
    /// The compiled NAND DAG (only for Bankai-tier users)
    pub nand_dag: Option<NandDag>,
    
    // ─── Validation ───────────────────────────────────────────
    /// Gate validation results
    pub validation: Vec<GateOutput>,
    /// Overall confidence [0, 1]
    pub confidence: f64,
    /// Whether this is the user's released Bankai
    pub is_released: bool,
}
```

### Bankai Release Ceremony

When a user's KB meets all release conditions, they can perform the **Bankai release**:

```bash
athena bankai release --name "my_bankai"
```

This triggers:
1. KB snapshot (frozen at this version)
2. Compilation to NAND DAG
3. KB hash stored as the user's Bankai fingerprint
4. User's AccessTier upgraded to Bankai
5. All future queries on this KB are solved at the NAND level
6. **The KB becomes read-only** — to modify, user must release a new version

```
🏆  B A N K A I   R E L E A S E D  🏆

  User:           nutypebuddha
  KB Name:        my_bankai  
  KB Hash:        7a3f9c2b1d8e4f6a0c2b5d7e9f1a3c8b
  Resolution:     92.3%
  NAND Coverage:  81.7%
  
  Compiled:       24 entities → 156 NAND gates
                   31 formulas → 892 NAND gates
                   17 relationships → 68 NAND gates
                   
  Total NAND DAG: 1,116 gates | depth: 14 | width: 37
  
  "The blade is no longer a blade — it is your 
   understanding, made atomic."
```

---

## 16. Shikai → Bankai Progression

The progression from Shikai to Bankai is the core gamification loop:

### Stage 1: Asauchi (Tier 0)
- User sees `athena info`, `athena wheel`, `athena validate`
- Read-only access to the built-in Vedic framework
- Cannot modify KB, cannot query deeper

### Stage 2: Zanpakuto (Tier 1)
- User runs `athena register <name>` to claim identity
- Can now ask queries, see results, explore the wheel
- NLP preprocessing is active, user sees parsed intents
- "Your Zanpakuto has a name. It knows you."

### Stage 3: Shikai (Tier 2) — BUILD
- User starts with an empty KB
- `athena shikai entity add` — add entities with properties
- `athena shikai formula add` — add formulas with expressions
- `athena shikai relate` — connect entities and formulas
- `athena shikai status` — see KB completeness
- `athena shikai search` — search own KB
- `athena shikai graph` — visualize KB

### Stage 4: Release (Threshold)
- KB reaches resolution > 85%, NAND > 70%
- `athena shikai status` shows release-ready
- `athena bankai release --name "..."` triggers compilation
- KB is frozen, compiled, and enshrined

### Stage 5: Bankai (Tier 3) — SOLVED
- User's KB is now a solved atomic system
- `athena solve "query"` uses the compiled NAND DAG
- Answers are provably true (within KB)
- `athena bankai status` shows the released form
- User can start a NEW Shikai (build a second KB)
- Multiple Bankais can coexist

### Visual Progression

```
Tier 0:  Asauchi  [░░░░░░░░░░] 0%   "Nameless blade"
Tier 1:  Zanpakuto[▓▓░░░░░░░░] 20%  "Named and known"  
Tier 2:  Shikai   [▓▓▓▓▓░░░░░] 50%  "Building your KB"
Tier 3:  Shikai   [▓▓▓▓▓▓▓▓░░] 80%  "Almost complete"
Tier 4:  BANKAI!  [▓▓▓▓▓▓▓▓▓▓] 100% "SOLVED"
```

---

## 17. User Journey: From Asauchi to Bankai

### Session 1: First Encounter

```bash
$ athena info
  Athena — Relational Intelligence Engine
  Layer: asauchi
  "Speak its name to awaken the Zanpakuto."

$ athena wheel
  ☉ Surya (0°) — Self & Leadership
  ☽ Chandra (40°) — Mind & Emotion
  ♂ Mangala (80°) — Action & Energy
  ☿ Budha (120°) — Logic & Communication
  ♃ Brihaspati (160°) — Wisdom & Law
  ♀ Shukra (200°) — Arts & Value
  ♄ Shani (240°) — Structure & Time
  ☊ Rahu (280°) — Innovation & Ambition
  ☋ Ketu (320°) — Spirituality & Science

$ athena register "seeker"
  ✓ Zanpakuto awakened.
  Your identity: seeker
  Access tier: Zanpakuto
  "The blade knows your name."
```

### Session 2: Building Knowledge

```bash
$ athena shikai entity add \
    --id "gravity" \
    --name "Gravity" \
    --domain "mangala" \
    --property "strength=9.81" \
    --tag "force" \
    --tag "physics"

  ✓ Entity added: gravity (Mangala domain)
  Resolution: 0.0% — add formulas to connect it

$ athena shikai formula add \
    --id "fall_time" \
    --expression "sqrt(2*height/g)" \
    --inputs "height,g" \
    --output "time" \
    --domain "mangala" \
    --tag "physics"

  ✓ Formula added: fall_time (Mangala domain)
  ⚠ Formula is floating — needs entity grounding for 'g'

$ athena shikai relate \
    --from "gravity" \
    --to "fall_time" \
    --type "provides" \
    --confidence 0.95

  ✓ Relationship: gravity → fall_time (confidence 0.95)
  Resolution: 12.5% — keep going!
```

### Session 10: Approaching Release

```bash
$ athena shikai status
  ┌─ Shikai KB Status ─────────────────────────────────┐
  │  Owner: seeker                                      │
  │  Resolution: 87.3% ████████████░░  ✓                │
  │  NAND Coverage: 72.1% █████████░░░░  ✓              │
  │  Connected: 2 components  ⚠ (merge recommended)     │
  │  Acyclic: ✓                                         │
  │  Entities: 12                                       │
  │  Formulas: 18                                       │
  │  Relationships: 24                                  │
  │                                                      │
  │  ★ BANKAI RELEASE READY ★                            │
  │  Run: athena bankai release --name "physics_v1"      │
  └──────────────────────────────────────────────────────┘

$ athena bankai release --name "physics_v1"
  🏆 BANKAI RELEASED: physics_v1 🏆
  Hash: a1b2c3d4e5f6...
  Compiled: 12 entities → 234 NAND gates
  18 formulas → 567 NAND gates
  Total: 801 gates

  Your understanding is now atomic.
```

### Session 11: Solving with Bankai

```bash
$ athena solve "how long to fall 100 meters on Earth"
  ┌─ BankaiSolve ──────────────────────────────────────┐
  │  Using: physics_v1 (released Bankai)                │
  │                                                      │
  │  Step 1: fall_time(height=100, g=9.81)              │
  │    = sqrt(2 * 100 / 9.81)                           │
  │    = sqrt(200 / 9.81)                                │
  │    = sqrt(20.387)                                    │
  │    = 4.515 seconds                                   │
  │                                                      │
  │  Confidence: 0.97 (NAND-verified)                    │
  │  Gate: MathGate ✓ | LogicGate ✓ | FormalGate ✓       │
  └──────────────────────────────────────────────────────┘
```

---

## 18. Implementation Phases

### Phase 0: Foundation (Week 1) — ✓ COMPLETED
- [x] Update `Domain` enum → `Graha` enum (9 variants)
- [x] Update `Sign` → `Rashi` (Vedic names, sidereal)
- [x] Add `Tattva` (5 elements + Akasha) if missing
- [x] Add `Purushartha` (4 life goals)
- [x] Add `GrahaRelationship` (Mitra/Shatru/Sama)
- [x] Update `VedicAspect` (Yuti, Saptama, Visesha Drishti, etc.)
- [x] Rewrite `EdgeTable` from 12×12 to 9×9
- [x] Update `WheelGraph` for 9 nodes
- [x] Update `Aspect::between()` for 9-graha distances
- [x] Tests: all wheel tests pass with 9 grahas

### Phase 1: Classification (Week 2) — ✓ COMPLETED
- [x] Rewrite `AtomClassification` → `VedicAtomClassification`
- [x] Vedic-first: grahas, rashis, nakshatras, tattvas, gunas, purusharthas
- [x] Update `ChangeSorter` keyword mappings for Vedic domains
- [x] Update `VedicClassification` as primary (not secondary)
- [x] Add merge/similarity methods for new axes
- [x] Tests: classification tests pass

### Phase 2: Descent & Gyro (Week 3) — ✓ COMPLETED
- [x] Rename `DescentLayer` variants to Vedic names
- [x] Update descent resolution for Graha/Rashi/Nakshatra layers
- [x] Update `GyroState` for 9 grahas × 40°
- [x] Update `GyroDynamics` constants
- [x] Update `Orientation::dominant_sign()` → `dominant_graha()`
- [x] Update `alignment_weights()` for 9-position cosine falloff
- [x] Tests: all descent + gyro tests pass

### Phase 3: Entities (Week 4) — 🟡 PARTIAL
- [x] Migrate existing entities to Vedic domains (renamed files, merged colliding grahas)
- [ ] Create `entities/navagraha.toml` (9 graha entities)
- [ ] Create `entities/27_nakshatras.toml`
- [ ] Add `nakshatra_connections` field to entity TOML
- [ ] Add `vedic_classification` field (graha/rashi/tattva/guna arrays)
- [ ] Update `SeedEntity` struct to include Vedic fields
- [ ] Update entity search to index Vedic fields
- [ ] Tests: all entity tests pass ✓

### Phase 4: Formulas (Week 5) — 🟡 PARTIAL
- [x] Rename formula files from Western to Vedic domain names
- [x] Rename domain/from/to fields to Vedic graha names
- [ ] Rewrite formula descriptions with Vedic framing
- [ ] Add nakshatra field to formula TOML
- [ ] Add `tattva`, `guna`, `purushartha` fields to formula metadata
- [x] Verify all expressions are still mathematically correct
- [x] Tests: all formula registry tests pass, regression tests pass ✓
- [x] Tests: verify known outputs still match ✓

### Phase 5: Zanpakuto NLP Expansion (Week 6)
- [ ] Move `Intent` enum from `shikai/` to `zanpakuto/`
- [ ] Move formula detection from Shikai to Zanpakuto NLP
- [ ] Move argument extraction from Shikai to Zanpakuto NLP
- [ ] Move level/cycle detection from Shikai to Zanpakuto NLP
- [ ] Expand `NlpContext` to contain all parsed data
- [ ] Add `identity_name` and `access_tier` to NlpContext
- [ ] Add graha-based domain scoring (replace sign-based)
- [ ] Tests: NLP tests pass, Shikai tests adapt

### Phase 6: Shikai KB Engine (Weeks 7-8) [CRITICAL]
- [ ] Create `src/shikai/kb/` module
- [ ] Implement `KnowledgeBase` struct and all CRUD operations
- [ ] Implement `KBEntity`, `KBFormula`, `KBRelationship`
- [ ] Implement `KBMetadata` and completeness tracking
- [ ] Implement KB search, graph export, TOML serialization
- [ ] Implement KB import/export
- [ ] Implement `KBCompiler` (preliminary — NAND compilation)
- [ ] Add CLI commands: `athena shikai entity add|remove|list`, etc.
- [ ] Add `athena shikai status` command
- [ ] Add `athena shikai graph` command (ASCII graph output)
- [ ] Update Shikai::process() to consult user KB before routing
- [ ] Tests: KB engine tests (100+ tests for CRUD, status, search)

### Phase 7: Bankai Release (Week 9)
- [ ] Implement `KBCompiler::compile()` — full NAND DAG compilation
- [ ] Implement `KBCompiler::validate()` — KB consistency checks
- [ ] Implement `KBCompiler::verify()` — NAND DAG soundness
- [ ] Implement release conditions (resolution, coverage, acyclic, etc.)
- [ ] Add `athena bankai release --name <name>` command
- [ ] Add `athena bankai status` command
- [ ] Frozen KB snapshot on release
- [ ] AccessTier upgrade on successful release
- [ ] Multiple Bankai support
- [ ] Tests: compilation tests, release tests

### Phase 8: MCP Update (Week 10)
- [ ] Update all 12 MCP tools for Vedic wheel
- [ ] Add `athena_shikai_kb_build` tool (entity/formula/relationship add)
- [ ] Add `athena_shikai_kb_status` tool
- [ ] Add `athena_bankai_release` tool
- [ ] Add `athena_bankai_solve` tool (Bankai-gated solve)
- [ ] Update `athena_wheel` for 9 grahas
- [ ] Update `athena_traverse` for Vedic aspects
- [ ] Update `athena_formula_search` for Vedic metadata
- [ ] Update `athena_entity_*` tools for Vedic fields
- [ ] Tests: all MCP integration tests pass

---

## 19. Migration Path

### Zero-Downtime Approach

The migration happens in stages with backward compatibility:

1. **Phase 0-2** (Vedic wheel + classification): Old Western code paths are deprecated but still compile with `#[allow(deprecated)]`. New Vedic types coexist.

2. **Phase 3-4** (Entities + formulas): Files are duplicated with `_vedic` suffix. Old files still load. `FormulaRegistry` and `EntityRegistry` load both.

3. **Phase 5** (Zanpakuto expansion): Old Shikai still works but is deprecated. New `Zanpakuto::process()` is the recommended path.

4. **Phase 6** (Shikai KB): Old `Shikai::process()` remains as a compatibility shim. New KB-based Shikai is the primary path.

5. **Phase 7** (Bankai release): Old Bankai solve remains. New KB-compilation path is added. Both work.

6. **Phase 8** (Cleanup): Deprecated types removed. `v0.1.x` compatibility flag for any remaining old code.

### Migration Commands

```bash
# Phase 0: Build feature flags
cargo build --release                     # Vedic (default after migration complete)
cargo build --release --features western  # Old Western wheel (temporary compat)

# Phase 3: Entity migration utility
athena migrate entities --from western --to vedic

# Phase 6: KB import from existing data
athena shikai import --from-entities entities/ --from-formulas formulas/
athena shikai import --bootstrap  # Creates starter KB from built-in data
```

---

## 20. Appendix: Vedic Category Reference

### 9 Grahas (Planets)

| # | Graha | Sanskrit | Domain | Tattva | Guna | Rashi(s) | Day |
|---|-------|----------|--------|--------|------|----------|-----|
| 0 | Surya | सूर्य | Self & Leadership | Fire | Sattva | Simha | Sun |
| 1 | Chandra | चन्द्र | Mind & Emotion | Water | Sattva | Karka | Mon |
| 2 | Mangala | मङ्गल | Action & Engineering | Fire | Rajas | Mesha, Vrishchika | Tue |
| 3 | Budha | बुध | Logic & Communication | Earth | Rajas | Mithuna, Kanya | Wed |
| 4 | Brihaspati | बृहस्पति | Wisdom & Law | Ether | Sattva | Dhanu, Meena | Thu |
| 5 | Shukra | शुक्र | Arts & Value | Water | Sattva | Vrishabha, Tula | Fri |
| 6 | Shani | शनि | Structure & Time | Air | Tamas | Makara, Kumbha | Sat |
| 7 | Rahu | राहु | Innovation & Tech | Air | Tamas | — (shadow) | — |
| 8 | Ketu | केतु | Spirituality & Science | Ether | Tamas | — (shadow) | — |

### 12 Rashis (Signs)

| # | Rashi | Sanskrit | Lord | Tattva | Guna | Purushartha |
|---|-------|----------|------|--------|------|-------------|
| 0 | Mesha | मेष | Mangala | Fire | Rajas | Artha |
| 1 | Vrishabha | वृषभ | Shukra | Earth | Tamas | Kama |
| 2 | Mithuna | मिथुन | Budha | Air | Rajas | Artha |
| 3 | Karka | कर्क | Chandra | Water | Sattva | Kama |
| 4 | Simha | सिंह | Surya | Fire | Sattva | Dharma |
| 5 | Kanya | कन्या | Budha | Earth | Tamas | Moksha |
| 6 | Tula | तुला | Shukra | Air | Rajas | Kama |
| 7 | Vrishchika | वृश्चिक | Mangala | Water | Tamas | Moksha |
| 8 | Dhanu | धनु | Brihaspati | Fire | Sattva | Dharma |
| 9 | Makara | मकर | Shani | Earth | Tamas | Artha |
| 10 | Kumbha | कुम्भ | Shani | Air | Sattva | Moksha |
| 11 | Meena | मीन | Brihaspati | Water | Sattva | Dharma |

### 27 Nakshatras (Lunar Mansions)

| # | Nakshatra | Lord | Shakti (Power) | Pada 1 Rashi | Symbol |
|---|-----------|------|----------------|-------------|--------|
| 0 | Ashwini | Ketu | Speed, healing | Mesha | Horse |
| 1 | Bharani | Shukra | Endurance, birth | Mesha | Yoni |
| 2 | Krittika | Surya | Cutting, purification | Mesha | Razor |
| 3 | Rohini | Chandra | Growth, beauty | Vrishabha | Cart |
| 4 | Mrigashira | Mangala | Seeking, searching | Vrishabha | Deer |
| 5 | Ardra | Rahu | Storm, transformation | Mithuna | Teardrop |
| 6 | Punarvasu | Brihaspati | Return, renewal | Mithuna | Bow |
| 7 | Pushya | Shani | Nourishment, growth | Karka | Flower |
| 8 | Ashlesha | Budha | Entanglement, healing | Karka | Serpent |
| 9 | Magha | Ketu | Ancestry, authority | Simha | Throne |
| 10 | PurvaPhalguni | Shukra | Love, enjoyment | Simha | Couch |
| 11 | UttaraPhalguni | Surya | Union, stability | Kanya | Bed |
| 12 | Hasta | Chandra | Skill, completion | Kanya | Hand |
| 13 | Chitra | Mangala | Beauty, art | Kanya | Pearl |
| 14 | Svati | Rahu | Independence, wind | Tula | Coral |
| 15 | Vishakha | Brihaspati | Purpose, achievement | Tula | Archway |
| 16 | Anuradha | Shani | Devotion, loyalty | Vrishchika | Lotus |
| 17 | Jyeshtha | Budha | Protection, courage | Vrishchika | Earring |
| 18 | Mula | Ketu | Depth, foundation | Dhanu | Root |
| 19 | PurvaAshadha | Shukra | Victory, purification | Dhanu | Fan |
| 20 | UttaraAshadha | Surya | Conquest, determination | Makara | Tusk |
| 21 | Shravana | Chandra | Listening, learning | Makara | Ear |
| 22 | Dhanishtha | Mangala | Prosperity, fame | Kumbha | Drum |
| 23 | Shatabhisha | Rahu | Healing, secrecy | Kumbha | Circle |
| 24 | PurvaBhadrapada | Brihaspati | Transformation, fire | Meena | Sword |
| 25 | UttaraBhadrapada | Shani | Purification, water | Meena | Twins |
| 26 | Revati | Budha | Journey, nourishment | Meena | Fish |

### 5 Tattvas (Great Elements)

| Tattva | Sanskrit | Quality | Guna | Sense | Action |
|--------|----------|---------|------|-------|--------|
| Ether (Akasha) | आकाश | Substrate, potential | Sattva | Hearing | Speech |
| Air (Vayu) | वायु | Movement, touch | Rajas | Touch | Grasping |
| Fire (Tejas) | तेजस | Transformation, form | Rajas | Sight | Walking |
| Water (Apas) | आपः | Flow, cohesion | Sattva | Taste | Reproduction |
| Earth (Prithvi) | पृथ्वी | Stability, structure | Tamas | Smell | Elimination |

### 3 Gunas (Primordial Qualities)

| Guna | Sanskrit | Direction | Color | Function |
|------|----------|-----------|-------|----------|
| Sattva | सत्त्व | ↑ Rise | White | Harmony, clarity, illumination |
| Rajas | रजस् | → Expand | Red | Activity, passion, movement |
| Tamas | तमस् | ↓ Descend | Black | Inertia, stability, form |

### 4 Purusharthas (Life Goals)

| Purushartha | Sanskrit | Meaning | Associated Graha |
|-------------|----------|---------|-----------------|
| Dharma | धर्म | Righteousness, duty | Surya, Brihaspati |
| Artha | अर्थ | Prosperity, wealth | Mangala, Shani |
| Kama | काम | Desire, pleasure | Chandra, Shukra |
| Moksha | मोक्ष | Liberation | Ketu, Budha (via knowledge) |

### Vedic Aspect Summary

| Aspect | Sanskrit | Arc Distance | Type | Quality |
|--------|----------|-------------|------|---------|
| Conjunction | Yuti | 0° (0 steps) | Universal | Unity |
| Sequential | Sama | 40° (1 step) | Universal | Direct flow |
| Division | Varga | 80° (2 steps) | Universal | Secondary |
| Trine | Trikona | 120° (3 steps) | Universal | **Harmonious** |
| Full Aspect | Saptama | 160° (4 steps) | **All grahas** | Opposing view |
| Jupiter Special | Guru Drishti | 5th, 7th, 9th | Jupiter only | **Expansive** |
| Saturn Special | Shani Drishti | 3rd, 7th, 10th | Saturn only | **Karmic discipline** |
| Mars Special | Mangala Drishti | 4th, 7th, 8th | Mars only | **Hidden action** |

---

> **"From nameless blade to solved universe — your knowledge, made atomic."**
>
> — Athena v0.2.0

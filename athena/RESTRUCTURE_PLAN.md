# Athena Restructure — Gyroscopic Zodiac Engine v2

## Core Insight

```
Formulas = atomic provable primitives (NAND gates, math ops)
Entities = every token the LLM generates (dynamic, runtime)
Wheel    = the sorting machine that routes tokens → formulas via signs
```

**Three things, one pipeline:**

```
LLM Token → Change Sorter → classified into Sign(s)
                              ↓
                          Sign informs which primitive formulas to apply
                              ↓
                          NAND Core evaluates formula on token's values
```

There is nothing else. No bridging formulas. No static entity registry. No precomputed edge table. The wheel SPINS in real time and the sign a token lands on determines which provable primitives fire.

---

## Layer 0: Primitive Formulas (atomic, provable)

**`src/primitive/`** ✅ DONE (37 tests)

Only atomic formulas that can be proven correct by truth table or Peano arithmetic:

| Gate | Expression | Proven By |
|------|-----------|-----------|
| `nand(a,b)` | `1 - a*b` | Truth table (Sheffer stroke) |
| `not(a)` | `nand(a, a)` | Truth table |
| `and(a,b)` | `not(nand(a, b))` | Truth table |
| `or(a,b)` | `nand(not(a), not(b))` | Truth table |
| `nor(a,b)` | `not(or(a, b))` | Truth table |
| `xor(a,b)` | `or(and(a, not(b)), and(not(a), b))` | Truth table |
| `xnor(a,b)` | `not(xor(a, b))` | Truth table |
| `implies(a,b)` | `or(not(a), b)` | Truth table |

**Math primitives** (same NAND DAG evaluator):

| Primitive | Expression | Notes |
|-----------|-----------|-------|
| `add(a,b)` | `a + b` | Built from NAND half-adders |
| `sub(a,b)` | `a - b` | Via NAND subtractor circuit |
| `mul(a,b)` | `a * b` | Via NAND multiplier |
| `div(a,b)` | `a / b` | Via NAND divider |
| `neg(a)` | `-a` | |
| `abs(a)` | `abs(a)` | |
| `sqrt(a)` | `sqrt(a)` | |

All math primitives are **optionally** NAND-compiled. For performance, they can use direct `f64` ops with the NAND DAG as the proof-of-correctness layer. The NAND DAG IS the audit trail.

**Key rule**: If a formula can't be expressed as a NAND DAG, it doesn't belong in Athena. The formula registry should be the minimum set of primitives — every complex expression is evaluated by composing primitives through the wheel, not by storing complex formulas.

---

## Layer 1: Entities = Runtime Tokens

**`src/entity/`** will be rewritten. Static TOML entities are replaced by **runtime entities** created from every token the LLM generates.

```
Token "mass"
  → Entity {
      id: "tok_7f3a2",
      text: "mass",
      classification: AtomClassification {
          signs: [0.0, 0.9, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0],
          // Taurus (physics) dominant, Libra trace (engineering)
          elements: [0.0, 0.85, 0.1, 0.05],
          // Earth dominant
          modalities: [0.1, 0.8, 0.1],
          // Fixed dominant
          ...
      },
      numeric_properties: {},  // set by context, parsed from adjacent tokens
  }
```

**What the entity system becomes:**

| Before | After |
|--------|-------|
| 65 static entities in TOML files | Dynamic entities created per token |
| Pre-defined constants (Mars mass, etc.) | Values extracted from token context |
| Entity → domain mapping | Entity classified by Change Sorter |
| Entity search by keyword | Entity = the token itself, no search needed |
| Entity-eval (formula grounded in entity) | Sign→primitive mapping IS the eval |

**The static entities file becomes optional** — a seed set of known entities that the system can recognize in training, but every novel token creates a new runtime entity.

**What remains from the old entity system**: The `Entity` struct itself (id, text, classification, properties) is still useful — it's just created dynamically instead of loaded from TOML.

---

## Layer 2: Change Sorter (Token → Sign)

**`src/astrology/`** ✅ DONE scaffolding (61 tests, 8 files)

The Change Sorter classifies every token across all 7 astrology axes simultaneously:

```
Token "force"
  → signs:  [0.0, 0.9, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0]
            ↑ Taurus (physics)         ↑ Libra (engineering)
  → elements: [0.1, 0.85, 0.0, 0.05]
  → modalities: [0.2, 0.75, 0.05]
  → rulers: [0.0, 0.0, 0.6, 0.0, 0.3, 0.0, 0.0]
  → houses: [0.0, 0.0, 0.2, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.3, 0.0, 0.0]
  → aspects: [0.0, 0.8, 0.5, 0.3, 0.1]
  → polarity: 0.7
```

The dominant sign determines which primitive formulas are most relevant.

---

## Layer 3: Gyroscopic Wheel (Sign → Formula Routing)

**`src/gyro/`** — TODO

Each sign has a set of "best primitive formulas" that fire when a token lands on that sign:

| Sign | Domain | Key Primitives |
|------|--------|---------------|
| ♈ Aries | Math & Logic | `add`, `sub`, `mul`, `div`, `nand`, `and`, `or`, `not` |
| ♉ Taurus | Physics | `nand`, `add`, `mul` (for F=ma, KE, momentum) |
| ♊ Gemini | Astronomy | `add`, `sub`, `sqrt` (orbital mechanics) |
| ♋ Cancer | Earth Science | `add`, `mul`, `div` (climate, ecology) |
| ♌ Leo | Biology | `add`, `mul` (populations, genetics) |
| ♍ Virgo | Economics | `add`, `mul`, `div` (prices, markets) |
| ♎ Libra | Engineering | `nand`, `add`, `mul`, `sqrt` (structures, circuits) |
| ♏ Scorpio | CS & AI | `nand`, `and`, `or`, `xor` (logic, algorithms) |
| ♐ Sagittarius | History | `add` (chronology, statistics) |
| ♑ Capricorn | Language | `nand`, `and` (grammar rules, parsing) |
| ♒ Aquarius | Philosophy | `nand`, `implies` (logic, ethics) |
| ♓ Pisces | Psychology | `add`, `mul` (statistics, models) |

The wheel's orientation (determined by accumulated token mass) gates which primitives fire with what weight. As tokens accumulate on Taurus, the Taurus primitives dominate. As the query shifts to Scorpio, the wheel precesses and Scorpio primitives activate.

**This IS the routing.** No graph edges, no precomputed paths. The gyroscopic orientation IS the routing table.

---

## Layer 4: Pipeline

```
LLM generates token "mass"
  ↓
Change Sorter classifies "mass" → dominant sign: ♉ Taurus (0.9)
  ↓
Taurus gate opens: add, mul, sub primitives become active
  ↓
Next token "5" → numeric value extracted, added to context
  ↓
Next token "velocity" → classified → ♉ Taurus (0.8) + ♊ Gemini (0.2)
  ↓
Wheel precesses slightly toward Gemini (velocity → astronomy/cosmology overlap)
  ↓
"kinetic_energy" → composed from mul(mul(mass, velocity), velocity) × 0.5
  ↓
Formula expressed as NAND DAG, evaluated, result returned
```

**No complex formulas.** `kinetic_energy` is not stored as a formula — it's composed from atomic primitives (`mul`, `add`) guided by the wheel. The expression `0.5 × mass × velocity²` is a composition graph that the wheel builds dynamically.

---

## Implementation Phases (Revised)

| Phase | What | Files | Tests |
|-------|------|-------|-------|
| **0** ✅ | NAND primitive core | `src/primitive/` | 37 |
| **1** ✅ | Astrology classification | `src/astrology/` | 61 |
| **2** 🔜 | Dynamic entity system | Rewrite `src/entity/` — tokens become entities at runtime, no static TOML required | Target: 20+ |
| **3** 🔜 | Gyroscopic wheel | `src/gyro/` — 3-axis gyro, dynamic sign→primitive mapping, precession | Target: 30+ |
| **4** 🔜 | Pipeline integration | Wire Change Sorter → Gyro → NAND Core into single streaming path | Target: 15+ |
| **5** 🔜 | Remove obsolete | Strip `src/gates/`, bridging/vortex formulas, `NonMathRegistry`, static EDGE_TABLE | — |

---

## What Gets Removed

| Component | Reason |
|-----------|--------|
| `src/gates/*` | Gates are NAND compositions — no separate gate layer needed |
| `formulas/bridging/*.toml` | Bridging is done dynamically by the wheel |
| `formulas/vortex/*.toml` | Vortex is done dynamically by the wheel |
| `formulas/nonmath/*.toml` | Grammar/code patterns → Change Sorter classification |
| `src/wheel/edges.rs` | Aspect moved to `src/astrology/aspects.rs` |
| `src/wheel/graph.rs` | Static EDGE_TABLE replaced by gyroscopic dynamics |
| Static entities (most of `entities/*.toml`) | Entities become runtime tokens |
| `NonMathRegistry` | Folded into Change Sorter |
| `src/gates/` directory | Entire module deleted |

---

## What Stays

| Component | Reason |
|-----------|--------|
| `src/formula/` | Still holds the atomic primitive registry (but simplified to only primitives) |
| `src/wheel/nodes.rs` | Domain enum still needed for sign↔domain mapping |
| `src/bankai/eval.rs` | Still evaluates NAND expressions (but via primitive module) |
| `src/mcp/` | Still exposes tools |
| `src/asauchi/` | CLI still works |
| `src/zanpakuto/` | Access control still works |
| `src/shikai/` | Query parsing still works (but simplified — no domain inference) |

---

## Success Criteria (Revised)

| Criterion | How to Verify |
|-----------|---------------|
| NAND functionally complete | All 7 gates pass truth table tests ✅ (37 tests) |
| Change Sorter classifies any token | `athena classify "kinetic energy"` returns weighted AtomClassification ✅ (61 tests) |
| Gyroscopic wheel maintains determinism | Same token stream → same gyro state |
| Dynamic sign→primitive routing | Token classified to Taurus → `add`, `mul` available; to Scorpio → `nand`, `xor` available |
| No static entity TOML required | System creates entities from raw token stream at startup |
| All old concepts expressible | A query routed through wheel + primitives produces same result as old bridging formula system |
| Real-time streaming works | Tokens flowing through Change Sorter → Gyro → NAND Core in a single pass |

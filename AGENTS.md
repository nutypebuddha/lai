# Laverna — Agent Instructions

Vedic reasoning engine reboot. 4-layer architecture:
**Asauchi** → **Zanpakuto** → **Shikai** → **Bankai**.
NAND gate primitives at the bottom. Determinism-first.

## Environment
- aarch64 Linux; check disk before building: `df -h / | tail -1`
- `CARGO_BUILD_JOBS` is NOT hardcoded — set per-invocation
- `/sdcard` is vfat FUSE: no symlinks, no exec bits, use `cp`

## Dev cycle
```bash
cargo clippy -- -D warnings && cargo test --lib && cargo fmt -- --check
```

## CI gate order
`fmt --check` → `clippy -D warnings` (default + `--features llm`) →
`cargo deny check` → `cargo test` → `cargo test --features llm --lib` → `cargo audit`

## Build & features
```bash
cargo build --release                                     # native
cargo build --release --target x86_64-unknown-linux-musl --no-default-features
```
| feature | enables | default |
|---------|---------|---------|
| `mcp` | rmcp + tokio JSON-RPC server | no |
| `websearch` | ureq (World Bank stats) | via `mcp` |
| `budget` | token budget tracking | no |
| `bench` | criterion harness | no |
| `llm` | llama-gguf local LLM backend | no |
| `portable` | embed corpus in binary | no |

## Architecture
- **Layer 0 — Primitive**: `src/primitive/`, `src/descent/`, `src/gyro/`
- **Layer 1 — Asauchi**: `src/asauchi/`, `src/formula/`, `src/entity/`, `src/ephemeris/`, `src/chart/`
- **Layer 2 — Zanpakuto**: `src/zanpakuto/`, `src/shikai/`
- **Layer 3 — Bankai**: `src/bankai/`, `src/mcp/`

Pipeline: query → zanpakuto_nlp → descent_engine → shikai_process → bankai_solve

## Naming Conventions (GNU/UNIX Pure Function Style)

All functions must be **pure**: no side effects, deterministic, all inputs as
parameters, all outputs as return values. No global state.

### Functions
- `snake_case` — always
- **Verb-first**: `compute_*`, `evaluate_*`, `validate_*`, `transform_*`, `encode_*`, `decode_*`
- **Module prefix** when disambiguation needed: `nand_gate()`, `nand_not()`, `nand_and()`
- **No abbreviations**: `accumulator` not `acc`, `left_operand` not `lhs`
- **Bool predicates**: `is_*`, `has_*`, `can_*`

### Types
- `PascalCase` — always
- **Suffix by role**: `*Registry`, `*Engine`, `*Gate`, `*Result`, `*Config`

### Constants
- `SCREAMING_SNAKE_CASE`

### Modules
- `snake_case` — single word preferred

### Example
```rust
/// Pure function: NAND gate. Universal gate — all others derive from this.
pub fn nand_gate(left_input: bool, right_input: bool) -> bool {
    !(left_input && right_input)
}

/// Pure function: Half adder. Returns (sum, carry).
pub fn half_adder(left_operand: bool, right_operand: bool) -> (bool, bool) {
    let sum = xor_gate(nand_gate(left_operand, right_operand), or_gate(left_operand, right_operand));
    let carry = and_gate(left_operand, right_operand);
    (sum, carry)
}
```

## Conventions
- Formulas, not facts: encode relationships, not static lookups
- Cross-domain by default: new formulas reference ≥2 grahas
- Commits: Conventional Commits (`feat(wheel):`, `fix(bankai):`)
- Errors: `anyhow` at call sites, `thiserror` for library types

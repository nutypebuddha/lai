# Athena — Agent Instructions

## Session Log

### 2026-07-07 — AUDIT-001 + TICKET-005 (fixed 6/6 tickets)

Fixed 4 bugs + triaged ~42 variable-name collisions:

| Ticket | What | Where |
|--------|------|-------|
| TICKET-003 | Renamed `photosynthetic_efficiency` output `quantum_yield` → `photosynthetic_quantum_yield` | `formulas/atomic/leo_bio.toml` |
| TICKET-001 | Added `marginal_likelihood` formula to derive `evidence` from `likelihood`+`prior`+`false_positive_rate` | `formulas/atomic/sagittarius_history.toml` |
| TICKET-004 | Reject inf/NaN from eval results | `src/bankai/eval.rs` |
| TICKET-002 | Variable-name collision warning in `reason` planning path | `src/main.rs` |
| AUDIT-001 | Renamed 7 collision-causing outputs: `velocity`→`reaction_velocity`, `alpha`→`cronbach_alpha_coefficient`, `growth_rate`→`complexity_growth_rate`, `z`→`z_statistic`, `r`→`r_coefficient`, `frequency`→`word_frequency`, `error`→`sampling_error` | `formulas/` (5 files) |
| TICKET-005 | `--args` JSON rejection with helpful error; `entity-get` positional `id` | `src/main.rs` (2 changes) |

All 364 tests pass, clippy/fmt clean. Release binary exported to sdcard.

### 2026-07-07 — MCP classify/gyro tools + CLI classify command

Added `athena_classify` and `athena_gyro` MCP tools, plus the `athena classify <token>` and `athena gyro` CLI commands.

| Addition | What | Where |
|----------|------|-------|
| MCP tool | `athena_classify` — token classification across all 7 axes | `src/mcp/tools.rs` + `mod.rs` |
| MCP tool | `athena_gyro` — gyroscopic wheel state inspection | `src/mcp/tools.rs` + `mod.rs` |
| CLI command | `athena classify <token>` — colorized astrology classification | `src/main.rs` |
| CLI command | `athena gyro` — spinning wheel state display with mass bars | `src/main.rs` |
| Getter | `Bankai::change_sorter()`, `gyro_state()`, `gyro_state_mut()` | `src/bankai/mod.rs` |
| Tool count | 12 → 14 tools in ToolRegistry | `src/mcp/tools.rs` |

All 380 tests pass, clippy/fmt clean. Release binary exported to sdcard.

| Ticket | What | Where |
|--------|------|-------|
| TICKET-003 | Renamed `photosynthetic_efficiency` output `quantum_yield` → `photosynthetic_quantum_yield` | `formulas/atomic/leo_bio.toml` |
| TICKET-001 | Added `marginal_likelihood` formula to derive `evidence` from `likelihood`+`prior`+`false_positive_rate` | `formulas/atomic/sagittarius_history.toml` |
| TICKET-004 | Reject inf/NaN from eval results | `src/bankai/eval.rs` |
| TICKET-002 | Variable-name collision warning in `reason` planning path | `src/main.rs` |
| AUDIT-001 | Renamed 7 collision-causing outputs: `velocity`→`reaction_velocity`, `alpha`→`cronbach_alpha_coefficient`, `growth_rate`→`complexity_growth_rate`, `z`→`z_statistic`, `r`→`r_coefficient`, `frequency`→`word_frequency`, `error`→`sampling_error` | `formulas/` (5 files) |
| TICKET-005 | `--args` JSON rejection with helpful error; `entity-get` positional `id` | `src/main.rs` (2 changes) |

All 364 tests pass, clippy/fmt clean. Release binary exported to sdcard.

Single-crate Rust reasoning engine (clap CLI + MCP server). Formulas as TOML loaded at runtime.
Cross-domain reasoning via a 12-node zodiac symbolic wheel with 5 aspect types and 7-layer token descent.

## Commands

```
# dev cycle (must run in this order)
cargo clippy -- -D warnings && cargo test && cargo fmt -- --check

# single test / focused verification
cargo test --test <name>                        # integration test (see tests/*.rs)
cargo test --lib <module>::<tests_module>       # unit test subset
cargo test --lib formula::registry::tests       # after editing formulas/
cargo test --lib                                # unit tests only (fast)

# build
cargo build --release                           # aarch64 native, ~90s cold
cargo build --release --features budget         # +token budget tracking

# cross-compile (linker in .cargo/config.toml)
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-musl --no-default-features

# bench (3 criterion benches)
cargo bench

# run
./target/release/athena info                    # system info
./target/release/athena shikai "query"          # NLP + descent + gyro + formula match
./target/release/athena solve "query"           # full pipeline to computed answer
./target/release/athena search <keyword>        # formulas + entities
./target/release/athena descent "query"         # 7-layer token descent matrix
./target/release/athena mcp                     # stdio JSON-RPC server (default feature)
./target/release/athena reason --have a,b --want z  # BFS path-finding
./target/release/athena entity-get <id>          # entity lookup (positional)
./target/release/athena entity-search <keyword>  # search entities
./target/release/athena classify <token>         # 7-axis astrology classification
./target/release/athena gyro                     # gyroscopic wheel state (orientation, precession, mass distribution)
```

## Architecture

```
Asauchi → Zanpakuto → Shikai → Bankai
(CLI)     (access)    (parse)  (compute)
```

**Pipeline (wired end-to-end):**
```
Raw query → NLP (Zanpakuto) → DescentEngine::resolve_nlp() → SettlingMatrix
                                                                ↓
                                                       GyroState::apply_matrix()
                                                                ↓
                                                       GyroState::update()
                                                                ↓
                                                       Shikai::process_with_context()
                                                                ↓
                                                       Bankai::solve()
```

### Descent (7 layers per token)

```
Macro(depth 0) → Domain(1) → Aspect(2) → Element(3) → Formula(4) → Entity(5) → NAND(6)
```

Each token sinks through layers until it can't resolve further. Tokens at NAND are provably true at gate level. The `SettlingMatrix` aggregates all tokens and computes aspects between each pair, a layer histogram, and both Western + Vedic (graha/guna/bhūta) classifications.

### Gyro (spinning zodiac wheel)

Physical model: 12 signs at 30° intervals. Token mass applies torque via `apply_matrix()` causing precession. Gyro orientation influences formula alignment. `GyroState` lives in `main.rs` and gets updated per-query.

### Other

- 12 domains: Aries(Math)→Pisces(Psychology). Aspects: Conjunction(0°), Sextile(60°), Trine(120°), Square(90°), Opposition(180°).
- Formula types: Atomic (1 domain), Bridging (2 domains via `from`/`to`), NonMath (grammar/code/logic). Vortex dir is empty.
- MCP: 14 tools, `athena mcp`, default feature `mcp` (needs `rmcp`+`tokio`). Disable with `--no-default-features`.
- `entities/` files serve both Athena and the root `/root/entities/` collection. Modifying entities changes both.

## Formula System

TOML in `formulas/atomic/` (12 files). **No recompile** for formula edits — `build.rs` watches `formulas/` for fast re-link.

```toml
[[formula]]
id = "add"
domain = "aries"
inputs = ["a", "b"]
output = "sum"
expression = "a + b"   # meval-compatible
```

- `bridging/`: 4 files, each formula has `from`/`to` domains + `aspect`.
- `nonmath/`: 3 files (grammar, code patterns, logic rules).
- **Parse errors = panic at startup.** Always run `cargo test --lib formula::registry::tests` after editing formula files.
- Duplicate formula IDs warn but don't crash (last one wins).
- `build.rs` embeds a build timestamp — two sequential builds produce different binaries.

## Test Suite (~364 tests: 327 unit + 37 integration)

| File | Count | Run when |
|------|-------|----------|
| `tests/formula_registry_integration` | 10 | After formula edits |
| `tests/bankai_integration` | 13 | After bankai/compute changes |
| `tests/formula_regression` | 2 | After formula changes (known outputs) |
| `tests/mcp_integration` | 8 | After MCP changes |
| `tests/wheel_integration` | 4 | After wheel/graph changes |

## CI Pipeline (`.github/workflows/ci.yml`)

`fmt --check` → `clippy -D warnings` → `cargo deny check` → `cargo test` → `cargo bench`.
Tag push (`v*`): builds x86_64-musl, aarch64-musl (both `--no-default-features`, no MCP), x86_64-gnu (with MCP) + GitHub Release.

## Export (Android/proot environment)

```bash
# check disk before builds
df -h / | tail -1          # abort if < 2 GB free

# native build + export
cargo build --release
cp target/release/athena /sdcard/Download/athena-export/athena-aarch64

# x86_64 cross-compile + export
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/athena /sdcard/Download/athena-export/athena-x86_64

# source snapshot
zip -r /sdcard/Download/athena-export/athena-src-$(date +%Y%m%d).zip \
  Cargo.toml Cargo.lock rust-toolchain.toml build.rs \
  Dockerfile docker-compose.yml .dockerignore \
  src/ formulas/ entities/ tests/ benches/ \
  -x "src/**/target/*" -x "target/*"

# warnings
- sdcard is vfat-like FUSE: no symlinks, no exec bits, no chmod +x. Use cp, never cp -a.
- don't build inside /sdcard (slow, no exec).
- unzip on Android may strip exec bits — always cp binaries directly.
- vfat duplicates files with -1, -2 suffixes on name collision.
```

## Known Footguns

- **`clap` panics on unknown subcommands** — no suggestion. Use `--help`.
- **Formula TOML parse errors = opaque panic at startup.** Test after every formula edit.
- **`--args` must be `key=value`** — JSON `--args '{"x":1}'` now returns a helpful error message listing the correct format. No trailing comma, no spaces around `=`.
- **MCP vanished if built `--no-default-features`.** The `athena mcp` subcommand is gone entirely.
- **`deny.toml` is a real CI gate.** `cargo deny check` runs in CI (advisories, licenses, bans). Keep synced.
- **`entity-get` takes positional `id`** — `athena entity-get my_id`, not `athena entity-get --id my_id`.
- **`cargo test` includes empty doc-test targets.** The `ok. 0 passed` blocks are normal.
- **`entities/` TOML** shares the same files with `/root/entities/`. Verify both registries load.

## Config Files

| File | Purpose |
|------|---------|
| `.cargo/config.toml` | Linker per target (aarch64, armv7, x86_64) |
| `deny.toml` | License/advisory/bans for CI |
| `.pre-commit-config.yaml` | Local hooks: fmt, clippy, machete, typos |
| `.mcp.json` | MCP client integration config |
| `rust-toolchain.toml` | Channel + targets (includes x86_64 for cross-compile) |

## Release Profile

`lto = "fat"`, `codegen-units = 1`, `strip = "symbols"`, `opt-level = 3`, `panic = "abort"`.

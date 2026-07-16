# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Dogfood Athena first

Athena's MCP server is registered at user scope and her tools (`mcp__athena__*`) are pre-approved —
no permission prompts. **Reach for her before doing the work yourself**: dogfooding finds bugs, and
her responses are token-compacted.

- Any arithmetic, unit chain, or derivation → `athena_reason` (with `execute:true`) or
  `athena_evaluate`, not mental math.
- Checking a claim or expression → `athena_validate` before trusting it.
- "Does a formula/entity for X exist?" → `athena_formula_search` / `athena_entity_search`, not
  grepping the TOML files (registry search also exercises the collision-prone load path).
- Understanding graph structure while editing wheel/domain code → `athena_wheel`,
  `athena_traverse`, `athena_entity_aspect`.
- If MCP tools are unavailable (stale session, `--no-default-features` build), fall back to the
  CLI: `athena solve/search/traverse/...` (`/root/.cargo/bin/athena`) — also pre-approved.
- Responses are compact by default; pass `detail:"full"` only when the trimmed fields matter.
  At the end of a work session, `athena_savings` shows what the compaction layer saved.
- When a tool returns something wrong or surprising, that's a finding — file/fix it, don't route
  around it silently.

**After rebuilding `target/release/athena`, the running MCP server still serves the old binary —
its results reflect the code from when the session started.** Verify changes with the freshly
built CLI binary; MCP output only reflects a rebuild after the server reconnects (`/mcp`) or a new
session starts.

## Environment

Running on **Android inside proot** (aarch64, tight disk). See `AGENTS.md` in this directory for
the full environment/build/sync playbook — it is kept up to date and takes priority over this file
for anything disk-space, sdcard-sync, or Android-specific. The highlights:

- Check free space before release builds: `df -h / | tail -1` (abort/`cargo clean` if < 2GB free).
- `entities/` at the repo root (`/root/entities/`) and `/root/athena/entities/` are the same files —
  editing one affects both.
- Native target is `aarch64-unknown-linux-gnu`; no cross-compilation needed locally.

## Commands

```bash
cargo build --release              # release binary at target/release/athena
cargo test                         # full suite (unit + integration)
cargo test --lib                   # unit tests only, fast
cargo test --test wheel_integration     # one integration test file
cargo test --lib formula::registry::tests   # one module's tests
cargo clippy -- -D warnings         # must pass clean (CI-enforced, RUSTFLAGS=-D warnings)
cargo fmt -- --check                # formatting check (CI-enforced)
cargo bench                         # criterion: formula_search, wheel_traversal, shikai_processing
cargo build --release --features budget   # +token budget tracking feature
```

Run in this order after any change: `cargo clippy -- -D warnings && cargo test && cargo fmt -- --check`
— clippy is fastest and catches mistakes before burning time on the full test run.

CI (`.github/workflows/ci.yml`) additionally runs `cargo deny check` (licenses/bans/advisories) and
`cargo audit`, and cross-builds musl targets with `--no-default-features` (no MCP). Pre-commit hooks
(`.pre-commit-config.yaml`) run fmt, clippy, `cargo-machete` (unused deps), and `typos`.

Default feature `mcp` (rmcp + tokio) enables the MCP server; building with `--no-default-features`
means the `mcp` CLI subcommand will fail with a missing-subcommand error.

## Architecture

Athena is a deterministic, offline reasoning engine: no facts are stored, only *formulas* (the
relationships between values) and a graph that routes a query across domains to compose them.
Everything is designed to be provable/reproducible — no randomness, no ML in the core path.

### The 4 CLI-facing layers

```
Asauchi (src/asauchi)   → public CLI entry surface (info, validate)
Zanpakuto (src/zanpakuto) → identity/access tier + NLP preprocessing (tokenize/stem)
Shikai (src/shikai)     → NL query parsing: intent, domain, formula, entity resolution
Bankai (src/bankai)     → compute engine: eval, chain, compose, traverse, solve, search
```
`src/main.rs` is a single clap `Commands` enum (~30 subcommands) dispatching into these layers —
start there to see the full surface (`eval`, `chain`, `compose`, `traverse`, `wheel`, `search`,
`reason`, `pipeline`, `classify`, `gyro`, `descent`, `entity-*`, `budget*`, `mcp`, ...).

### Formulas and entities are external data, not code

`formulas/{atomic,bridging,vortex,nonmath}/*.toml` and `entities/*.toml` are loaded at runtime by
`src/formula/registry.rs` and `src/entity/mod.rs` — editing a `.toml` file needs no recompile logic
change, just a rebuild (see `build.rs`, which embeds a timestamp and re-links on any change under
`formulas/`). A malformed TOML panics at startup with no graceful recovery — after touching either
directory, always run:
```
cargo test --lib formula::registry::tests
cargo test --lib entity::registry::tests
cargo test
```

### Important: mid-migration from Western zodiac to Vedic grahas

The wheel's domain model is **actively being migrated** (see `VEDIC_ALIGNMENT.md`, the design doc,
and `RESTRUCTURE_PLAN.md`, a separate rewrite plan) and the migration is only partially applied —
code, docs, and data disagree with each other right now:

- `src/wheel/nodes.rs`: **`Domain` is now a type alias for `Graha`** (`src/astrology`), a **9-node**
  Vedic planet wheel (Surya, Chandra, Mangala, Budha, Brihaspati, Shukra, Shani, Rahu, Ketu) at 40°
  intervals — not the 12-sign zodiac wheel (Aries–Pisces) that `README.md` and the original
  `AGENTS.md` prose still describe.
- `formulas/atomic/*.toml` have mostly been renamed/re-tagged to graha domains (e.g.
  `mangala_math.toml`, `domain = "mangala"`), but some duplicate old zodiac-named files caused a
  formula/entity **ID collision bug** — see `ATHENA-COLLISION-TICKET.md` for the diagnosis (260
  formulas silently overwritten because two files declared the same formula `id` under different
  domain tags) before trusting formula counts.
- `entities/*.toml` at the repo root are **still zodiac-named** (`aries.toml`, `pisces.toml`, ...)
  and have not been migrated to grahas.
- `src/primitive/` (NAND-gate provable primitives), `src/astrology/` (7-axis Western + Vedic token
  classification), `src/gyro/` (gyroscopic sign→primitive routing), and `src/descent/` (7-layer
  token descent: Macro→Domain→Aspect→Element→Formula→Entity→NAND) are newer modules building toward
  the `RESTRUCTURE_PLAN.md` vision (route tokens through the wheel to atomic NAND primitives instead
  of static formula lookup). `src/gates/` (math/logic/fact/confidence/formal validation) is old-model
  and slated for removal once that migration completes — don't assume it's permanent architecture.

**When in doubt about which domain model applies, check the code (`src/wheel/nodes.rs`,
`src/astrology/`) over the prose in `README.md`/`AGENTS.md` — the docs lag the graha migration.**
Test counts and formula/entity counts quoted in `README.md` and `AGENTS.md` are stale; get current
numbers from `cargo test` output directly rather than citing the docs.

### Layer tiers within formulas

- **Atomic** (`formulas/atomic/`): single-domain, 0-hop (e.g. `F = ma`).
- **Bridging** (`formulas/bridging/`): 2-domain, 1-hop composition.
- **Vortex** (`formulas/vortex/`): multi-domain spirals, 3+ hops.
- **Nonmath** (`formulas/nonmath/`): grammar/code/logic patterns treated as first-class formulas.

### Validation gates

`src/gates/` (math, logic, fact, confidence, formal) independently verify a claim/expression before
it's trusted — this is what lets Athena catch "confidently wrong" numbers rather than just computing
them (see `athena-bug-tickets-2026-07-06.md` for a concrete example: `bayes_theorem` accepting an
internally-inconsistent hand-supplied marginal likelihood with no gate catching it).

### MCP server

`src/mcp/` exposes the engine over stdio JSON-RPC (`athena mcp`, wired via `.mcp.json`). Tool list
lives in `src/mcp/tools.rs`; keep it in sync with the CLI subcommands it wraps — it's a second
surface for the same Bankai/entity/budget operations, not a separate implementation.

Responses over the MCP wire are **compacted by default** to save the calling model's context
tokens (`src/mcp/compact.rs`): lists cap at `limit` (default 25, `omitted` marks truncation),
descriptions lose the `" — ..."` cosmological flavor suffix, `null` fields are pruned, and JSON is
emitted un-pretty-printed. `detail: "full"` on any tool call bypasses compaction. `handle_request`
itself still returns full data — compaction happens only in `server.rs::call_tool_sync`, so
integration tests against `handle_request` see complete responses. The `athena_savings` tool
reports estimated tokens saved this session (baseline pretty-full vs emitted). With
`--features budget`, `athena_budget_stats`/`athena_budget_reset` expose the `TokenBudget` wired
into `AthenaMCP::with_context`.

## Conventions (from CONTRIBUTING.md)

- **Formulas, not facts** — a new static lookup value should usually be a formula instead.
- **Determinism first** — no randomness, no ML, no non-deterministic dependencies in the core path.
- **Cross-domain by default** — every new formula should reference at least one other domain.
- Conventional Commits style (`feat(wheel): ...`, `fix(bankai): ...`, `docs(api): ...`).
- Prefer `anyhow` for error propagation at call sites; `thiserror` for library error types.

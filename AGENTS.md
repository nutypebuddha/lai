# L.ai — Agent Instructions

Offline, deterministic, fail-loud verification umbrella for AI. **One project,
four functions.** All code lives in this single repository (`nutypebuddha/lai`).

> Persona: derived from a real chart for 1994-04-14 20:09 CDT, Saint Croix Falls
> WI — Libra asc + Jupiter, Saturn in Aquarius, Sun Aries, Moon Taurus. Measured,
> structural, plain-spoken; refuses to guess. *Verify, don't trust.*

## Workspace

| Function | Path | Crate | Lang |
|----------|------|-------|------|
| L.ai · Proof | `proof/` | `laverna` | Rust |
| L.ai · Gate | `gate/` | `lai-gate` (+ `lai-gate-wasm`) | Rust / WASM |
| L.ai · Athena | `athena/` | `athena` | Rust |
| L.ai · Bridge | `bridge/` | — | Node/TypeScript |

The root `Cargo.toml` is a virtual workspace: `members = [proof, gate, athena,
proof/laverna-wasm, gate/cid-wasm]`.

## Environment
- aarch64 Linux, proot-distro Debian on Android/Termux
- Workspace root: `/root/Laverna/`
- `CARGO_BUILD_JOBS` is NOT hardcoded — set per-invocation
- `/sdcard` is bind-mounted from `/storage/emulated/0`

## Dev cycle (whole workspace)

```bash
cargo fmt -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Per-function work happens inside the member dir (e.g. `cd proof`). Proof-specific
agent rules (architecture, determinism rule, naming conventions) live in
`proof/AGENTS.md`.

## CI gate order
`cargo fmt -- --check` → `cargo clippy --workspace --all-targets -- -D warnings` →
`cargo deny check` → `cargo test --workspace`. Tagged releases build a static
x86_64-musl `laverna` binary + bundle `bin/llama/`.

## Brand
- Public mark: **L.ai**. Tagline: **Verify, don't trust.**
- `Laverna` is the internal *code name* for L.ai · Proof (per `proof/docs/brand.md`).
- Sister functions: L.ai · Gate (formerly CID), L.ai · Bridge (formerly
  CID-Bridge), L.ai · Athena (formerly Athena-). All merged + relicensed
  Apache-2.0 into this repo.

## License
Apache-2.0, sole author `nutypebuddha`. `Cargo.toml` license fields, `LICENSE`,
and `NOTICE` must stay in agreement. New source files open with:
```rust
// Copyright 2026 nutypebuddha
// SPDX-License-Identifier: Apache-2.0
```

## Conventions
- Functions pure: no global state, deterministic, all inputs as params.
- Determinism rule: sort any HashMap/unordered output by a stable key before
  printing (correctness, not style).
- Commits: Conventional Commits (`feat(gate):`, `fix(proof):`).
- Errors: `anyhow` at call sites, `thiserror` for library types.

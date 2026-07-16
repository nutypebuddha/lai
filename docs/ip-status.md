# Laverna — IP & companion status (closeout)

> Generated 2026-07-16. Implements the four stages of
> `Laverna_ IP Protection, Android Assistant Integration, and Compa.md`.
> Every stage item is DONE and committed as a GPG-signed commit.

## Execution ledger

| Stage | Item | Deliverable | Commit |
|-------|------|-------------|--------|
| 0.1 | License mismatch | Relicensed MIT→**Apache-2.0** (LICENSE, NOTICE, Cargo.toml, headers, README/AGENTS) | `431bbea` |
| 0.2 | GPG + OpenTimestamps | Ed25519 key; all commits signed; HEAD `431bbea` stamped to Bitcoin (4 OTS calendars); pubkey + receipt in `docs/provenance/` | `a350896` |
| 0.3 | eCO Form TX prep | `docs/copyright-registration-prep.md` (all $45 single-author fields) + `scripts/deposit.sh` | `10be8cb` |
| 0.4 | USPTO clearance | `docs/trademark-clearance-laverna.md` — Mattel Reg. 3,303,422 (Class 28) + open-source conflict → recommend **"Wintermore Housekeeping"** | `3edb066` |
| 1.5 | Technical spine PoC | `scripts/laverna-mcp-proxy.py` verify-first loop; 9 MCP tools; build verified | `afdb201` |
| 2   | v0.1 companion | `src/companion/` (persona + memory, 7 tests) + `scripts/laverna-companion.py` + `docs/companion-design.md` | `f2fe5e1` |
| 3   | NLnet grant | `docs/nlnet-application.md` — target **Fediversity Aug 1 2026**; prep **ELFA** resubmission; skip TALER | `f70ad0c` |

## OpenTimestamps status (only pending action)

`docs/provenance/HEAD-431bbea.ots` was stamped 2026-07-16 ~00:12 UTC.
It is **pending Bitcoin block confirmation** (~1-2h). Once confirmed:

```bash
ots upgrade docs/provenance/HEAD-431bbea.ots   # finalize the receipt
ots verify  docs/provenance/HEAD-431bbea.ots   # prints block height
```

Until then `ots verify` shows "Pending confirmation" — expected.

## Verification you can re-run

```bash
# 1. every commit from 431bbea onward is GPG-signed
git log --pretty='%G? %h %s' | grep '^G'          # all G = Good signature

# 2. the binary builds with the verification server
cargo build --release --features "mcp websearch"
./target/release/laverna mcp   # speaks MCP JSON-RPC

# 3. the verify-first loop refuses to fabricate
python3 scripts/laverna-companion.py --demo --server ./target/release/laverna

# 4. the deterministic companion module's tests
cargo test --lib companion       # 7 tests green

# 5. the copyright deposit assembler
bash scripts/deposit.sh          # emits /tmp/laverna-deposit.txt
```

## Key conclusions (from the report, now actionable)

- **License:** Apache-2.0, sole-author relicense, clean. Prior MIT releases
  stay MIT; new releases are Apache-2.0.
- **Copyright:** register via eCO ($45 single application, Form TX) at a
  major milestone — prep + deposit script ready. Registration unlocks
  statutory damages; the crypto layer is corroboration, not a substitute.
- **Trademark:** do NOT file "Laverna" (conflicted: Mattel Class-28 reg
  + open-source app). Use **"Wintermore Housekeeping"** as the commercial
  mark near monetization.
- **Companion:** differentiator = *"the companion that never lies"* — every
  factual claim routes through Laverna's deterministic, SHA-256-verified
  substrate with a user-visible receipt; unverifiable claims are refused.
- **Funding:** apply to **NGI Fediversity by 2026-08-01** (local-first /
  personal-freedom fit); prepare **ELFA** resubmission when it opens
  (post-summer 2026, tightest fit); skip TALER.

# eCO Copyright Registration — Form TX (Prep Sheet)

**Status:** DRAFT prep for filing at <https://eco.copyright.gov>. Not legal advice.
**Prepared:** 2026-07-16. **Work:** Laverna v0.3.0 source code.

## Filing facts (Stage 0.3 of IP report)

| Field | Value |
|-------|-------|
| Application type | **Single Application** (one work, one author = sole claimant, not for hire) |
| Filing fee | **$45** (single-author); verify at copyright.gov before paying — USCO proposed raising *standard* fee to $85 in Mar 2026, but the $45 single-author fee is the relevant one |
| Type of work | **Literary Work — Form TX** (software registered as a literary work) |
| Title of work | `Laverna` |
| Title / version | `Laverna`, version `0.3.0` |
| Publication status | **Unpublished** (Cargo.toml `publish = false`; never released on crates.io or as a distributed binary at registration time) → file as unpublished; re-register at significant future releases only |
| Year of creation | 2026 |
| Author | `nutypebuddha` (sole author, no outside contributors, no CLA/DCO needed) |
| Claimant | `nutypebuddha` (same as author — sole claimant) |
| Nature of authorship | **Computer program** (entire program; original code + build/embedded corpus logic) |
| Rights / license note | Licensed Apache-2.0 (open source). Registration does NOT forfeit copyright; open-sourcing is an exercise of exclusive rights. Prior MIT releases remain MIT for those who obtained them. |
| AI assistance disclosure | If any deposited code is verbatim AI-generated beyond *de minimis*, disclose per USCO Part 2 report (Jan 29, 2025) and describe your human contribution (architecture, selection, arrangement, editing). A solo-architected engine where AI was an assistant is squarely copyrightable in its human-authored expression. |

## Deposit requirement (computer program)

- **Rule:** first 25 + last 25 pages of source code. Whole program is >50 pages (34,031 lines / ~40 lines per page ≈ 850 pages), so submit **identifying portions only** — approximately **first 1,000 and last 1,000 lines** of source, including the page bearing the copyright notice.
- **Trade-secret redaction:** Laverna is open source, so redaction is largely moot; available if any private modules exist (block <50% of deposit, or first 10 + last 10 unblocked).
- **Recommended deposit assembly** (generated at filing time, NOT committed to repo verbatim to avoid bloating the tree — script in `scripts/`):
  1. `src/main.rs` (entry point, ~4,136 lines) — first ~500 lines + last ~500 lines.
  2. `src/lib.rs` (crate root, lists all modules) — full (short).
  3. `build.rs` (corpus embedding logic) — full.
  4. One representative deep module, e.g. `src/tanto/solver.rs` or `src/bankai/verifier.rs` — first + last portions.
  5. `Cargo.toml` (shows `license = "Apache-2.0"`, version, author).
- **Include the copyright notice page:** ensure a `// Copyright 2026 nutypebuddha` header appears in the deposited excerpt (present in `lib.rs`/`main.rs`).

## Pre-filing checklist

- [ ] Confirm $45 fee at copyright.gov (fee schedules move)
- [ ] Assemble deposit file (first/last 1,000 lines as above) as a PDF or plain-text
- [ ] Decide publication status = **Unpublished** (accurate as of 2026-07-16)
- [ ] Note registration made within 5 yrs of creation = prima facie evidence of validity
- [ ] File within 3 months of any future publication to preserve statutory-damages eligibility
- [ ] Keep the OpenTimestamps receipt (`docs/provenance/HEAD-431bbea.ots`) + GPG-signed git history as corroborating provenance (not a substitute for registration)

## Why register at all (recap from IP report)

Registration is a **precondition to filing an infringement suit** for a US work
(17 U.S.C. §411) and the **only** path to **statutory damages ($750–$30,000/up
to $150,000 willful) and attorney's fees** (17 U.S.C. §504). Without it you're
limited to hard-to-prove actual damages/profits. The crypto layer proves
existence/integrity; registration unlocks the remedies.

# NLnet grant strategy & draft — Ł.AI

> Living draft. Lead with **verification infrastructure** + **local-first / offline
> trust**, not the astrology engine. The chart engine is one concrete
> proof-of-capability domain among 528 formulas. **Umbrella brand: Ł.AI**
> (see `docs/brand.md`); Laverna = **Ł.AI · Proof**, CID = **Ł.AI · Gate**,
> CID-Bridge = **Ł.AI · Bridge**.

## 0. Current call landscape (verified 2026-07-16)

NLnet is mid-transition from NGI0 to the **Open Internet Stack**. Per
nlnet.nl/propose: *"temporarily only accepting proposals for two specific
funds (GNU Taler and Fediversity). Others will need to have some patience:
our regular open call will reopen after the summer."*

| Fund | Status (2026-07-16) | Deadline | Fit for Laverna |
|------|-------------------|----------|-----------------|
| **NGI Fediversity** | **OPEN** | **2026-08-01 12:00 CEST** | **Strong-ish.** Local-first hosting stack, service portability, *personal freedom*. A deterministic, offline verification sidecar that keeps AI claims local + auditable maps to "personal freedom" + "runs everywhere". Best available fit right now. |
| **NGI TALER** | **OPEN** | **2026-08-01 12:00 CEST** | Weak. Privacy-preserving *payments*. Laverna is not a payment system. Only pursue if reframed as "verifiable local micro-payment accounting" — not advised; skip. |
| **ELFA** (Encrypted Local-First Architecture) | Call opens **"soon" post-summer 2026** | TBA (watch nlnet.nl/ELFA) | **Best thematic fit.** Offline-first, E2E-encrypted, local-first workspaces. Laverna's deterministic, offline, content-addressed proof objects are squarely "local-first trustworthy infra". **Target this when it opens.** |
| NGI0 Commons / regular open call | Paused; **reopens after summer 2026** | Rolling 1st of even months (historically) | The original target; revisit post-summer. |

**Decision:** Submit to **NGI Fediversity by 2026-08-01** (the only strong open
call with a near deadline), framed as a *local-first, offline verification
component* for trustworthy AI. Simultaneously **prepare an ELFA submission**
for when that call opens (post-summer 2026), which is the tighter fit.
Do **not** burn the Aug 1 slot on TALER.

## 1. Applicant & project basics

- **Project:** Ł.AI — auditable, offline, deterministic verification
  infrastructure for trustworthy AI. (Laverna = Ł.AI · Proof; plus CID
  = Ł.AI · Gate per-token validation; CID-Bridge = Ł.AI · Bridge chatbot fan-out.)
- **License:** Apache-2.0 (recognized FOSS license; NLnet-eligible).
  *Relicensed from MIT on 2026-07-16 as sole author — see
  `docs/copyright-registration-prep.md`.*
- **Form:** individual applicant (solo dev); first-time proposal ceiling €50,000;
  lifetime cap per third party in the Commons Fund €500k. Milestone-based
  payment (not upfront).
- **Language/footprint:** Rust; single static binary (~9 MB musl), runs on
  x86_64 Linux and on Android/Termux (demonstrated, not hypothetical).
- **Repo:** `/root/Laverna` — `v0.3.0`, offline-first, no network at
  runtime except optional opt-in websearch.
- **Provenance already in place (strengthens the app):** GPG-signed git
  history + an OpenTimestamps Bitcoin receipt for the relicense HEAD
  (`docs/provenance/`), plus a drafted eCO Form TX copyright filing.

## 2. The problem we address

LLM pipelines increasingly emit numbers, formalizations, and "verified"
conclusions they cannot actually justify. The dominant guardrail tooling
(Guardrails AI, NVIDIA NeMo Guardrails, LMQL) is Python, online-oriented,
and schema/policy-validation — it checks *shape* and *policy*, not *truth*.
Nobody occupies the gap of **offline-first, deterministic, MCP-native, Rust
verification sidecar** that emits machine-checkable proof objects.

Research basis Laverna is built on:
- PAL (Gao et al., ICML 2023): LLMs collapse to 23.2% accuracy without external compute.
- Logic-LM (Pan et al., EMNLP 2023): translate-then-solve with error feedback = +39.2%.
- LLM-Modulo (ICML 2024): LLM = generator, external = critic. Never reverse.

Laverna is the external critic — and it runs **locally**, with no cloud.

## 3. What we are building

A verification substrate where every claim is either **proven** or **refused** —
never guessed.

- **Proof-carrying objects.** `solve --proof-out` emits a JSON proof embedding
  the resolved inputs, the descent result, the embedded-corpus version +
  content hash, and a SHA-256 digest over the canonical payload. `verify
  <proof>` re-runs the descent from the recorded query against the embedded
  corpus and demands a byte-identical payload. Tampering, corpus drift, or a
  forged score all fail loud. (Spec: `docs/verify-api.md`.)
- **Typed refusals.** Every rejection is one of a fixed, machine-readable set —
  `OutOfScope`, `Underspecified`, `TooComplex`, `NoTranslation`,
  `MissingTimezone` — each carrying a `fix_suggestion` so an LLM
  orchestration loop can self-correct instead of parsing error prose.
  (Manifesto: `docs/manifesto.md`.)
- **Determinism by construction.** No unordered collection in any output path;
  canonical reduction order; content-addressed corpus (FNV-1a) so a drift
  fails verification rather than shipping a wrong answer. No invented scalars —
  a value that must be assumed is a refusal, not a default.
- **MCP-native.** `laverna mcp` already exists; the `verify` checker is the
  reference trusted math/logic verifier the MCP ecosystem lacks. A working
  **verify-first proxy** (`scripts/laverna-mcp-proxy.py`) demonstrates the
  LLM↔Laverna loop end-to-end: factual queries answered via a tool call
  (VERIFIED, with tool name as the receipt); subjective ones refused as
  UNVERIFIED, never fabricated. See `docs/stage1-poc.md`.

## 4. Closest funded precedent

**Provability Fabric** (NLnet-funded): open-source infrastructure "for making
AI and software systems trustworthy through evidence that can be independently
verified," integrating "formal verification, runtime security, and end-to-end
audit trails." Laverna is a narrower, deployment-ready, **offline** instance of
the same thesis: independent, reproducible verification of AI-produced claims,
at the edge and under local control.

## 5. Why now / why us

- The market gap is real and unoccupied (see §2).
- The architecture is already implemented and demonstrably working: proof objects,
  the independent `verify` checker, typed refusals, a determinism audit, and a
  working verify-first MCP loop are done in `v0.3.0`.
- **Edge/local deployment is demonstrated, not proposed:** Laverna runs in
  Termux on Android — a genuinely constrained offline environment. This is
  precisely the "local-first / personal freedom / runs everywhere" thesis
  NLnet's current calls (Fediversity, and ELFA when it opens) are funding.
- IP hygiene is already handled: Apache-2.0 license, GPG-signed + Bitcoin-
  timestamped provenance, drafted copyright filing — unusual discipline for a
  solo dev, and exactly what a careful funder wants to see.

## 6. Proposed work (Fediversity submission, milestone-broken)

1. **Proof-object standard (public, small).** Publish `docs/verify-api.md`
   as a minimal open standard for everyday numeric/logical claims (distinct
   from DRAT/LFSC formal-proof formats) — a standards play a solo dev can
   originate. *Milestone deliverable: spec v1 + 2 independent implementations.*
2. **Reference MCP verifier, hardened + listed.** Harden `laverna mcp`'s
   `verify` tool (reduce producer/checker shared code so verification is
   genuine checking); list it in ≥1 MCP registry (mcp.so, Smithery).
   *Deliverable: registry listing + integration test suite.*
3. **Local-first deployment story.** Package Laverna for NixOS / Fediversity-
   style local hosting (single static binary, no runtime network); document
   the offline verification sidecar pattern for self-hosted LLM stacks.
   *Deliverable: Nix flake / container + runbook.*
4. **Corpus breadth & governance.** Grow the 528-formula corpus (currently
   astrology + pharmacology + strategy profiles) toward general numeric/
   logical domains; the community overlay loader is already in place for forks.
   *Deliverable: +2 domains, overlay CI check.*

## 7. Intended impact

Trustworthy AI needs infrastructure that *proves* rather than *asserts*. Laverna
gives an LLM orchestration loop a deterministic, **offline**, reproducible
critic it can branch on — turning "fail loudly instead of hallucinate" from a
slogan into an API, and keeping that verification under the user's local
control rather than in a vendor cloud.

## 8. ELFA watch-list (post-summer 2026)

When ELFA's first call opens, resubmit §6 reframed around ELFA's explicit
"Encrypted Local-First" / "private workspaces" / "healthy social networking"
goals: Laverna as the **verification layer for local-first AI assistants** —
the trustworthy substrate under a companion that "never lies," fully offline,
content-addressed, and user-controlled. This is the tightest possible fit and
should be the primary target once the call opens.

## Appendix: before/after example (the pitch in one screen)

**Before:** user supplies `April 14, 1994, 8:09 PM` with no timezone.
Engine assumes UTC, silently computes the *wrong* chart, emits it as fact.

**After:** engine refuses with `[REFUSAL MissingTimezone]` + a fix
suggestion; computes only against an unambiguously resolved UTC instant, and
embeds the corpus hash + digest in the resulting proof object so the chart can
be independently re-checked. Nothing invented.

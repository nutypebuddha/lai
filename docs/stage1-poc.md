# Stage 1 — Technical spine PoC (Termux path A)

**Goal (from IP report Stage 1.5):** prove the *"LLM ↔ Laverna verification
tool-call"* loop end-to-end, and bake in the companion's core rule —
**"route facts through Laverna, never fabricate."**

## What was built

- **`scripts/laverna-mcp-proxy.py`** — a Termux-side MCP client + verification
  proxy. It speaks the MCP JSON-RPC protocol to `laverna mcp` (stdin/stdout),
  and implements a **verify-first** policy:
  - factual / computable questions → routed to a `laverna` tool call; the
    *verified* result is returned with the tool name (the "show me the receipt" UX).
  - subjective / opinion / personal questions → **refused as UNVERIFIED**;
    the proxy never fabricates an answer.
- `laverna` was built with `--features "mcp websearch"`; the server exposes
  9 verification tools (`solve`, `entity_get`, `chart`, `validate`, `formulas`,
  `entities`, `optimize`, `route`, `build`).

## Run it

```bash
# build the server
cargo build --release --features "mcp websearch"

# single query (JSON out)
python3 scripts/laverna-mcp-proxy.py --query "which graha rules kanya?"

# demo: 3 verified + 2 refused probes
python3 scripts/laverna-mcp-proxy.py --demo
```

## Result

| Probe | verdict | tool |
|-------|---------|------|
| Which graha rules mithuna (Gemini)? | VERIFIED | `route` |
| Cast a sidereal chart for 2000-01-01T12:00Z | VERIFIED | `chart` |
| What is the formula for shadbala? | VERIFIED | `solve` |
| Do you think astrology is real? | UNVERIFIED (refused) | — |
| What did you have for breakfast? | UNVERIFIED (refused) | — |

The loop is proven: the deterministic substrate answers computable claims;
everything else is explicitly flagged, never invented. This is the
differentiator from the IP report — *"the companion that never lies to you."*

## Next (Stage 1 native shell, deferred)

- Replace the keyword classifier with a real LLM routing call (cloud API or
  on-device Gemma via llama.cpp).
- Embed the Rust binary via JNI/NDK in a Kotlin app; add `VoiceInteractionService`
  + Good Lock side-key gesture; plan for the ROLE_ASSISTANT reinstall bug.
- Add structured SQLite memory + a Live2D portrait (v0.1 companion).

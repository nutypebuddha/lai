# Bug: formulas/ and entities/ are loaded relative to cwd — stale data served silently

**Found:** 2026-07-07, by dogfooding `athena_reason` over MCP.
**Severity:** high — every engine answer (reason paths, entity lookups, formula graph) silently
reflects whatever `./formulas` and `./entities` happen to exist in the server's working directory.

## Symptom

With the MCP server registered at user scope and Claude Code started in `/root`:

- `athena_reason have=mass,velocity want=kinetic_energy` → `no path found` (formula exists,
  `athena_evaluate kinetic_energy {mass:2, velocity:3}` → 9.0 in the same session).
- `athena_reason have=mass,velocity want=momentum` → `no path found` (formula exists on disk).
- `newtons_second` / `work_energy` paths resolve fine, masking the breakage.

## Root cause

`src/main.rs:1286` loads `["formulas/atomic", "formulas/bridging"]` and `src/main.rs:1319` loads
`["entities"]` — all **cwd-relative**. The user-scope MCP server inherits Claude Code's cwd, so a
session started in `/root` loads the **pre-graha snapshots** `/root/formulas` and `/root/entities`
(old zodiac files; `kinetic_energy` there has `output = "ke"`, `domain = "taurus"`). The zodiac→
graha rulership mapping (taurus→Shukra) makes the stale data *look* migrated in tool output.

Note `/root/entities` and `/root/athena/entities` are **no longer the same files** (different
inodes, divergent contents) — AGENTS.md's claim that they are is stale.

## Applied mitigation (config-level, done 2026-07-07)

`scripts/athena-mcp.sh` pins cwd to the repo root before exec'ing `target/release/athena mcp`.
Both the user-scope registration (`/root/.claude.json`) and `.mcp.json` now launch through it.

## Proper fix (code-level, TODO)

Resolve the data root explicitly instead of trusting cwd, e.g. in priority order:
1. `ATHENA_HOME` env var (or a `--data-dir` flag on the `mcp`/CLI subcommands),
2. directory of the running executable, walking up to find `formulas/`,
3. cwd as the last resort — and **log the resolved absolute paths at startup** so a wrong data
   root is visible instead of silent.

Also consider deleting or archiving the stale `/root/formulas` and `/root/entities` snapshots so
they can't be picked up again (verify nothing else depends on them first).

## Regression check after the code fix

From any cwd outside the repo:
`cd /tmp && /root/athena/target/release/athena mcp` (or the CLI equivalents) — then
`reason have=mass,velocity want=kinetic_energy` must return the 1-step `kinetic_energy` chain.

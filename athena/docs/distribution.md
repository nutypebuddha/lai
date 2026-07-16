# Distribution — how Athena ships

Decisions locked 2026-07-08. Scope: **portable Linux only** (x86_64 + aarch64,
static musl). No Windows/macOS targets.

## The capstone

Athena's copilot is the *smallest viable local LLM* — Qwen2.5-0.5B-Instruct,
Q4_K_M GGUF (469 MB) — acting synergistically with large cloud LLMs: the local
model handles routing, formula suggestion, and explanation offline; anything
heavier goes to a remote backend (`athena llm --backend remote --endpoint …`,
OpenAI-compatible). Quantizing harder buys almost nothing (Q2_K is still
396 MB — embedding tables dominate at this scale), so Q4_K_M is the pinned
default.

## Release artifacts (per tag, from CI)

| Artifact | Contents | Audience |
|---|---|---|
| `athena-<tag>-<target>.tar.gz` | static binary (`mcp`+`llm`) + `models/qwen2.5-0.5b-instruct-q4_k_m.gguf` + `SHA256SUMS` | offline out-of-the-box; zero setup |
| `athena-slim-<target>-bin` | bare static binary, no `mcp`/`llm` | minimal installs |
| `SHA256SUMS` | checksums of all assets | everyone |

Targets: `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`.
Static linkage is CI-verified (`readelf -d` must show no `NEEDED` entries).

## Model discovery (in priority order)

1. `ATHENA_MODEL_PATH` / config `model_path`
2. `./models/` (cwd)
3. `models/` **next to the executable** — this is what makes the tarball
   work from any cwd
4. `$XDG_CACHE_HOME/athena/models/` (default `~/.cache/athena/models/`)
5. `/sdcard/Download/athena-export/models/` (Android proot export)

If nothing is found and `auto_download` is enabled (the default), the binary
fetches the pinned model from HuggingFace into the XDG cache dir, streams it
through sha256, and refuses the file on mismatch
(`src/inference/download.rs`).

## The pin

```
qwen2.5-0.5b-instruct-q4_k_m.gguf
sha256 74a4da8c9fdbcd15bd1f6d01d621410d31c6fc00986f5eb687824e7b93d7a9db
491,400,032 bytes
Qwen/Qwen2.5-0.5B-Instruct-GGUF
```

The pin lives in **one place**: `capstone/athena-qwen.toml`, the capstone
manifest — model identity as KB data, loaded at runtime like `formulas/` and
`entities/` (search order: `./capstone/`, `capstone/` next to the binary,
`$XDG_CONFIG_HOME/athena/capstone/`). CI's release job reads the same file to
build the tarball, and the tarball ships it alongside the binary. Swapping
the capstone (e.g. for a fine-tuned Athena-Qwen) is a data change: edit the
manifest, no recompile.

`DEFAULT_MODEL_SHA256` in `src/inference/download.rs` is only the built-in
fallback for running without any manifest; a test
(`test_repo_manifest_parses_and_matches_download_pin`) keeps the repo
manifest parseable.

## Feature policy

`llm` is **not** a default cargo feature: local dev iteration on the Android
proot box stays lean (`default = ["mcp"]`). Every CI/release binary that ships
to users is built `--features llm` — the capstone is always present in what
people download.

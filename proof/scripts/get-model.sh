#!/usr/bin/env bash
#
# Copyright 2026 nutypebuddha
# SPDX-License-Identifier: Apache-2.0
#
# get-model.sh — one-command model bootstrap for L.ai's local LLM.
#
# Downloads a small, permit-free instruct GGUF into bin/models/ so the
# bundled llama.cpp engine has weights to speak with. No API key, no auth,
# no network at runtime afterwards.
#
# Default model: Qwen2.5-0.5B-Instruct (Q4_K_M), ~469 MB.
#   Source: https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF
#   Permissive Apache-2.0 weights; downloaded over plain HTTPS (no LFS login).
#
# Usage:
#   ./scripts/get-model.sh                 # download default into bin/models/
#   MODEL_URL=... ./scripts/get-model.sh   # override the GGUF URL
#   DEST_DIR=... ./scripts/get-model.sh    # override the target directory

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST_DIR="${DEST_DIR:-$ROOT/bin/models}"
MODEL_URL="${MODEL_URL:-https://huggingface.co/Qwen/Qwen2.5-0.5B-Instruct-GGUF/resolve/main/qwen2.5-0.5b-instruct-q4_k_m.gguf}"
MODEL_NAME="$(basename "$MODEL_URL")"

# Expected byte size of the pinned default (Qwen2.5-0.5B-Instruct Q4_K_M).
EXPECTED_BYTES="${EXPECTED_BYTES:-491400032}"

mkdir -p "$DEST_DIR"

# Refuse to clobber an existing model; the engine uses the first *.gguf found.
if ls "$DEST_DIR"/*.gguf >/dev/null 2>&1; then
  echo "A model already exists in $DEST_DIR — leaving it in place." >&2
  echo "Delete it first if you want to replace the weights." >&2
  ls -1 "$DEST_DIR"/*.gguf
  exit 0
fi

OUT="$DEST_DIR/$MODEL_NAME"
echo "Downloading $MODEL_NAME -> $OUT" >&2
echo "Source: $MODEL_URL" >&2

if command -v curl >/dev/null 2>&1; then
  curl -fL --retry 3 --retry-delay 2 -C - -o "$OUT" "$MODEL_URL"
else
  wget -c -O "$OUT" "$MODEL_URL"
fi

SIZE="$(stat -c%s "$OUT" 2>/dev/null || stat -f%s "$OUT" 2>/dev/null)"
if [ "${SIZE:-0}" -lt 1000000 ]; then
  echo "Download looks too small ($SIZE bytes) — likely an error page. Aborting." >&2
  rm -f "$OUT"
  exit 1
fi

if [ "$SIZE" != "$EXPECTED_BYTES" ]; then
  echo "Warning: downloaded $SIZE bytes; expected $EXPECTED_BYTES." >&2
  echo "The model may have changed upstream. Continuing, but verify quality." >&2
else
  echo "Size check OK ($SIZE bytes)." >&2
fi

echo "" >&2
echo "Model ready: $OUT" >&2
echo "Run the assistant (needs --features llm):" >&2
echo "  cargo run --features llm -- mcp" >&2
echo "  # or, after a release build:" >&2
echo "  laverna mcp" >&2

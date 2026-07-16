#!/bin/sh
# Serve Athena's pinned capstone model via llama.cpp (NEON-optimized build
# salvaged from ollama, 2026-07-08 — see docs/distribution.md).
#
# Athena's remote backend defaults to this endpoint (127.0.0.1:8080/v1);
# with ~/.config/athena/inference.toml pointing here, plain `athena llm ...`
# uses it automatically.
LLAMA_DIR=/usr/local/lib/athena-llama
MODEL="${ATHENA_MODEL_PATH:-$HOME/.cache/athena/models/qwen2.5-0.5b-instruct-q4_k_m.gguf}"

exec env LD_LIBRARY_PATH="$LLAMA_DIR" "$LLAMA_DIR/llama-server" \
  -m "$MODEL" -c 1024 --host 127.0.0.1 --port 8080 "$@"

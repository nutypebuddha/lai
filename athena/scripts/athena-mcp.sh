#!/usr/bin/env bash
# athena-mcp.sh — start the MCP server with cwd pinned to the repo root.
# formulas/ and entities/ are loaded relative to cwd (src/main.rs), so starting
# the server from any other directory silently serves whatever stale copies live
# there (e.g. the pre-graha /root/formulas snapshot).
set -euo pipefail
cd "$(dirname "$(readlink -f "$0")")/.." && exec target/release/athena mcp "$@"

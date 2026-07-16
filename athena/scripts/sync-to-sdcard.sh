#!/usr/bin/env bash
# sync-to-sdcard.sh — export Athena binary + source snapshot
#
# Usage:  ./scripts/sync-to-sdcard.sh          # export binary + source
#         ./scripts/sync-to-sdcard.sh binary    # binary only
#         ./scripts/sync-to-sdcard.sh source    # source zip only
#         ./scripts/sync-to-sdcard.sh bench     # add benchmark report
#
set -euo pipefail

SENDIR="/sdcard/Download/athena-export"
SCRIPTDIR="$(cd "$(dirname "$0")" && pwd)"
ROOTDIR="$(cd "$SCRIPTDIR/.." && pwd)"

mkdir -p "$SENDIR"

mode="${1:-all}"

if [[ "$mode" == "all" || "$mode" == "binary" ]]; then
  if [[ -f "$ROOTDIR/target/release/athena" ]]; then
    cp "$ROOTDIR/target/release/athena" "$SENDIR/athena"
    echo "✅ binary: $(ls -lh "$SENDIR/athena" | awk '{print $5}')"
  else
    echo "⚠️  binary not found at target/release/athena — build first"
  fi
fi

if [[ "$mode" == "all" || "$mode" == "source" ]]; then
  SNAPSHOT="$SENDIR/athena-src.zip"
  rm -f "$SNAPSHOT"
  cd "$ROOTDIR"
  zip -r "$SNAPSHOT" \
    Cargo.toml Cargo.lock rust-toolchain.toml build.rs \
    Dockerfile docker-compose.yml .dockerignore \
    src/ formulas/ entities/ tests/ benches/ \
    -x "src/**/target/*" -x "target/*" \
    > /dev/null
  echo "✅ source: $(ls -lh "$SNAPSHOT" | awk '{print $5}')"
fi

if [[ "$mode" == "bench" ]]; then
  if [[ -d "$ROOTDIR/target/criterion/report" ]]; then
    cp -r "$ROOTDIR/target/criterion/report" "$SENDIR/bench-report/"
    echo "✅ benchmarks: copied to bench-report/"
  else
    echo "⚠️  benchmark report not found at target/criterion/report — run cargo bench first"
  fi
fi

echo "📁 sync target: $SENDIR"
ls -lh "$SENDIR/" 2>/dev/null | tail -n +2

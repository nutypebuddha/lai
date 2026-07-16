#!/usr/bin/env bash
# athena-launcher.sh — pick the right musl binary for the host architecture
set -euo pipefail

dir="$(dirname "$0")"
arch="$(uname -m)"

case "$arch" in
  x86_64)  bin="athena-x86_64-unknown-linux-musl" ;;
  aarch64) bin="athena-aarch64-unknown-linux-musl" ;;
  armv7l)  bin="athena-armv7-unknown-linux-musleabihf" ;;
  *)
    echo "unsupported architecture: $arch" >&2
    echo "expected x86_64 or aarch64" >&2
    exit 1
    ;;
esac

if [ ! -x "$dir/$bin" ]; then
  echo "missing binary: $dir/$bin" >&2
  echo "download the correct artifact from the releases page" >&2
  exit 1
fi

exec "$dir/$bin" "$@"

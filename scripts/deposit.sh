#!/usr/bin/env bash
# Assemble the eCO Form TX deposit for Laverna (first 25 + last 25 pages rule).
# ~40 lines/page -> first 1,000 + last 1,000 lines of source, per Circular 61.
# Output: /tmp/laverna-deposit.txt  (concatenate into a PDF before uploading)
set -euo pipefail
cd "$(dirname "$0")/.."

OUT="/tmp/laverna-deposit.txt"
: > "$OUT"

emit() {  # $1=file  $2=head_lines  $3=tail_lines
  local f="$1" h="$2" t="$3"
  [ -f "$f" ] || return 0
  echo "===== FILE: $f =====" >> "$OUT"
  if [ "$(wc -l < "$f")" -le $((h + t)) ]; then
    cat "$f" >> "$OUT"
  else
    head -n "$h" "$f" >> "$OUT"
    echo "... ($(wc -l < "$f") total lines; middle omitted) ..." >> "$OUT"
    tail -n "$t" "$f" >> "$OUT"
  fi
  echo "" >> "$OUT"
}

H=500; T=500
emit src/main.rs        "$H" "$T"
emit src/lib.rs         999 999
emit build.rs           999 999
emit src/tanto/solver.rs "$H" "$T"
emit src/bankai/verifier.rs "$H" "$T"
emit Cargo.toml        999 999

echo "Deposit written to $OUT ($(wc -l < "$OUT") lines)."
echo "Convert to PDF or upload .txt at https://eco.copyright.gov (Form TX)."

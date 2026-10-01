#!/usr/bin/env bash
# Run the unit tests of every crate in ai/. Usage: tests/run.sh [crate ...]
set -u
cd "$(dirname "$0")/../ai"
crates=("$@")
[ ${#crates[@]} -eq 0 ] && crates=(en global layers prag coref qagen math latex world coder mlab vfs)
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$PWD/../target}"
fail=0
for c in "${crates[@]}"; do
  printf '%-8s ' "$c"
  if out=$(cd "$c" && cargo test --release -q 2>&1); then
    echo "ok   $(echo "$out" | grep -o '[0-9]* passed' | awk '{s+=$1} END {print s}') passed"
  else
    echo "FAIL"; echo "$out" | grep -E 'FAILED|panicked' | head -5; fail=1
  fi
done
exit $fail

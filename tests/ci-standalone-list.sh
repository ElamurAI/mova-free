#!/usr/bin/env bash
# Every standalone Cargo workspace under tools/ (a Cargo.toml with its own [workspace]) must be listed in the
# standalone-workers matrix of the CI workflow; the root `cargo build` does not build them. The root lane must also run
# the tests of the release fetch and the Open English WordNet install (`cargo test --release --locked -p mova-modules`),
# which building alone does not.
# Usage: tests/ci-standalone-list.sh [workflow file]   (default .github/workflows/ci.yml)
# Negative control: a workflow copy without one of the entries (or without the mova-modules lane) makes it exit 1.
set -u
cd "$(dirname "$0")/.."
ci="${1:-.github/workflows/ci.yml}"
missing=0
n=0
for m in tools/*/Cargo.toml; do
  grep -q '^\[workspace\]' "$m" || continue
  d="${m%/Cargo.toml}"
  n=$((n + 1))
  if ! grep -qE "^ *- $d\$" "$ci"; then
    echo "not in the CI standalone-workers matrix: $d" >&2
    missing=1
  fi
done
if ! grep -qE '^ *run: cargo test --release --locked -p mova-modules$' "$ci"; then
  echo "the root test lane does not run the mova-modules tests (release fetch, OEWN install)" >&2
  missing=1
fi
[ "$n" -gt 0 ] || { echo "no standalone workspaces found under tools/" >&2; exit 1; }
[ "$missing" -eq 0 ] && echo "ci-standalone-list: all $n standalone workspaces and the mova-modules lane are in $ci"
exit "$missing"

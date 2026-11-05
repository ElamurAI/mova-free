#!/usr/bin/env bash
# Download the Mova corpus. Usage: train/get-corpus.sh [version]   (default 0.5)
# 0.5 and later: `mova fetch <version> corpus` (SHA-256 checked, unpacked into the data directory).
# 0.2–0.4: the historical layout, downloaded from research.elamur.ai and unpacked into corpus/open/.
set -euo pipefail
v="${1:-0.5}"
root="$(cd "$(dirname "$0")/.." && pwd)"
case "$v" in
  0.2|0.3|0.4) ;;
  *) exec "$root/target/release/mova" fetch "$v" corpus ;;
esac
base="https://research.elamur.ai/download/${v%%.*}/$v"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cd "$tmp"
echo "downloading corpus-$v.tar.gz from $base"
curl -fL --progress-bar -o "corpus-$v.tar.gz" "$base/corpus-$v.tar.gz"
curl -fsSL -o "corpus-$v.tar.gz.sha256" "$base/corpus-$v.tar.gz.sha256"
sha256sum -c "corpus-$v.tar.gz.sha256"
mkdir -p "$root/corpus/open"
tar xzf "corpus-$v.tar.gz" --strip-components=1 -C "$root/corpus/open" --exclude="README.md"
echo "corpus $v unpacked into $root/corpus/open:"
ls "$root/corpus/open"

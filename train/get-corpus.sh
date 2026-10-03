#!/usr/bin/env bash
# Download the Mova corpus from research.elamur.ai, check its SHA-256 and unpack it into corpus/.
# Usage: train/get-corpus.sh [version]     (default 0.3)
set -euo pipefail
v="${1:-0.3}"
base="https://research.elamur.ai/download/${v%%.*}/$v"
root="$(cd "$(dirname "$0")/.." && pwd)"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
cd "$tmp"
echo "downloading corpus-$v.tar.gz from $base"
curl -fL --progress-bar -o "corpus-$v.tar.gz" "$base/corpus-$v.tar.gz"
curl -fsSL -o "corpus-$v.tar.gz.sha256" "$base/corpus-$v.tar.gz.sha256"
sha256sum -c "corpus-$v.tar.gz.sha256"
tar xzf "corpus-$v.tar.gz" --strip-components=1 -C "$root/corpus" --exclude="corpus-$v/README.md"
echo "corpus $v unpacked into $root/corpus:"
ls "$root/corpus"

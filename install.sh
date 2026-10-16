#!/usr/bin/env bash
# install.sh — fetch Mova's trained models into the data directory.
#
# The models are not in git (they are large and change between releases). This script downloads them from the
# release's download folder (where its corpus is too), checks each file against its SHA-256, and puts it where the crates look by default:
#
#   $MOVA_DATA/en/models/en.bin   the `en` model: tokenizer, tagger, parser, morphology and lemmas
#
# The models are distributed under CC BY-SA 4.0 (the licence of the UD treebanks they are trained on); the code of
# the repository, this script included, is under the Elamur Cosmic License.
#
# Usage:
#   ./install.sh                 # MOVA_DATA defaults to ~/mova-data
#   MOVA_DATA=/srv/mova ./install.sh
#   MOVA_MODELS_URL=<base url> ./install.sh   # another mirror
#
# exit 0 = installed (or already up to date), exit 1 = a download or a checksum failed.

set -euo pipefail

RELEASE="0.4"
BASE="${MOVA_MODELS_URL:-https://research.elamur.ai/download/0}"
DATA="${MOVA_DATA:-$HOME/mova-data}"

# file on the server (next to the release's corpus), place in the data directory, SHA-256
MODELS=(
  "en.bin en/models/en.bin 31a93401fe7135b17e1eacccf608d1a42e94ebc8951f011d7f80ff9db144eb86"
)

if [ -z "$BASE" ]; then
  echo "install.sh: no model server; set MOVA_MODELS_URL" >&2
  exit 1
fi
command -v curl >/dev/null || { echo "install.sh: curl is needed" >&2; exit 1; }
command -v sha256sum >/dev/null || { echo "install.sh: sha256sum is needed" >&2; exit 1; }

for entry in "${MODELS[@]}"; do
  read -r file path sum <<<"$entry"
  dest="$DATA/$path"
  if [ -f "$dest" ] && [ "$(sha256sum "$dest" | cut -d' ' -f1)" = "$sum" ]; then
    echo "install.sh: $path is up to date"
    continue
  fi
  mkdir -p "$(dirname "$dest")"
  echo "install.sh: downloading $path"
  curl -fL --retry 2 -o "$dest.part" "$BASE/$RELEASE/$file"
  got="$(sha256sum "$dest.part" | cut -d' ' -f1)"
  if [ "$got" != "$sum" ]; then
    rm -f "$dest.part"
    echo "install.sh: checksum mismatch for $path (expected $sum, got $got)" >&2
    exit 1
  fi
  mv "$dest.part" "$dest"
  echo "install.sh: $path installed"
done
echo "install.sh: done — models in $DATA"

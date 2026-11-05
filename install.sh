#!/usr/bin/env bash
# install.sh — fetch Mova's trained models into the data directory.
#
# The models are not in git (they are large and change between releases). This script builds the `mova` command
# (crate tools/modules; always through Cargo, which decides whether anything changed) and runs `mova fetch 0.5`,
# which downloads the release's model archive
# (mova-model-0.5.tar.gz, listed with its SHA-256 in releases/0.5-files.json), refuses it unless the SHA-256 matches,
# unpacks it and puts the files where the crates look by default:
#
#   $MOVA_DATA/en/models/en.bin   the `en` model: tokenizer, tagger, parser, morphology and lemmas
#   $MOVA_DATA/en/models/gparse/  the GPU parser model (`gparse`)
#   $MOVA_DATA/coref/gender.tsv   the coreference noun lexicon
#   $MOVA_DATA/raw/en-oewn/json/  Open English WordNet 2025 (JSON), read by the school's world model
#                                 (`world::wordnet`); fetched from the OEWN GitHub release, CC BY 4.0, by
#                                 `mova oewn install` (zip SHA-256, full file list, load check, staged)
#
# An earlier different file in one of those places is kept beside it as <file>.before-0.5. The models are distributed
# under CC BY-SA 4.0 (the licence of the UD treebanks they are trained on); the code of the repository, this script
# included, is under the Elamur Cosmic License. The other release files (the corpus, the dictionaries, add-on
# modules) and the BitTorrent ways are in releases/0.5-files.md.
#
# Usage:
#   ./install.sh                 # MOVA_DATA defaults to ~/mova-data
#   MOVA_DATA=/srv/mova ./install.sh
#   MOVA_MODELS_URL=<base url> ./install.sh   # another mirror (the folder that holds 0.5/)
#   MOVA_FETCH_VIA=torrent ./install.sh       # https (default), torrent or magnet
#   MOVA_OEWN_URL=<url of english-wordnet-2025-json.zip> ./install.sh   # another OEWN mirror
#
# exit 0 = installed (or already up to date), exit 1 = a build, a download or a checksum failed.

set -euo pipefail

RELEASE="0.5"
DATA="${MOVA_DATA:-$HOME/mova-data}"
export MOVA_DATA="$DATA"
cd "$(dirname "$0")"

command -v cargo >/dev/null || { echo "install.sh: cargo is needed to build the mova command (https://rustup.rs)" >&2; exit 1; }
command -v curl >/dev/null || { echo "install.sh: curl is needed" >&2; exit 1; }
command -v tar >/dev/null || { echo "install.sh: tar is needed" >&2; exit 1; }
command -v unzip >/dev/null || { echo "install.sh: unzip is needed for Open English WordNet" >&2; exit 1; }

# Always ask Cargo: it rebuilds when any source or dependency changed and does nothing otherwise.
TARGET="${CARGO_TARGET_DIR:-target}"
MOVA="$TARGET/release/mova"
echo "install.sh: building the mova command (Cargo checks freshness)"
cargo build --release --quiet --locked -p mova-modules
args=(fetch "$RELEASE" model --via "${MOVA_FETCH_VIA:-https}")
if [ -n "${MOVA_MODELS_URL:-}" ]; then
  args+=(--base "${MOVA_MODELS_URL%/}/$RELEASE/")
fi
"$MOVA" "${args[@]}"
# Open English WordNet 2025 (CC BY 4.0, McCrae et al.; https://github.com/globalwordnet/english-wordnet): the zip's
# SHA-256 and the full list of its files (tools/modules/oewn-2025-json.tsv) are checked; it is unpacked into a staging
# folder, checked again (every file, and that it loads) and only then renamed to raw/en-oewn/json. An installed folder
# is reused only when it passes the same checks.
oewn_args=(oewn install)
if [ -n "${MOVA_OEWN_URL:-}" ]; then
  oewn_args+=(--url "$MOVA_OEWN_URL")
fi
"$MOVA" "${oewn_args[@]}"
echo "install.sh: done — models and data in $DATA"
echo "install.sh: the school (courses, lessons and school dictionaries) is a separate archive: before running a course,"
echo "install.sh:   $TARGET/release/mova fetch $RELEASE school   (installs $DATA/school; MOVA_SCHOOL_DATA picks another folder)"
case "$TARGET" in
  /*) bin="$TARGET/release" ;;
  *) bin="$PWD/$TARGET/release" ;;
esac
echo "install.sh: run the command from the repository root as $TARGET/release/mova (e.g. $TARGET/release/mova fetch $RELEASE corpus),"
echo "install.sh: or put it on the PATH for plain \`mova\`:  export PATH=\"$bin:\$PATH\""

#!/usr/bin/env bash
# secret-guard — keep secrets out of commits.
#
# It matches VALUES, not a pattern. A regex with word boundaries around a password misses the same string inside a
# longer word, and the pattern itself would be a secret that cannot be kept in the repo. So the secrets come from
# `local.md`: an optional, untracked, local file of strings that must never be committed. Each contributor may create
# their own; it is never committed. Its values are searched for verbatim (`grep -F`) in the staged diff. A value is
# taken from a line `NAME_API_KEY=value` (also `..._SECRET...=`, `..._ACCESS_KEY...=`, `..._TOKEN=`), or from the
# second backtick-quoted string of a line that names a sudo password. Without local.md only the pattern layer runs:
# it catches Anthropic and AWS API keys.
#
# Usage (from the repo root, before a commit):
#   ./secret-guard.sh && git commit ...
# exit 0 = clean, exit 1 = a leak was found.

set -uo pipefail
cd "$(git rev-parse --show-toplevel)" || exit 2

# local.md sits in the root of the main working copy; a git worktree finds it through the common git directory.
# SECRET_GUARD_LOCAL is only for the negative control with a fake local.md.
LOCAL="${SECRET_GUARD_LOCAL:-$(cd "$(git rev-parse --git-common-dir)/.." && pwd)/local.md}"
[ -f "$LOCAL" ] || echo "secret-guard: no $LOCAL (optional) — the verbatim check is skipped, the pattern layer still runs"

# Known false positives: words of the nomenclature that contain a secret's substring.
FALSE_POSITIVES='mykiss|kissen'

DIFF=$(git diff --cached)
[ -z "$DIFF" ] && { echo "secret-guard: the staged diff is empty"; exit 0; }

leak=0
# The pattern layer: an Anthropic or AWS key in the staged diff is a leak even when local.md does not list it.
if printf '%s\n' "$DIFF" | grep -qE 'sk-ant-(api|admin)[0-9]{2}-[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16}'; then
  echo "secret-guard: LEAK — an Anthropic (sk-ant-…) or AWS (AKIA…) key in the staged diff"
  leak=1
fi
while IFS= read -r s; do
  [ -z "$s" ] && continue
  # -F: verbatim, not a regex; then the lines with known false positives are dropped.
  if printf '%s\n' "$DIFF" | grep -nF -- "$s" | grep -viE "$FALSE_POSITIVES" | grep -q .; then
    # NEVER print the secret itself — only its length, to tell which one it is.
    echo "secret-guard: LEAK — a secret of ${#s} characters in the staged diff"
    leak=1
  fi
done < <([ -f "$LOCAL" ] && { awk -F'`' '/sudo-пароль|sudo password/{print $4}' "$LOCAL"; sed -nE 's/^[A-Z_]*(API_KEY|SECRET[A-Z_]*|ACCESS_KEY[A-Z_]*|TOKEN)=//p' "$LOCAL"; })

if [ "$leak" -ne 0 ]; then
  echo "secret-guard: the commit is BLOCKED. Remove the secret from the staged changes."
  exit 1
fi

echo "secret-guard: clean"
exit 0

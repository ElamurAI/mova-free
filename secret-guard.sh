#!/usr/bin/env bash
# secret-guard — keep secrets out of commits.
#
# It matches VALUES, not a pattern. A regex with word boundaries around a password misses the same string inside a
# longer word, and the pattern itself would be a secret that cannot be kept in the repo. So the secrets come from
# `local.md`: an optional, untracked, local file of strings that must never be committed. Each contributor may create
# their own; it is never committed. Its values are searched for verbatim (`grep -F`) in the staged diff. A value is
# taken from a line `NAME_API_KEY=value` (also `..._SECRET...=`, `..._ACCESS_KEY...=`, `..._TOKEN=`), or from the
# second backtick-quoted string of the line that names the sudo password (the line containing `sudo`). Without local.md only the pattern layer runs:
# it catches Anthropic and AWS API keys.
#
# Machine paths: the public part of the repo (everything outside peta/) names data only through variables
# (README.md, «Paths»). The paths of one machine come from local.md too, from a section that starts with a line
# `paths:` and lists one string per line as `- <string>` (for example `- /home/<user>`); the section ends at the first
# other line. A staged line added outside peta/ that contains one of them verbatim is refused.
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

# Known false positives: words that contain a secret's substring, one per line, from an optional private list
# (`peta/secret-guard-false-positives.txt`, absent from public checkouts); without it nothing is excused.
FP_FILE=peta/secret-guard-false-positives.txt
FALSE_POSITIVES=$( [ -f "$FP_FILE" ] && grep -v '^#' "$FP_FILE" | grep . | paste -sd'|' )
FALSE_POSITIVES=${FALSE_POSITIVES:-'$^'}

# `--range <a>..<b>` scans what a branch would bring instead of the staged diff: run it before a merge, since a merge
# commit does not pass through the guard.
if [ "${1:-}" = "--range" ]; then SRC=("$2"); else SRC=(--cached); fi
DIFF=$(git diff "${SRC[@]}")
[ -z "$DIFF" ] && { echo "secret-guard: the diff is empty"; exit 0; }

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
done < <([ -f "$LOCAL" ] && { awk -F'`' '/sudo/{print $4}' "$LOCAL"; sed -nE 's/^[A-Z_]*(API_KEY|SECRET[A-Z_]*|ACCESS_KEY[A-Z_]*|TOKEN)=//p' "$LOCAL"; })

if [ "$leak" -ne 0 ]; then
  echo "secret-guard: the commit is BLOCKED. Remove the secret from the staged changes."
  exit 1
fi

# Machine paths: only lines added outside peta/; a path is not a secret, so the offending lines are shown.
ADDED=$(git diff "${SRC[@]}" -U0 --no-color -- . ':(exclude)peta' | awk '/^\+\+\+ /{f=substr($0,7); next} /^@@/{split($3,a,/[+,]/); n=a[2]; next} /^\+/{print f ":" n ": " substr($0,2); n++}')
paths=0
while IFS= read -r p; do
  [ -z "$p" ] && continue
  hits=$(printf '%s\n' "$ADDED" | grep -F -- "$p")
  if [ -n "$hits" ]; then
    echo "secret-guard: MACHINE PATH «$p» added outside peta/:"
    printf '%s\n' "$hits" | head -20 | sed 's/^/  /'
    paths=1
  fi
done < <([ -f "$LOCAL" ] && awk '/^paths:[[:space:]]*$/{s=1; next} s && /^- /{sub(/^- /,""); sub(/[[:space:]]+$/,""); print; next} s{s=0}' "$LOCAL")

if [ "$paths" -ne 0 ]; then
  echo "secret-guard: the commit is BLOCKED. Name the place through a variable (README.md, «Paths»), or keep it in peta/."
  exit 1
fi

echo "secret-guard: clean"
exit 0

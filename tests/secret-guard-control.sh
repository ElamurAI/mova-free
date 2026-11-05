#!/usr/bin/env bash
# Negative control for secret-guard.sh: a gate that cannot show red is not a gate.
# In a throwaway git repo, with a fake local.md (SECRET_GUARD_LOCAL), the guard must refuse a staged line that adds a
# listed machine path outside peta/ and a listed secret, and must pass the same path inside peta/ and a line that
# names the place through a variable. Usage: tests/secret-guard-control.sh   (exit 0 = all cases as expected)

set -uo pipefail
GUARD="$(cd "$(dirname "$0")/.." && pwd)/secret-guard.sh"
T=$(mktemp -d)
trap 'rm -rf "$T"' EXIT
git -C "$T" init -q
cat > "$T/fake-local.md" <<'EOF'
# fake local.md for the control
FAKE_API_KEY=fake-secret-value-12345

paths:
- /srv/fake-machine
- FakeLibraryDir
EOF
export SECRET_GUARD_LOCAL="$T/fake-local.md"

fail=0
# case <name> <expected exit> <file> <line>: stage one added line in a fresh index, run the guard
case_() {
  git -C "$T" rm -rq --cached . 2>/dev/null
  mkdir -p "$(dirname "$T/$3")"
  printf '%s\n' "$4" > "$T/$3"
  git -C "$T" add "$3"
  (cd "$T" && "$GUARD" >/dev/null 2>&1)
  local got=$?
  if [ "$got" -eq "$2" ]; then echo "ok   $1 (exit $got)"; else echo "FAIL $1: exit $got, expected $2"; fail=1; fi
  rm -f "$T/$3"
}
case_ "machine path in docs"        1 docs/a.md   'The corpus is in /srv/fake-machine/corpora/x.'
case_ "machine path in code"        1 src/b.rs    'let d = "/srv/fake-machine/raw";'
case_ "second listed path"          1 c.md        'see ~/FakeLibraryDir/raw'
case_ "secret value"                1 d.md        'key: fake-secret-value-12345'
case_ "machine path inside peta/"   0 peta/e.env  'MOVA_BIG=/srv/fake-machine'
case_ "variable instead of a path"  0 f.md        'The corpus is in $MOVA_DATA/corpora/x.'
# --range: a path a branch would bring in by a merge
git -C "$T" -c user.name=t -c user.email=t@t commit -q --allow-empty -m base
git -C "$T" checkout -q -b side
printf '%s\n' 'data in /srv/fake-machine/x' > "$T/g.md"
git -C "$T" add g.md && git -C "$T" -c user.name=t -c user.email=t@t commit -q -m side
(cd "$T" && "$GUARD" --range HEAD~1..HEAD >/dev/null 2>&1); got=$?
if [ "$got" -eq 1 ]; then echo "ok   --range catches a path in a branch (exit 1)"; else echo "FAIL --range: exit $got, expected 1"; fail=1; fi
(cd "$T" && "$GUARD" --range HEAD..HEAD >/dev/null 2>&1); got=$?
if [ "$got" -eq 0 ]; then echo "ok   --range on an empty range (exit 0)"; else echo "FAIL --range empty: exit $got, expected 0"; fail=1; fi
exit $fail

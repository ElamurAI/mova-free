# mova-json

Deterministic JSON edits, merges and rebinds for Mova configs, registries, groupant manifests and JSONL logs. It replaces ad-hoc Python helpers in routine work.

Writes keep every unedited byte. On load the tool detects the layout (pretty with any indent, as `json.dumps(indent=N, ensure_ascii=False)` and `serde_json` write it; compact; trailing newline) and checks that it reproduces the file exactly. Key order, number lexemes and string escapes are kept. A file whose layout it cannot reproduce is refused for writing unless `--reformat` is given; `check` reports it (exit 4). On 2026-11-05, 4793 of the 4863 tracked `*.json` files were reproduced byte for byte.

## Build

```
CARGO_TARGET_DIR=... cargo build --offline --release --manifest-path tools/mova-json/Cargo.toml
cargo test --offline --release --manifest-path tools/mova-json/Cargo.toml
```

A standalone crate (own workspace and lock). Its only dependency is `sha2`. It parses JSON with its own ordered value type: enabling `serde_json/preserve_order` in the main workspace would reorder maps in every other crate.

## Commands

```
mova-json check FILE
mova-json get FILE PATH [--raw]
mova-json set FILE PATH VALUE [--str]
mova-json del FILE PATH
mova-json push FILE PATH VALUE... [--str]
mova-json insert-before FILE PATH ANCHOR VALUE... [--str]
mova-json remove-value FILE PATH VALUE [--str]
mova-json replace-str FILE OLD NEW
mova-json rebind REPO MANIFEST... --paths P... [--add]
mova-json merge3 FILE... [--rebind-all]
mova-json rebind-groupant REPO BASE_COMMIT FILE...
mova-json jsonl-count FILE PATH
mova-json jsonl-filter FILE PATH=VALUE
```

- **PATH**: `workers[name=dolphin-modeler].argv`, `capabilities.crab-health.parameters.socket`, `nodes[-1]`. A `[key=value]` selector must match exactly one object in the array. `\` escapes `.`, `[`, `]` and `=`. `""` or `.` is the root.
- **VALUE**: JSON text, or a plain string with `--str`. Use `--` to end flags before values that start with `--`:
  `mova-json insert-before crab.json 'workers[name=x].argv' --str -- -a --add-dir /some/dir`
- **set** inserts a missing final key at its sorted place when the object's keys are sorted, and at the end otherwise.
- **insert-before** needs an anchor that occurs exactly once. **remove-value** removes every equal element and fails when none matches.
- **replace-str** changes string values, never keys, and prints occurrences and strings changed.
- **rebind** refreshes `source_bindings[].sha256` for the listed repository paths in each manifest. With `--add` it inserts missing bindings sorted by path. Stale bindings that are not listed stay stale.
- **merge3** runs during a git merge (`MERGE_HEAD` present). It does a 3-way merge of base, HEAD and MERGE_HEAD:
  - Objects merge by key.
  - Arrays whose objects all have unique `(path, id, section, type, name)` keys merge by key.
  - Other arrays merge with diff3. Two insertions at the same point join only if no element repeats, so two different `--add-dir` additions conflict.
  - When both sides change `sha256` next to a `path`, the hash is recomputed from the working tree.
  - If any true conflict remains, it is printed, nothing is written and the exit code is 3. Run `git add` yourself.
  - `--rebind-all` refreshes every `{path, sha256}` pair, as the old Python helper did.
- **rebind-groupant** maps each `coverage.byte_range` in `FILE.groupant/**/manifest.json` through a line diff from BASE to the current source. It rehashes the covered bytes and the source, and rebinds tracked group manifests that bind FILE to its old hash. Fragments whose covered text changed are marked `TEXT CHANGED` and need a model review. Obernant `revision` fields are left alone.
- **jsonl-count** prints `count<TAB>value`, sorted by count descending. Lines without the field count as `<missing>`. **jsonl-filter** prints the matching lines.

Exit codes: 0 ok, 1 error, 2 usage, 3 merge conflicts, 4 `check` found a layout it cannot reproduce.

## Limits

- Duplicate keys are kept, not reported.
- Numbers compare by lexeme, so `1.0` and `1` differ.
- Pretty files with inline arrays, blank lines or tabs need `--reformat`.
- Byte ranges that change inside a line map to the boundaries of the unchanged block.
- `mergejson.py`'s two-way union merge has no equivalent.
- There is no locking against concurrent writers.

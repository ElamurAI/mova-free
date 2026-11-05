# mova-json tests

## domain
The crate's library under `cargo test`, the repository's real JSON files (peta files optional in public checkouts) and temporary git repositories under the system temp directory.

## forward
Twelve tests: real-file round trip with layout mutation controls; escape/lexeme preservation; path edits with ambiguity/missing controls; sorted insertion; replace_str; JSONL; merge clean cases; merge true conflicts with a same-change control; offset mapping with an identity control; rebind with stale/missing/non-canonical controls; rebind_groupant with a TEXT CHANGED control; merge3_file inside a real conflicting git merge.

## invariant
Each positive check has a paired red control in the same test or a recorded mutation run; temporary directories are per-process and removed by the test that made them.

## cost

About 0.1 s release, dominated by parsing contracts.json and git subprocesses.

## behavior-complement
Not covered: concurrent writers, files over hundreds of MB, Windows paths, criss-cross merges with several merge bases, non-UTF-8 JSON.

## reverse-reconstruction
A failing test name identifies the violated property; it does not identify the mutated line.

## checks

`cargo test --offline --release --manifest-path tools/mova-json/Cargo.toml`: 12 passed (two identical runs).

## epoch-memory

Initial suite with mutation controls recorded in merge.rs.groupant/model.md.

## History

### 2026-11-05 · cost

About 0.1 s release (2026-11-05), dominated by parsing contracts.json and git subprocesses.

### 2026-11-05 · checks

`cargo test --offline --release --manifest-path tools/mova-json/Cargo.toml`: 12 passed (2026-11-05, two identical runs).

### 2026-11-05 · epoch-memory

2026-11-05: initial suite with mutation controls recorded in merge.rs.groupant/model.md.

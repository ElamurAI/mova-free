# mova-json tool composition

## domain
The Octopus's routine JSON work in the repository and its run directories: config edits (Crab, deputy, dolphins), registry and groupant manifest maintenance, merges of branches that touch the same JSON, and log tallies. Children: library (value/layout/paths), merge (git merges, bindings, byte ranges), CLI, tests.

## forward
CLI parses argv and calls one library operation; edits go library-load -> path operation -> save; merge3/rebind/rebind-groupant compose the merge child with the library's load_for_edit/save, so every written file passes the same byte-identity gate. git is the only subprocess.

## invariant
Every write of an existing file preserves the bytes of everything not edited when the file was reproducible; otherwise the write is refused unless --reformat. Conflicts and ambiguous selections stop with a non-zero exit instead of choosing a side. Hashes are refreshed only for listed paths, settled hash conflicts, or explicit --rebind-all.

## cost
One file read and at most one write per edit; merges add three git show calls per file; costs are linear in file size except bounded LCS tables.

## behavior-complement
Not done: committing, staging, merging non-JSON files, verifying that a rebound model still describes the changed source, refreshing obernant revisions, editing files that Python-free tooling cannot reproduce without --reformat, any network or teacher call.

## reverse-reconstruction
The repository history plus the command line reproduce every output (deterministic order rules, no clocks or randomness). Outputs alone do not identify prior values for overwriting edits or the inputs of a clean merge.

## replaces-python-helpers
Type: independent_evidence_join. The scratch helpers merge3.py, mergejson.py, bind.py and rebind.py are replaced: merge3 (symmetric, reports true conflicts instead of resolving them to one side), rebind (bind.py with --add, for any manifests named), rebind-groupant (rebind.py, exact LCS instead of difflib). mergejson.py's union-with-prune two-way merge has no equivalent; merge3 needs the merge base.

## checks
`cargo test --offline --release --manifest-path tools/mova-json/Cargo.toml` (12 tests with red controls), CLI runs on copies of real configs, `mova-modelcheck groupants` on the registry.

## epoch-memory

Built after the operator's rule of no Python in routine work. Standalone crate (own `[workspace]` and lock) depending only on sha2, like tools/crab and tools/module-overlay.

## History

### 2026-11-05 · epoch-memory

2026-11-05: built after the operator's rule of no Python in routine work. Standalone crate (own `[workspace]` and lock) depending only on sha2, like tools/crab and tools/module-overlay.

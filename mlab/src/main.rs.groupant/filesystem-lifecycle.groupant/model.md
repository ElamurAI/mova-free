# mlab-main-filesystem-lifecycle

## descriptant

### mlab-main-filesystem-lifecycle

Build FsOpts, open backend and import memory roots, run code, export only if ok, then write journal; return status based on config/finalization versus interpreter.

## approximant

### Forward relation

Build FsOpts, open backend and import memory roots, run code, export only if ok, then write journal; return status based on config/finalization versus interpreter.

#### Domain

Valid Rust value/storage invariants and available bounded resources; exact branch and effect observations only.

#### Executable approximant gap

No useful percentage-error surrogate is established for these categorical, textual or effectful observations. Supported future caps are 1%, 2%, 5%, 10%; no executable approximation or production substitution is claimed.

## invariant

### Invariants and costs

Creation/import precedes interpreter execution; journal is after export attempts and can be absent if export errors.

#### Failure ordering

limits parse fails before transfer mode check; make_fs errors yield exit2; finish_fs errors also yield exit2 even after successful script.

#### Cost

O(argument scans plus delegated import/run/export/journal work); host writes not transactional.

## models

# Typed models — mlab-main-filesystem-lifecycle

Source-derived partial relations; finite checks do not prove universality.

## behavior_complement

ok=false excludes export_dir calls but allows journal output. Nonmem with transfer flags excludes make_fs.

## reverse_reconstruction

Successful run observations are compatible with repeated shadowed flags; skipped export message constrains ok=false but not original script cause.

## state_invariant

Creation/import precedes interpreter execution; journal is after export attempts and can be absent if export errors.

## failure_precedence

limits parse fails before transfer mode check; make_fs errors yield exit2; finish_fs errors also yield exit2 even after successful script.

## representation_transport

flag_values consumes adjacent windows, so dangling flags absent; imports/exports split first colon and default virtual root /.

## resource_cost

O(argument scans plus delegated import/run/export/journal work); host writes not transactional.

## tests

### Evidence and limitations

Parent source checks: cli_import_export_journal, cli_dir_mode_reads_and_writes_inside_root_only. The fragment's actual statements lie in the recorded byte range; parent checks need not exercise every branch in this fragment. See ../model.md#tests for source-level control and limitations.

AST projection generated twice to fresh files and compared byte-for-byte. Runtime evidence is finite and does not establish every branch, allocation failure, backend implementation or effect path. No optimization or speed claim.

## History

### 2026-10-31 · descriptant

#### Modeling epoch — 2026-10-31

Prospective source read preceded edits. Bounded synthetic checks and selected existing safe tests exercise actual code; paired debug/release results and compiled negative-control assertions are retained externally. Frozen scenario, corpus, external measurement, full-school and training effects are unexecuted. Partial lexical syntax supplies no macro-expanded or semantic proof.

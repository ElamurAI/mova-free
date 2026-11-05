# vfs-memory-snapshots

## descriptant

### vfs-memory-snapshots

Snapshot, restore, dump and fingerprint projection.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Snapshot, restore, dump and fingerprint projection.

Snapshot stores root Arc,cwd,clock,next_ino,usage,tmp and journals. Restore assigns saved fields except faulted clock then appends Restore. dump traverses BTree order and hashes with FNV.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Snapshot, restore, dump and fingerprint projection. Shared state and order: Snapshot stores root Arc,cwd,clock,next_ino,usage,tmp and journals. Restore assigns saved fields except faulted clock then appends Restore. dump traverses BTree order and hashes with FNV.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Journal history and live limits/fault persist across restore; from_snapshot clears restore journal.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-memory-snapshots. Domain: Snapshot, restore, dump and fingerprint projection. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Later edits cannot alter retained snapshot; normal restore cannot preserve changed clock.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-memory-snapshots. Domain: Snapshot, restore, dump and fingerprint projection. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Snapshot identifies saved represented state, not preceding history. Hash collides; dump omits limits and journal.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## journal_boundary

Subject: vfs-memory-snapshots. Domain: Snapshot, restore, dump and fingerprint projection. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Snapshot and Restore each append trace entries without rollback.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## policy_exclusion

Subject: vfs-memory-snapshots. Domain: Snapshot, restore, dump and fingerprint projection. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Limits and fault are not captured; restoring under changed limits retains them.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## ownership_cost

Subject: vfs-memory-snapshots. Domain: Snapshot, restore, dump and fingerprint projection. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Dropping last root Arc can recursively free nodes; O(1) claims require retained divergent roots and fixed cwd.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## fingerprint_loss

Subject: vfs-memory-snapshots. Domain: Snapshot, restore, dump and fingerprint projection. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: 64-bit FNV is finite observation, not collision-free inverse.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Journal history and live limits/fault persist across restore; from_snapshot clears restore journal.

## Cost

Clone cwd O(cwd bytes); shared root O(1); restore may drop divergent tree. dump O(serialized nodes+payload).

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: Clone cwd O(cwd bytes); shared root O(1); restore may drop divergent tree. dump O(serialized nodes+payload).

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

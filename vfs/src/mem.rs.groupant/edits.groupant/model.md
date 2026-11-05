# vfs-memory-edits

## descriptant

### vfs-memory-edits

Single-file content mutation, sparse offsets and quota preflight.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Single-file content mutation, sparse offsets and quota preflight.

Derive old size, new size by replace/append/At/truncate/touch; check arithmetic and quotas, tick, COW update, then usage bytes=old total-old size+new size.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Single-file content mutation, sparse offsets and quota preflight. Shared state and order: Derive old size, new size by replace/append/At/truncate/touch; check arithmetic and quotas, tick, COW update, then usage bytes=old total-old size+new size.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Rejected size/quota edits preserve dump; existing Touch does not rewrite metadata but does tick. New Touch creates empty file.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-memory-edits. Domain: Single-file content mutation, sparse offsets and quota preflight. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: offset MAX plus nonempty byte cannot wrap into an accepted low offset.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-memory-edits. Domain: Single-file content mutation, sparse offsets and quota preflight. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Replace output has arbitrary old payload antecedents; append with known suffix permits prefix recovery; truncation loses discarded suffix.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## error_order

Subject: vfs-memory-edits. Domain: Single-file content mutation, sparse offsets and quota preflight. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Trailing slash IsDir precedes path validation; node quota for new file precedes length quotas.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## sparse_zero

Subject: vfs-memory-edits. Domain: Single-file content mutation, sparse offsets and quota preflight. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: At beyond EOF fills holes with zero; read_at beyond EOF returns empty.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## touch_projection

Subject: vfs-memory-edits. Domain: Single-file content mutation, sparse offsets and quota preflight. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Touch existing file allows absent write bit and leaves file times, but clock advances.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## quota_growth

Subject: vfs-memory-edits. Domain: Single-file content mutation, sparse offsets and quota preflight. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Total cap enforced only on growth; shrinking remains allowed after lower limits.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Rejected size/quota edits preserve dump; existing Touch does not rewrite metadata but does tick. New Touch creates empty file.

## Cost

COW shared parent map clones plus new content; append/At/truncate can clone old full buffer. Zero-filled hole costs offset-oldlen bytes.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: COW shared parent map clones plus new content; append/At/truncate can clone old full buffer. Zero-filled hole costs offset-oldlen bytes.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

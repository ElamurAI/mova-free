# vfs-memory-transfers

## descriptant

### vfs-memory-transfers

Explicit real-directory import/export and recursive helpers.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Explicit real-directory import/export and recursive helpers.

Import sorts real names, copies regular bytes, carries link text without reading target; export creates destination and writes regular nodes, skips memory links, rejects encountered real child symlinks.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Explicit real-directory import/export and recursive helpers. Shared state and order: Import sorts real names, copies regular bytes, carries link text without reading target; export creates destination and writes regular nodes, skips memory links, rejects encountered real child symlinks.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Counters describe successful prefix; first error stops with earlier effects retained. Root destination/ancestor trust required; no transactional or race-safe export promise.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-memory-transfers. Domain: Explicit real-directory import/export and recursive helpers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Import does not dereference child symlink target; export encountering child symlink cannot overwrite through it.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-memory-transfers. Domain: Explicit real-directory import/export and recursive helpers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Roundtrip recovers regular bytes under clean destination; skipped links/devices/nonUTF8 names and metadata are unrecoverable.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## sorted_prefix

Subject: vfs-memory-transfers. Domain: Explicit real-directory import/export and recursive helpers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Import iteration sorted names; errors leave earlier imported nodes.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## transfer_counts

Subject: vfs-memory-transfers. Domain: Explicit real-directory import/export and recursive helpers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Dirs exclude initial source root; bytes count successful regular payloads, links separately.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## host_preflight

Subject: vfs-memory-transfers. Domain: Explicit real-directory import/export and recursive helpers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Import metadata file cap checked before read, but concurrent size change remains possible.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## export_boundary

Subject: vfs-memory-transfers. Domain: Explicit real-directory import/export and recursive helpers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Child symlink check precedes kind dispatch; trusted root ancestors and stable filesystem required.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Counters describe successful prefix; first error stops with earlier effects retained. Root destination/ancestor trust required; no transactional or race-safe export promise.

## Cost

O(total bytes+nodes+sum k log k import sorting), recursive stack depth and host syscall costs.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(total bytes+nodes+sum k log k import sorting), recursive stack depth and host syscall costs.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

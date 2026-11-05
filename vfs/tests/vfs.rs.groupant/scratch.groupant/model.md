# vfs-gates-scratch

## descriptant

### vfs-gates-scratch

Temporary fixture creation and destruction.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Temporary fixture creation and destruction.

scratch joins TMPDIR with PID/name, best-effort removes old path then creates directory; Drop best-effort recursively removes same root.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Temporary fixture creation and destruction. Shared state and order: scratch joins TMPDIR with PID/name, best-effort removes old path then creates directory; Drop best-effort recursively removes same root.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Selected calls use fixed internal names and dedicated TMPDIR; no external caller-controlled paths admitted.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-gates-scratch. Domain: Temporary fixture creation and destruction. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: No fixture path intentionally leaves dedicated TMPDIR for selected constant names; cleanup failure is not a panic.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-gates-scratch. Domain: Temporary fixture creation and destruction. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Existing missing vs removed prior directory produce same newly created root; cleanup success is not observable from Drop.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## name_domain

Subject: vfs-gates-scratch. Domain: Temporary fixture creation and destruction. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Name is internal finite fixture identifier, not arbitrary traversal-bearing input.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## process_identity

Subject: vfs-gates-scratch. Domain: Temporary fixture creation and destruction. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: PID separates concurrent test processes; tests within process run serially.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## best_effort_drop

Subject: vfs-gates-scratch. Domain: Temporary fixture creation and destruction. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Drop ignores remove error; tests do not guarantee cleanup under process crash.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## host_scope

Subject: vfs-gates-scratch. Domain: Temporary fixture creation and destruction. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: All dirfs outside targets remain siblings under same owned TMPDIR.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Selected calls use fixed internal names and dedicated TMPDIR; no external caller-controlled paths admitted.

## Cost

Creation/removal O(number of owned entries plus host I/O), PathBuf allocation O(path bytes).

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: Creation/removal O(number of owned entries plus host I/O), PathBuf allocation O(path bytes).

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

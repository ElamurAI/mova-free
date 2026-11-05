# vfs-gates

## descriptant

### vfs-gates

Finite behavior gates and owned temporary fixture cleanup.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Finite behavior gates and owned temporary fixture cleanup.

scratch creates TMPDIR/vfs-test-PID-name after best-effort removal; Drop best-effort removes own root. Tests instantiate bounded Vfs, DirFs and transfer fixtures; named gates compare expected result then inject Fault variants.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Finite behavior gates and owned temporary fixture cleanup. Shared state and order: scratch creates TMPDIR/vfs-test-PID-name after best-effort removal; Drop best-effort removes own root. Tests instantiate bounded Vfs, DirFs and transfer fixtures; named gates compare expected result then inject Fault variants.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Selected tests use only owned TMPDIR host data; absolute_paths_never_reach_the_host is excluded for actual /etc/passwd read; open_modes excluded for external nonexistent-root probe. Runtime Fault toggles are not compiled mutants.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-gates. Domain: Finite behavior gates and owned temporary fixture cleanup. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Selected host fixture cleanup never intentionally targets outside dedicated TMPDIR; a passing finite gate does not establish universal confinement, allocation or concurrency semantics.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-gates. Domain: Finite behavior gates and owned temporary fixture cleanup. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Pass outcomes admit all implementations agreeing on finite fixtures, not unique implementation. Failed gate identifies violated observation but may admit many source defects.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## fixture_ownership

Subject: vfs-gates. Domain: Finite behavior gates and owned temporary fixture cleanup. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Scratch path combines fixed name and process id under dedicated TMPDIR. Same-process repeated names are sequential; cleanup errors ignored.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## fault_oracle

Subject: vfs-gates. Domain: Finite behavior gates and owned temporary fixture cleanup. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Five existing Fault checks require gates Err; compiled source mutants are a separate evidence class.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## snapshot_witness

Subject: vfs-gates. Domain: Finite behavior gates and owned temporary fixture cleanup. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: dump comparison observes state+counters but excludes journal, limits and fault; retained Arc bytes checked separately.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## partial_prefix_witness

Subject: vfs-gates. Domain: Finite behavior gates and owned temporary fixture cleanup. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: wave31 sparse fixture proves failed mkdir_all(/a/b) can leave /a, while checked write_at overflow leaves dump unchanged.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Selected tests use only owned TMPDIR host data; absolute_paths_never_reach_the_host is excluded for actual /etc/passwd read; open_modes excluded for external nonexistent-root probe. Runtime Fault toggles are not compiled mutants.

## Cost

O(fixture bytes+tree work), maximum selected populate blob100000 bytes; four thread sandboxes in isolation test; no benchmark main.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(fixture bytes+tree work), maximum selected populate blob100000 bytes; four thread sandboxes in isolation test; no benchmark main.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

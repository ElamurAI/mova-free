# vfs-memory-resolution

## descriptant

### vfs-memory-resolution

Canonical path stack and symlink expansion only.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Canonical path stack and symlink expansion only.

Queue parts, consume dotdot against stack, expand target components at front; absolute link resets root, missing final component succeeds; >40 hops fails Loop.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Canonical path stack and symlink expansion only. Shared state and order: Queue parts, consume dotdot against stack, expand target components at front; absolute link resets root, missing final component succeeds; >40 hops fails Loop.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Stack/nodes depth align; no root pop under Fault=None; final nonfollow link retained.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-memory-resolution. Domain: Canonical path stack and symlink expansion only. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Under normal fault no above-root dotdot succeeds, and hop41 does not resolve.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-memory-resolution. Domain: Canonical path stack and symlink expansion only. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Canonical components admit textual aliases and link chains; missing final remains compatible with later creation or NoEnt lookup.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## hop_budget

Subject: vfs-memory-resolution. Domain: Canonical path stack and symlink expansion only. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Each followed Link increments hops before >40 check.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## missing_partition

Subject: vfs-memory-resolution. Domain: Canonical path stack and symlink expansion only. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Missing last component returns components; missing intermediate returns NoEnt.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## follow_last

Subject: vfs-memory-resolution. Domain: Canonical path stack and symlink expansion only. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: follow_last false preserves final link but still follows interior links.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## absolute_reset

Subject: vfs-memory-resolution. Domain: Canonical path stack and symlink expansion only. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Absolute target clears stack and truncates node references to root.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Stack/nodes depth align; no root pop under Fault=None; final nonfollow link retained.

## Cost

O(expanded path bytes+sum log fanout), queue stores pending expansion;40-link cap not total-input cap.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(expanded path bytes+sum log fanout), queue stores pending expansion;40-link cap not total-input cap.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

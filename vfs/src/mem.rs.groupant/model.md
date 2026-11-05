# vfs-memory

## descriptant

### vfs-memory

Arc tree, symlink resolution, edits, snapshots and explicit transfers.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

Integration review narrows the quota complement to single-file mutations. Compound mkdir/import/export paths retain their separately modeled successful prefixes.

## approximant

### Forward relation

Arc tree, symlink resolution, edits, snapshots and explicit transfers.

State S=(root,cwd,clock,next_ino,usage,tmp;limits,journal,fault). Resolution expands links at most40 hops. Validated edits tick then COW mutate; file bytes and node counts update. snapshot captures first6 fields while journal records; restore reuses root and copies cwd, restoring counters except deliberate fault.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Arc tree, symlink resolution, edits, snapshots and explicit transfers. Shared state and order: State S=(root,cwd,clock,next_ino,usage,tmp;limits,journal,fault). Resolution expands links at most40 hops. Validated edits tick then COW mutate; file bytes and node counts update. snapshot captures first6 fields while journal records; restore reuses root and copies cwd, restoring counters except deliberate fault.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Usage counts file data and all nonroot nodes, not link text. Single-file quota failures leave dump unchanged but journal records. mkdir_all and transfers can partially commit. Journal/limits/fault are outside snapshot.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-memory. Domain: Arc tree, symlink resolution, edits, snapshots and explicit transfers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: For single-file mutations, Fault=None quota rejection cannot install an over-limit file or advance the edit clock. Snapshot isolation excludes later writes changing retained root. Absolute virtual paths never directly invoke host except explicit import/export.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-memory. Domain: Arc tree, symlink resolution, edits, snapshots and explicit transfers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Restored snapshot determines represented state but admits any intervening mutation/journal history; limits/fault remain external. Reads admit any unseen tree; overwrite loses old bytes; dump/fingerprint do not uniquely identify histories.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## quota_order

Subject: vfs-memory. Domain: Arc tree, symlink resolution, edits, snapshots and explicit transfers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Node quota checked for new file before size, checked off+len before FBig, then total growth before mutation. Arithmetic overflow/allocation beyond bounded fixtures remains gap.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## cow_ownership

Subject: vfs-memory. Domain: Arc tree, symlink resolution, edits, snapshots and explicit transfers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Root and child Arc sharing preserves snapshots; copying maps scales with fanout, not only depth. Shared read buffers retain old bytes.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## partial_effects

Subject: vfs-memory. Domain: Arc tree, symlink resolution, edits, snapshots and explicit transfers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: mkdir_all creates prefixes incrementally; import/export stop after first error with earlier effects retained. No all-operation transaction claim.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## snapshot_projection

Subject: vfs-memory. Domain: Arc tree, symlink resolution, edits, snapshots and explicit transfers. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Snapshot excludes limits,journal,fault. cwd cloning and dropping divergent roots refute unconditional O(1) wording; finite tests establish state restoration only.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Usage counts file data and all nonroot nodes, not link text. Single-file quota failures leave dump unchanged but journal records. mkdir_all and transfers can partially commit. Journal/limits/fault are outside snapshot.

## Cost

Resolve O(path+expanded link bytes+sum log fanout); COW clones shared directory maps O(sum fanouts), shared content O(old bytes) on append/At/truncate. Snapshot O(cwd bytes); restore also drops unshared divergent tree. dump O(tree+content+path output).

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: Resolve O(path+expanded link bytes+sum log fanout); COW clones shared directory maps O(sum fanouts), shared content O(old bytes) on append/At/truncate. Snapshot O(cwd bytes); restore also drops unshared divergent tree. dump O(tree+content+path output).

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

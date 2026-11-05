# vfs-memory-directories

## descriptant

### vfs-memory-directories

Metadata, directory mutations, rename, links and cwd.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Metadata, directory mutations, rename, links and cwd.

stat follows optionally and rejects trailing slash nondirectory. mkdir_all iterates prefixes. remove computes subtree weight before unlink. rename resolves source/destination, rejects root/self-descendant/type mismatch, checks both parents then removes source and inserts same Arc, subtracting overwritten weight. chmod masks07777; set_mtime updates ctime to tick; chdir changes cwd.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Metadata, directory mutations, rename, links and cwd. Shared state and order: stat follows optionally and rejects trailing slash nondirectory. mkdir_all iterates prefixes. remove computes subtree weight before unlink. rename resolves source/destination, rejects root/self-descendant/type mismatch, checks both parents then removes source and inserts same Arc, subtracting overwritten weight. chmod masks07777; set_mtime updates ctime to tick; chdir changes cwd.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Rename same canonical path succeeds without tick; nonempty destination directory rejected. Removing root Busy. read_dir uses sorted BTree names; chmod/read checks owner bits, no execute-bit traversal model.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-memory-directories. Domain: Metadata, directory mutations, rename, links and cwd. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Directory cannot be moved beneath itself; ordinary remove_dir cannot delete nonempty directory; root removal does not succeed.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-memory-directories. Domain: Metadata, directory mutations, rename, links and cwd. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Rename loses overwritten destination and old name; metadata times can map many operations to same saturating clock. Sorted entries omit mutation chronology.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## rename_order

Subject: vfs-memory-directories. Domain: Metadata, directory mutations, rename, links and cwd. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Destination resolve occurs before source lookup; identical path returns before parent write checks.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## remove_weight

Subject: vfs-memory-directories. Domain: Metadata, directory mutations, rename, links and cwd. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Usage subtracts all regular bytes and all removed nodes including directory; symlink text contributes0 bytes.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## permission_scope

Subject: vfs-memory-directories. Domain: Metadata, directory mutations, rename, links and cwd. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Only owner read/write bits checked; access traversal is not full POSIX permission semantics.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## partial_mkdir

Subject: vfs-memory-directories. Domain: Metadata, directory mutations, rename, links and cwd. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Earlier created prefixes persist if later quota/type/path check fails.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: escape_by_dotdot_is_denied, symlinks_cannot_lead_outside, bad_names_and_paths, limits_fire_with_clear_errors_and_leave_state_unchanged, limits_parse, snapshot_changes_restore_is_byte_identical, snapshot_is_isolated_copy_on_write, restore_after_failed_ops_and_limits, posix_like_semantics, virtual_clock_is_deterministic, journal_records_ops_errors_and_denials, import_export_round_trip, dirfs_confines_to_root, gates_green_and_negative_controls_red, wave31_sparse_copy_and_partial_directory_failure.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Rename same canonical path succeeds without tick; nonempty destination directory rejected. Removing root Busy. read_dir uses sorted BTree names; chmod/read checks owner bits, no execute-bit traversal model.

## Cost

Directory reads O(k+names); recursive weight O(subtree nodes); rename path lookups plus COW parent map copies; mkdir_all repeats prefix resolution, potentially quadratic path work.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: Directory reads O(k+names); recursive weight O(subtree nodes); rename path lookups plus COW parent map copies; mkdir_all repeats prefix resolution, potentially quadratic path work.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

# vfs-host-confinement

## descriptant

### vfs-host-confinement

DirFs path preflight and journal wrapper.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

DirFs path preflight and journal wrapper.

normalize virtual path then append components under canonical root. Final symlink must canonicalize inside; otherwise probe ancestors until canonicalization succeeds. with calls operation only after successful real_path.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

DirFs path preflight and journal wrapper. Shared state and order: normalize virtual path then append components under canonical root. Final symlink must canonicalize inside; otherwise probe ancestors until canonicalization succeeds. with calls operation only after successful real_path.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Stable root required; failed preflight recorded by with and OS closure not called.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-host-confinement. Domain: DirFs path preflight and journal wrapper. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Outside final or ancestor link does not reach OS closure when Fault=None.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-host-confinement. Domain: DirFs path preflight and journal wrapper. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Accepted canonical path admits virtual aliases; denied result cannot uniquely identify malicious vs dangling target.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## lexical_first

Subject: vfs-host-confinement. Domain: DirFs path preflight and journal wrapper. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Above-root dotdot rejects before host canonicalization.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## dangling_link

Subject: vfs-host-confinement. Domain: DirFs path preflight and journal wrapper. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Final symlink canonicalization failure returns escape rather than following missing target.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## ancestor_probe

Subject: vfs-host-confinement. Domain: DirFs path preflight and journal wrapper. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Nonexistent regular child uses canonical deepest existing ancestor.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## check_access_race

Subject: vfs-host-confinement. Domain: DirFs path preflight and journal wrapper. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: No atomic descriptor confinement between canonicalize and std IO.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: dirfs_confines_to_root, gates_green_and_negative_controls_red.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Stable root required; failed preflight recorded by with and OS closure not called.

## Cost

Path construction O(bytes); up to depth canonicalize probes with OS-dependent work.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: Path construction O(bytes); up to depth canonicalize probes with OS-dependent work.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

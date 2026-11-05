# vfs-snapbench

## descriptant

### vfs-snapbench

Timing helpers and unexecuted snapshot workload.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Timing helpers and unexecuted snapshot workload.

per_op(n,f) invokes f exactly n times then divides elapsed seconds by n; fmt chooses ns if s<1e-6, microseconds if s<1e-3 else ms. Main scales1000/10000/100000 files plus64MiB and compares snapshot,restore,write,dump timings.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Timing helpers and unexecuted snapshot workload. Shared state and order: per_op(n,f) invokes f exactly n times then divides elapsed seconds by n; fmt chooses ns if s<1e-6, microseconds if s<1e-3 else ms. Main scales1000/10000/100000 files plus64MiB and compares snapshot,restore,write,dump timings.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. For n>0 bounded helper elapsed is finite nonnegative; timing is nondeterministic. fmt boundaries use strict less-than. n0 remains undefined useful average (infinite/NaN possible).

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-snapbench. Domain: Timing helpers and unexecuted snapshot workload. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: At exactly1e-6 output cannot be ns, at1e-3 cannot be microseconds. Helper tests do not validate benchmark speed claims.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-snapbench. Domain: Timing helpers and unexecuted snapshot workload. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Formatted rounded duration admits intervals and cannot reconstruct raw clock or workload timing distribution. Per-op mean admits many individual duration sequences.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## iteration_count

Subject: vfs-snapbench. Domain: Timing helpers and unexecuted snapshot workload. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: n3 calls closure3 times; n0 invokes none but divides by0, outside useful timing domain.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## format_partition

Subject: vfs-snapbench. Domain: Timing helpers and unexecuted snapshot workload. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Strict thresholds at1e-6 and1e-3; rounding precision differs by branch.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## benchmark_scope

Subject: vfs-snapbench. Domain: Timing helpers and unexecuted snapshot workload. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: black_box prevents easy elimination but timings include closure/drop effects; no portable complexity measurement follows.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## allocation_domain

Subject: vfs-snapbench. Domain: Timing helpers and unexecuted snapshot workload. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Largest main fixture and million iterations skipped; only three no-allocation closure invocations tested.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: wave31::bounded_helpers.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

For n>0 bounded helper elapsed is finite nonnegative; timing is nondeterministic. fmt boundaries use strict less-than. n0 remains undefined useful average (infinite/NaN possible).

## Cost

per_op O(n*closure cost), main allocation O(N*1024+64MiB), repeated shared-buffer edits copy64MiB; main intentionally not executed.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: per_op O(n*closure cost), main allocation O(N*1024+64MiB), repeated shared-buffer edits copy64MiB; main intentionally not executed.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

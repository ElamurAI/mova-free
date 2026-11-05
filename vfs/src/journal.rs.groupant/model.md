# vfs-journal

## descriptant

### vfs-journal

Bounded trace retention and counters.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Bounded trace retention and counters.

record first increments seq and classified error counters; room=cap for success or saturating_mul(cap,2) for error. If len>=room drop++, else append Entry(seq,time,op,path,bytes,errno). clear resets counters while retaining cap.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Bounded trace retention and counters. Shared state and order: record first increments seq and classified error counters; room=cap for success or saturating_mul(cap,2) for error. If len>=room drop++, else append Entry(seq,time,op,path,bytes,errno). clear resets counters while retaining cap.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Before u64 overflow, total=retained+dropped; errors/denied/limited count dropped events too. Retained seq increases but may have gaps. cap0 drops even errors.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-journal. Domain: Bounded trace retention and counters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: At full cap ordinary success cannot append; error counters cannot omit dropped errors. Error headroom is bounded and does not promise every error retained.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-journal. Domain: Bounded trace retention and counters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Retained entries plus counters admit many dropped op/path/time traces. TSV replaces tab/newline with spaces, so original path has ambiguous fibers. clear erases all histories.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## retention_budget

Subject: vfs-journal. Domain: Bounded trace retention and counters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: room=cap or saturating2cap; error acceptance depends on current total retained length, not separate error quota.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## counter_conservation

Subject: vfs-journal. Domain: Bounded trace retention and counters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: seq=entries.len+dropped for nonoverflowing calls after clear; errors<=seq, denied<=errors, limited<=errors.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tsv_loss

Subject: vfs-journal. Domain: Bounded trace retention and counters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Path tabs and newlines map to spaces; TSV is observation-only, not a lossless event codec.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reset_projection

Subject: vfs-journal. Domain: Bounded trace retention and counters. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: clear keeps cap but erases seq/errors/denied/limited/dropped/entries; no replay state survives.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: journal::wave31::capped_trace_and_projection, journal_records_ops_errors_and_denials.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Before u64 overflow, total=retained+dropped; errors/denied/limited count dropped events too. Retained seq increases but may have gaps. cap0 drops even errors.

## Cost

O(path bytes) per retained record amortized, O(1) dropped record; memory O(sum retained path bytes+min(events,2cap)); TSV/summary output proportional to retained serialization.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(path bytes) per retained record amortized, O(1) dropped record; memory O(sum retained path bytes+min(events,2cap)); TSV/summary output proportional to retained serialization.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

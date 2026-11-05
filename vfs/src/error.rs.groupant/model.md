# vfs-error

## descriptant

### vfs-error

Errno numberings, error details and OS transport.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Errno numberings, error details and OS transport.

linux(Perm)=linux(NotCapable)=1; wasi distinguishes 63 and76. from_linux scans ALL excluding NotCapable; from_wasi scans all23. Display(detail)=detail, otherwise text plus Linux code. Unknown OS errno becomes Io with original display.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Errno numberings, error details and OS transport. Shared state and order: linux(Perm)=linux(NotCapable)=1; wasi distinguishes 63 and76. from_linux scans ALL excluding NotCapable; from_wasi scans all23. Display(detail)=detail, otherwise text plus Linux code. Unknown OS errno becomes Io with original display.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Within ALL, WASI roundtrip preserves tags; Linux roundtrip preserves all except NotCapable becomes Perm. Detail overrides tag text without changing errno.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-error. Domain: Errno numberings, error details and OS transport. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Under declared enum universe, WASI decoding does not map76 to Perm; known Linux2 does not become Io. Detail-bearing display cannot append automatic OS suffix.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-error. Domain: Errno numberings, error details and OS transport. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Linux code1 has compatible tags {Perm,NotCapable}; from_linux chooses Perm. Display with arbitrary detail cannot reconstruct errno or OS cause; recognized WASI codes recover exactly one tag.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## errno_quotient

Subject: vfs-error. Domain: Errno numberings, error details and OS transport. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Linux transport forms one non-singleton fiber {Perm,NotCapable}; this is intentional information loss, not inverse equality.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## wasi_bijection

Subject: vfs-error. Domain: Errno numberings, error details and OS transport. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: On 23 emitted u16 codes, from_wasi(wasi(e))=Some(e); unknown code has None.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## detail_precedence

Subject: vfs-error. Domain: Errno numberings, error details and OS transport. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: FsError{errno,Some(d)} displays d even when empty; default text depends on errno only when None.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## limit_partition

Subject: vfs-error. Domain: Errno numberings, error details and OS transport. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: is_limit true exactly for FBig,NoSpc,DQuot; root escape belongs to denied category instead.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: error::wave31::errno_fibers_and_display.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Within ALL, WASI roundtrip preserves tags; Linux roundtrip preserves all except NotCapable becomes Perm. Detail overrides tag text without changing errno.

## Cost

O(1) fixed23 variants/scans; O(detail bytes) owned string and display.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(1) fixed23 variants/scans; O(detail bytes) owned string and display.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

# vfs-path

## descriptant

### vfs-path

Lexical path validation and normalization.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Lexical path validation and normalization.

check rejects empty, NUL, then length>4096 in that order. normalize starts at [] for absolute else cwd clone; drops empty/dot; dotdot pops or escapes unless clamp; other components checked<=255 bytes. join([])=/.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Lexical path validation and normalization. Shared state and order: check rejects empty, NUL, then length>4096 in that order. normalize starts at [] for absolute else cwd clone; drops empty/dot; dotdot pops or escapes unless clamp; other components checked<=255 bytes. join([])=/.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Successful normalization under clamp=false never pops above root. UTF8 lengths are bytes; lexical normalization does not resolve links. Caller-provided cwd is assumed canonical.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-path. Domain: Lexical path validation and normalization. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: After empty/NUL validation, escape produces NotCapable rather than clamping; paths with component256 bytes do not succeed. Host containment is not implied.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-path. Domain: Lexical path validation and normalization. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Normalized /b/c admits /b//c, /b/./c and relative spellings with suitable cwd; canceled prefix information cannot be reconstructed. Empty output admits /, /., and balanced paths.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## failure_precedence

Subject: vfs-path. Domain: Lexical path validation and normalization. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: NUL wins over total length; empty wins NoEnt. Component check follows lexical dot/dotdot dispatch.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## byte_coordinates

Subject: vfs-path. Domain: Lexical path validation and normalization. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: NAME_MAX255 and PATH_MAX4096 count UTF8 bytes:128 copies of é exceed NAME_MAX.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## cwd_transport

Subject: vfs-path. Domain: Lexical path validation and normalization. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Absolute input discards cwd; relative input clones it. normalize(join(canonical), absolute) has canonical component image.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## textual_helpers

Subject: vfs-path. Domain: Lexical path validation and normalization. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: child and base_name do not validate names, resolve dots or prevent prefix injection; child(/,a)=/a.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: path::wave31::lexical_boundaries_and_fibers.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Successful normalization under clamp=false never pops above root. UTF8 lengths are bytes; lexical normalization does not resolve links. Caller-provided cwd is assumed canonical.

## Cost

O(path bytes+cwd bytes) time/storage; join/child/base_name linear in observed string lengths.

No timing or numerical error guarantee is inferred from finite tests.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(path bytes+cwd bytes) time/storage; join/child/base_name linear in observed string lengths.

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

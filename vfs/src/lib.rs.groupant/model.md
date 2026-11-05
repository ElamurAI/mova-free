# vfs-api

## descriptant

### vfs-api

Public types, limits parser, FileSystem dispatch and hashing.

#### Observation epoch

Bounded actual calls and compiled negative controls are recorded in model.md#tests. Production behavior bytes unchanged; only focused test blocks added. Snapshot constant-time wording is conditional on cwd and destruction; recursive operations may partially commit.

#### Unsupported approximants

No executable percentage surrogate is claimed for these categorical/state/effect observations. Future 1%,2%,5%,10% require a useful metric, explicit domain and exact reference.

## approximant

### Forward relation

Public types, limits parser, FileSystem dispatch and hashing.

parse_size trims, parses u64, multiplies by 1024^suffix with checked_mul. Limits starts defaults and applies comma entries in order. open mem constructs Vfs+limits; real constructs RealFs; dir canonicalizes nonempty root. Trait copy read then write returns byte length; read_to_string UTF8-decodes successful read.

Domain: bounded inputs with available allocation and nonoverflowing counters unless checked explicitly. The relation is partial and source-derived.

## composition

### Composition

Public types, limits parser, FileSystem dispatch and hashing. Shared state and order: parse_size trims, parses u64, multiplies by 1024^suffix with checked_mul. Limits starts defaults and applies comma entries in order. open mem constructs Vfs+limits; real constructs RealFs; dir canonicalizes nonempty root. Trait copy read then write returns byte length; read_to_string UTF8-decodes successful read.

Scoped children project this same source. Their domains and effects remain conditional; source execution connects these clauses rather than independently proving their wiring. Duplicate limit keys last-win. exists collapses every stat error to false. FNV uses wrapping u64 product. Limits affect mem dispatch only.

## models

# Typed source models

Source-bound partial relations; no universal finite-test proof.

## behavior_complement

Subject: vfs-api. Domain: Public types, limits parser, FileSystem dispatch and hashing. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Invalid/overflow size cannot silently wrap. copy cannot write destination when source read errors. Invalid UTF8 cannot produce a successful String.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## reverse_reconstruction

Subject: vfs-api. Domain: Public types, limits parser, FileSystem dispatch and hashing. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: Same Limits admits omitted defaults, whitespace and repeated overwritten keys. exists=false admits missing/denied/type errors; copy length admits arbitrary equal-length data. Hash preimages collide.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## checked_units

Subject: vfs-api. Domain: Public types, limits parser, FileSystem dispatch and hashing. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: K/M/G are powers1024, case-insensitive; empty/nondecimal/overflow fail, zero suffix remains zero.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## dispatch_tags

Subject: vfs-api. Domain: Public types, limits parser, FileSystem dispatch and hashing. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: mem/real/dir are tagged independent backends. RealFs construction does not authorize arbitrary host operations.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## trait_order

Subject: vfs-api. Domain: Public types, limits parser, FileSystem dispatch and hashing. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: copy reads all bytes before write and reports count only after write success; failures are not transactional rollback.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## fnv_fold

Subject: vfs-api. Domain: Public types, limits parser, FileSystem dispatch and hashing. Valid allocations, bounded counters, Fault=None unless explicitly named. Observation: one call or the stated finite trace.

Operator: h0=0xcbf29ce484222325; h(i+1)=(h(i) xor byte)*0x100000001b3 mod2^64; fixed-size hashes are not unique state identities.

Approximation: source-derived partial relation; complement is an inner excluded-behavior claim under these premises, converse retains compatible antecedents. Evidence: model.md#tests; unobserved concurrency/resource failures remain unknown.

## tests

### Checks and limitations

Exact named observations: wave31::parsing_and_trait_composition, limits_parse.

These finite observations are not semantic completeness. Source syntax generated twice and compared byte-for-byte. Host effects are limited to owned TMPDIR fixtures. Existing absolute_paths_never_reach_the_host and open_modes are skipped; snapbench main is skipped. Standalone helpers/reexports are only observed through named callers or representation checks; RealFs effect methods unexecuted.

## invariant-and-cost

# Invariant and cost

## Invariant

Duplicate limit keys last-win. exists collapses every stat error to false. FNV uses wrapping u64 product. Limits affect mem dispatch only.

## Cost

O(input bytes) parsing, O(file bytes) trait copy buffer; OS/memory backend costs remain separate; FNV O(bytes)/O(1).

No timing or numerical error guarantee is inferred from finite tests.

## wave31-evidence

### Wave31 VFS evidence

Eight existing Rust sources gain adjacent groupants, with nine nested fragments and 102 typed views. Each node explicitly references its cost component. Eight partial lexical syntax projections were generated twice and compared byte-for-byte; fragments reuse their parent's syntax and carry exact byte ranges and hashes. Six existing sources receive focused inline tests; no new Rust source or production behavior change is introduced. The original source bytes remain an exact prefix of each current file.

Seven compiled actual-source behavior mutants reject through named runtime assertions: errno mapping, lexical escape, journal headroom, binary suffix multiplication, restore clock, host confinement and duration format threshold. Every mutant has compile exit 0, assertion-test exit 101, exact original/restored SHA256 equality and restored positive exit 0. These are six library implementation mutants and one example-helper mutant; no assertion-only mutant is counted. The existing integration test also checks five runtime Fault toggles; those are a separate evidence class. mutation-summary.json retains the mapping and byte-restoration evidence.

Host fixtures use a dedicated TMPDIR. The existing absolute_paths_never_reach_the_host test is skipped because it reads the actual host password file; open_modes is skipped because it probes an unrelated nonexistent root. Import tests carry an absolute symlink target as text without following it. All intentionally accessible outside-root targets are synthetic sibling directories within the owned TMPDIR. RealFs effect methods and the large snapbench main remain unexecuted; only bounded example helpers are tested.

Prospective review corrected two cost/effect simplifications: snapshot clones cwd, and restore can destroy an unshared divergent tree, so unconditional constant time is not established; mkdir_all, import and export may preserve successful prefixes after later failures. These observations change models, not production implementation. Host confinement still assumes no concurrent directory replacement; safe temporary prefixes and trusted transfer destination roots remain domain conditions. Unsigned counter exhaustion, allocation failure and complete POSIX behavior remain gaps.

The local augmented-registry structural check passes with no diagnostics. Its one missing group sidecar is an external synthetic review root; the actual integration parent and shared registry are owned by the root agent. Earlier local check failures are retained: a leaf-only registry lacked unrelated ownership, and the composition field was initially represented incorrectly; corrected path-backed composition artifacts pass. This structural result is not semantic verification.

Strict scoped Clippy stops on four existing production lints: three collapsible conditions (host.rs and mem.rs) and mem.rs type complexity. These source regions are unchanged; no strict Clippy success is claimed. Only appended test blocks were formatted. No corpus, live teacher, benchmark throughput, WASI guest, GPU, database or executable percentage-approximation result is claimed.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31: source reviewed before edits; prospective review retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31/prospective.md. Predicted order/cost: O(input bytes) parsing, O(file bytes) trait copy buffer; OS/memory backend costs remain separate; FNV O(bytes)/O(1).

### 2026-10-31 · tests

Fresh receipts: $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31; run.sh retains four profile/repetition sets with raw logs and numeric statuses. native-commands.json and source-test-map.json disambiguate targets. Compiled mutations and exact restoration are separately retained in mutation-summary.json.

### 2026-10-31 · wave31-evidence

Heading `### Wave31 VFS evidence — 2026-10-31` stays in wave31-evidence as `### Wave31 VFS evidence`.

Twenty exact named tests pass twice in debug and twice in release: four library tests, fifteen integration tests and one example-helper test. Sorted actual test outcomes match across all four runs. Numeric exit files and raw logs are retained under $MOVA_DATA/runs/octopus-wave31-vfs-2026-10-31, together with native-commands.json, source-test-map.json, production-prefix-proof.json and outcomes-equality.status.

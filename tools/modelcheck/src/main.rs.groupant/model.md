# modelcheck-cli

## descriptant

### Modelcheck CLI groupant

Invariant: outputs use create_new and cannot replace an existing file; errors
propagate. No teacher or training calls occur. Cost is library inventory cost
plus O(serialized report bytes) time/space; dispatch is bounded by arguments.

Behavior-complement model (id nonbehavior): domain is one CLI invocation with
fixed inputs and a writable filesystem. Observation is exit/output; operator
excludes successful replacement of an already existing report via write_new.
Approximation is a partial inner-bound hypothesis about absent behavior.
Assumptions: operating-system create_new semantics. Evidence: existing
write_new implementation and focused CLI refusal check. Other exclusions unknown.

Reverse-reconstruction model (id antecedents): domain is a generated report;
operator returns compatible command invocations with the same registry hash
and report inputs. This is a partial relational specification: output does not
uniquely identify invocation spelling, output filename or earlier filesystem.
Observation horizon is one invocation. Evidence: argument dispatch review and
inventory tests, not an implemented inverse runner.

Combiner: compose CLI dispatch, library report generation and create_new output
as sequential partial functions, preserving error boundaries. Correct leaves do
not prove the composition. A partial source-bound AST projection is stored in syntax.json.

#### Wave35 composition command

compose-registry BASE_REGISTRY OVERLAY NEW_JSON reads and decodes both registry values, calls the validated composition function, serializes the result and creates NEW_JSON exclusively. Validation errors occur before output creation; an existing output remains unchanged. The four-token command includes its verb; other arities reject. Read/parse/allocation/write failures propagate. Exclusive creation does not promise rollback after an I/O failure during writing. No input file is rewritten, no automatic overlay discovery occurs, and no teacher or declared check command executes.

Reverse reconstruction preserves all readable input pairs yielding the same ordered registry union, including different valid splits; output alone does not recover unique original files. The complement excludes successful writes of invalid unions and replacement of pre-existing outputs. Cost adds two input reads, JSON decoding, two tree validations and one output serialization/write; no throughput gain is asserted. See model.md#wave35-evidence for native and actual-process scope.

## composition

### File-local composition

Each child is a scoped view of the same authoritative source and reuses its parent syntax artifact. Byte intervals need not cover tests or all helpers; this is not complete semantic coverage. Old descriptant epochs remain preserved.

- dispatch-output: CLI command arity, library dispatch, exclusive report writing and exit order.

Argument dispatch chooses exactly one tagged API path. Library success feeds serialization and exclusive writer. Report-gate branches then print a summary and enforce report validity. Syntax/extract-table paths have no summary gate. Library tests alone do not prove CLI composition; scoped process probes exercise the actual compiled CLI.

The external-library child models the bounded read-only helper layer. The dispatch child passes its selected repository/store/manifest to that helper, then uses exclusive write_new only after success. Hash/path failure does not reach output writing. Missing paper models remain explicit. Refer to model.md#external-library and external-library.groupant/model.md; no private peta dependency or collection-wide scan is introduced.

## description-profiles

### Source-survey description measure partition

The public library reader admits explicit `words_4000_5000_v1` editions only when the actual UTF-8 description has 4,000–5,000 Unicode-whitespace tokens containing a Unicode alphanumeric scalar. Its `description_measure` must declare unit words, matching count and bounds, and exact versioned tokenizer identifier. Actual `description_words` and `description_unicode_scalars` are checked independently. Unknown profiles and mismatched units/counts are rejected.

Historical absent-profile editions and explicit `chars_4000_5000_v1` editions retain 4,000–5,000 Unicode scalar admission. If an old edition declares an additional measure, that measure must agree with the legacy scalar interpretation; a reader never guesses that a long description means a word profile. A new generator default does not change this reader rule.

Type: error_partition. Domain: artifact-hash-valid survey manifests and finite valid UTF-8 description bytes. Observation: unit-specific local shape admission versus source provenance and scientific truth. Rejecting a word/scalar mismatch is a shape failure; a stale repository binding is a distinct source failure. For n>=4000 counted words, the elementary lower bound C>=2n-1>5000 excludes the historical character gate. Finite controls use 4,300 Cyrillic words, retain the old 4,500-scalar positive case, and turn unit/count/unknown-profile switches red. This is a mathematical argument plus executable examples, not a Lean proof.

Type: reverse_reconstruction. Observation: successful integrity receipt. Its antecedent fiber still contains scientifically sound and unsound texts with the same length and declared provenance; shape cannot identify a unique original paper or prove semantic source agreement. Paper scope remains zero repository bindings/original full text not checked. Algorithm scope remains one actual current repository path/hash binding. Source-kind, missing AST/proof, component confinement and artifact hash gates are unchanged.

Evidence: 22 library-module and 8 CLI tests passed in debug and release, including the new profile/metric refusal test and historical-reader controls. These checks ran locally with fixtures, no teacher, corpus, GPU or actual collection publication. Partial syntax artifacts retain macro/name-resolution limitations.

## external-library

### External library composition

The current CLI adds `library REPO STORE|- RELATIVE_MANIFEST NEW_JSON`. It performs bounded read-only checks before exclusive output creation; unlike groupants/changed gates, invalid library inputs fail before producing a report. The external-library child model defines namespace, missing-formalization, hash and budget assumptions. The dispatch child routes five positional strings including the verb; it neither executes declared commands nor imports the collection into Git.

The new reader belongs to public Mova. General algorithm/paper material lives outside Git through MOVA_GROUPANTS. The existing code-specific development sidecars and registry stay in Mova. No peta module is required. Metadata integrity and finite tests remain distinct from mathematical proof.

## forward

### Current forward model

Subject: adjacent source at manifest SHA-256. Fixed immutable snapshots and adequate resources are assumed; errors/races and allocation limits remain explicit.

#### dispatch-output

Dispatch exact command/arity to syntax, groupants, changed, inventory or extract-table. Inventory optional argument must be --require-all. Compute report, pretty serialize and create_new output. Groupants/changed/required inventory print summary and reject failed gate after writing report.

#### Wave35 composition command

compose-registry BASE_REGISTRY OVERLAY NEW_JSON reads and decodes both registry values, calls the validated composition function, serializes the result and creates NEW_JSON exclusively. Validation errors occur before output creation; an existing output remains unchanged. The four-token command includes its verb; other arities reject. Read/parse/allocation/write failures propagate. Exclusive creation does not promise rollback after an I/O failure during writing. No input file is rewritten, no automatic overlay discovery occurs, and no teacher or declared check command executes.

Reverse reconstruction preserves all readable input pairs yielding the same ordered registry union, including different valid splits; output alone does not recover unique original files. The complement excludes successful writes of invalid unions and replacement of pre-existing outputs. Cost adds two input reads, JSON decoding, two tree validations and one output serialization/write; no throughput gain is asserted. See model.md#wave35-evidence for native and actual-process scope.

#### External reference library

`undate REPO [--dry-run] NEW_REPORT` has three or four positional strings including the verb; the optional flag must be `--dry-run`. It refuses an existing report path before any work, calls `undate::run`, writes the report exclusively and prints a one-line summary. Without the flag the model.md and JSON edits are written before the report; every per-file verification and idempotence failure aborts before any write.

`sidecar-links REPO NEW_JSON` has three positional strings including the verb. It calls `links::check`, writes the report exclusively, prints a one-line summary and then fails when any dangling sidecar reference remains (report before gate error). `relink-evidence REPO [--dry-run] NEW_REPORT` has three or four positional strings; the optional flag must be `--dry-run`. It refuses an existing report path before any work, calls `links::relink`, which writes the rewritten Markdown and rebound JSON files unless the flag is given, then writes the report exclusively and prints a one-line summary; unresolved references are listed, not an error.

`library REPO STORE|- RELATIVE_MANIFEST NEW_JSON` has five positional strings including the verb. Validate one manifest and its fixed-snapshot hash/path bindings before an exclusive report write. Invalid library inputs fail before report creation. The external-library child specifies limits, missing states and non-execution. Existing report-before-gate-failure branches are unchanged.

## invariant-and-cost

### Current invariants and cost

These are source-derived partial constraints. Conditional costs count repeated IO and do not certify runtime bounds or end-to-end gains.

#### dispatch-output

No declared tests execute. Existing output is preserved by create_new. Invalid gate may leave a complete report plus nonzero exit; preflight/library/serialization failure occurs before output creation. write/sync errors may leave partial artifact.

Argument collection O(total argument bytes); dispatch comparisons constant command count. Library work plus O(report bytes) serialization/allocation and write/sync. Process launch costs not modeled as parsing improvement.

#### Wave35 composition command

compose-registry BASE_REGISTRY OVERLAY NEW_JSON reads and decodes both registry values, calls the validated composition function, serializes the result and creates NEW_JSON exclusively. Validation errors occur before output creation; an existing output remains unchanged. The four-token command includes its verb; other arities reject. Read/parse/allocation/write failures propagate. Exclusive creation does not promise rollback after an I/O failure during writing. No input file is rewritten, no automatic overlay discovery occurs, and no teacher or declared check command executes.

Reverse reconstruction preserves all readable input pairs yielding the same ordered registry union, including different valid splits; output alone does not recover unique original files. The complement excludes successful writes of invalid unions and replacement of pre-existing outputs. Cost adds two input reads, JSON decoding, two tree validations and one output serialization/write; no throughput gain is asserted. See model.md#wave35-evidence for native and actual-process scope.

## publication-boundary

### Publication command composition

The source-bound dispatcher exposes publication-catalog STORE REVIEWS|- NEW_CATALOG and publication-check REPO STORE CATALOG REL_MANIFEST [REVIEWS EXPECTED_REVIEWS_SHA] NEW_REPORT with exact argument counts. The call without reviews/digest is negative-only compatibility; positive admission requires both. Existing library remains integrity-only.

Catalog creation resolves a disjoint external store and serializes a bounded additive inventory through exclusive write_new. Bundle-subdirectory destinations are refused. A global policy-decision count requires current reader admission and is not publication permission by itself. Positive checking recomposes the current bounded catalog from caller-pinned reviews, checks inventory and normalized decision equality, applies artifact-disjoint evidence and narrative-only permission, then calls the existing independent integrity reader. Reports remain outside the store; original bundles are unchanged.

Domain: fixed readable local filesystem snapshots and completed calls. Observation: Result plus optional new output. Behavior complement: wrong arity selects usage failure and an existing output cannot be overwritten. Reverse reconstruction: a report constrains exact policy/inventory/artifact versions but cannot recover a unique reviewer, input spelling or directory. Detailed typed models reside at ../publication.rs.groupant/model.md#model and ../publication.rs.groupant/model.md#pinned-policy. Editorial approval, generated/unverified mathematical assurance, authorship and scientific proof remain distinct.

## wave33-catalog

# Wave33 additional typed models

Subject/revision: adjacent source at current manifest hash. Preserve the original baseline families and historical catalogs. These additions are source-derived partial constraints on one call at a fixed snapshot, with adequate resources. Full mathematical correspondence, races and automatic inverse execution remain gaps.

## dispatch_arity_partition

Known verb/arity pairs choose exactly one branch; inventory's optional fifth string must equal --require-all. Unknown command/arity maps to usage error before report creation.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## report_before_gate_error

groupants writes full report before rejecting valid=false; changed and require-all inventory similarly preserve their reviewable report after failed coverage gate.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## exclusive_write_failure_fiber

An existing output file prevents replacement. Open success followed by write/sync error may leave partial output, so nonzero exit does not imply no artifact.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## summary_information_loss

Summary JSON projects only selected fields of the full report and never infers verification. Many reports and invocation destinations share the same summary.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## wave33-evidence

### Wave33 native evidence

All 17 exact library tests passed twice in debug and twice in release (68 exact invocations), each executing exactly one test. Sorted target-qualified outcomes match across all four runs. Full test bodies were reviewed before execution. Eleven actual CLI process cases additionally pass twice per profile: projection success, existing-output refusal with unchanged bytes, ignored source, invalid Rust, invalid report persisted before rejection, require-all coverage failure, option-like revision rejection, unknown option, wrong arity, complete table and proof-error rejection. CLI cases use temporary synthetic repositories/files, not a live corpus.

Three compiled production-behavior controls reject through named assertions: inverted sorted membership, zero BOM base, and admission of retired 15-percent tier. Each source was restored byte-for-byte from backup, SHA-256 matched its original, and its exact test passed afterward. There are zero assertion-only controls, zero surviving controls and zero claimed old-code bug witnesses: the production change is an exact search optimization, not a correctness repair.

#### Membership measurement

The new shared helper is called by groupants registered-source absence checks and syntax admission. rust_files already returns BTreeSet-sorted paths. For sorted F, the old existential scan and binary search have identical Boolean membership. Fixed empty/single/Unicode/prefix/boundary cases, every discovered repository path plus misses, and 4096 synthetic paths plus misses compare their complete Boolean vectors against the old expression. Both measured implementations return identical hit counts; black_box prevents removing queries. Five paired samples alternate order, each three repetitions, in each of two release executions. Actual repository paths: 975, queries: 1099, hits per measured batch: 2925. Synthetic paths: 4096, queries: 4610, hits: 12288.

| Fixture | Run | Linear median ns | Binary median ns | Linear/binary |
|---|---:|---:|---:|---:|
| repository_paths | 1 | 822745 | 295542 | 2.784 |
| synthetic_4096 | 1 | 55611991 | 1039777 | 53.485 |
| repository_paths | 2 | 820819 | 299779 | 2.738 |
| synthetic_4096 | 2 | 56177933 | 1038791 | 54.080 |

These are membership-kernel observations on this CPU, not whole-inventory, training, GPU or corpus speedups. The synthetic common-prefix workload particularly penalizes linear scan. Git discovery, hashing, repeated artifact/catalog reads and registry subtree walks remain untouched and may dominate commands. No executable percentage approximant or approximation-error claim is supplied.

#### Scope and binding

All three file manifests and eleven scoped children bind the final source hashes. Two fresh syntax projections per source match byte-for-byte; children reuse parent JSON. Models retain prior epochs, the original twenty syntax models and both original library/CLI baselines. Current typed totals: library tree 48, syntax tree 42, CLI tree 12, overall 102. Syntax is partial, macros opaque; integrity checks do not reparse artifacts. Source ranges are snapshot anchors, not proof that models cover every path.

Raw logs, numeric exits, source hashes, mutation backups and paired timing records live in the external wave33 modelcheck run receipts. Native metadata and source/test mappings are supplied for root integration. Ancestor manifests and registry are owned by the integration worker; local stale ancestor bindings are expected until integration refreshes them. No installed executable was replaced, and no teacher/network/model/corpus operation ran.

#### Complete report reference differential

A temporary actual CLI build replaced only the new membership helper body with the old linear existential predicate. Source bytes were restored exactly before either CLI inspected the repository. Old/new complete groupants reports match byte-for-byte (10,660,823 bytes), including diagnostics and order. All three owned file sidecars and eleven children validate with no local diagnostics. The only four global errors are stale integration-owned ancestor source bindings; root refreshes those after merging. This is same-output evidence, not a whole-command timing measurement.

## wave35-evidence

### Wave35 registry composition evidence

All 22 native modelcheck tests pass twice in debug and twice in release. Sorted named outcomes are identical across all four runs. Three new synthetic tests exercise group directory selection/confinement, composition preconditions and union-tree rejection, and strict unregistered-sidecar inspection followed by composed ownership. Existing tests retain their original finite scopes.

Seven actual CLI cases pass twice per profile (28 process invocations): composition, deterministic recomposition, existing-output refusal without byte changes, invalid union rejection before output creation, invalid overlay schema rejection, wrong arity rejection, and groupants discovery of the selected group directory. Synthetic fixture inputs remain unchanged.

Four compiled production mutants fail named assertions: force default group directory, bypass override path/type guards, omit composed-tree validation, and omit base-tree validation. Exact original bytes are restored and checked before the final positive runs. These are feature controls, not evidence of a previously claimed runtime bug. No assertion-only mutant is counted.

Package all-target Clippy completes with existing warnings; strict -D warnings fails on prior library lints. The initial escape fixture used an incorrect diagnostic substring; correcting that assertion required no production change. Source projections repeat byte-identically, all owned manifest hashes and byte scopes are refreshed, and ancestor bindings are left to integration. Raw logs and controls are under the external wave35 registry run directory. Structural declarations and finite synthetic checks do not establish semantic proof, all-source coverage, whole-system performance or an approximation bound.

Final structural inspection reports no diagnostics in either owned file tree. Its only four failures are stale integration-owned source hashes in model-infrastructure and mova ancestor manifests, for the two changed sources. The changed-source gate passes with exactly lib.rs and main.rs registered; focused rustfmt and diff-whitespace checks pass. Integration must refresh those ancestor bindings before claiming a globally valid tree.

## History

### 2026-10-29 · descriptant

Subject: adjacent main.rs at the manifest source hash. Epoch: 2026-10-29.
Forward approximant: dispatch an argument vector to changed, inventory,
groupants or extract-table. Serialize a successful report to a new output.
For groupants, write the report before rejecting valid=false. Registry parse
and structural errors can fail before output. Declared checks never execute.

Epoch memory: on 2026-10-29 groupants joins existing commands; invalid sidecars
retain a reviewable report while absent sidecars remain explicit backlog.
No general model generation or semantic verification is claimed.

Group epoch, 2026-10-29: the groupants stdout summary now includes group_sidecars
and missing_group_sidecars from the library report. Invalid existing group
artifacts retain a report and cause failure; absent groups remain backlog.
The projection adds two counters and keeps create_new and failure boundaries.

Syntax epoch, 2026-10-29: syntax REPO SOURCE NEW_JSON composes path admission,
UTF-8 parsing, source-bound partial AST projection and exclusive output. The
baseline complement excludes replacement of an existing artifact and output
creation following parse failure. Reverse reconstruction preserves ambiguity
in output location and unrelated repository state while constraining source
identity and selected structure. Source parsing adds library syntax cost plus
serialization. No declared check, teacher or training command executes. See the
syntax file groupant for twenty scoped models and explicit projection gaps.

### 2026-10-31 · descriptant

#### Wave33 current source review — 2026-10-31

Preserved all earlier descriptions and typed models; current model.md#forward, model.md#invariant-and-cost and model.md#composition refine scope, effect order and repeated IO costs. The current syntax.json binds exact source bytes; scoped children share it and retain nonempty byte intervals. Current tests and mutation limits are in model.md#wave33-evidence. The membership optimization replaces only existential scans over the already sorted Git path vector; its reference equivalence and paired kernel timings do not claim faster whole inventories. Executable percentage approximants remain unsupported for these categorical validation decisions: no useful loss metric/domain has been established. Future caps remain 1, 2, 5 and 10 percent.

### 2026-10-31 · wave33-evidence

Heading `### Wave33 native evidence — 2026-10-31` stays in wave33-evidence as `### Wave33 native evidence`.

### 2026-10-31 · wave35-evidence

Heading `### Wave35 registry composition evidence — 2026-10-31` stays in wave35-evidence as `### Wave35 registry composition evidence`.

### 2026-11-01 · descriptant

2026-11-01: general algorithm/paper groupants now live outside Git. The public CLI reads one external manifest through explicit MOVA_GROUPANTS configuration and verifies bounded source/asset integrity. A dedicated external-library child models the helper layer; dispatch preserves fail-before-report for invalid library input. This is not semantic verification, paper download, training or automatic publication. Earlier scoped evidence remains historical.

### 2026-11-01 · external-library

#### Generated source surveys, 2026-11-01

`source_survey_groupant` is a separate namespace from existing reviewed algorithm bundles and source-bound paper-use nodes. Its integrity predicate is A ∧ C ∧ U ∧ M ∧ S: bounded artifact hashes A; declared component paths C; actual UTF-8 description count U in [4000,5000] under its explicitly versioned word/scalar metric, with absent-profile editions retaining historical scalar units; generated/unverified status plus missing AST and proof M; admitted source kind/format and provenance scope S. Algorithm surveys require one current repository path/hash binding. Paper surveys have zero repository bindings: their declared external-source digest is syntactically checked, but the original paper text, licence and scientific statements are not independently checked by this reader. It must not invent such a binding.

This admits a substantive or an empty-content 4500-scalar draft equally if both satisfy shape constraints; therefore acceptance never establishes scientific quality, originality, completeness, semantic source derivation or mathematical proof. Existing explicit paper/algorithm bundle contracts are unchanged. No declared command executes. No source digest is refreshed.

Typed complement: within this finite manifest observation, a stale algorithm SHA, malformed kind/format pair, nonmissing AST/proof, undeclared component path, invalid scalar count or changed artifact yields failure. The converse reconstruction is a fibre: every admitted receipt with source_bindings=0 can correspond to many paper texts and claims sharing the declared metadata. Unique paper recovery or scientific correctness does not follow. A source-bound algorithm receipt narrows to repository byte content at that path/hash under SHA-256 collision-resistance and a fixed filesystem snapshot; it still leaves generated-model truth undetermined.

Finite evidence: seven CLI fixture tests pass in debug and release, including two new tests exercising deterministic repeated receipts; paper0/algorithm1 provenance scopes; non-ASCII scalar counting; stale source; 3999-scalar text; count mismatch; kind/format exclusion; promoted proof and tampered-artifact controls. These are bounded executable observations, not a proof of total reader correctness.

#### Versioned word standard, 2026-11-01

[Description profile partition](model.md#description-profiles) defines the new standard of4,000–5,000 words and independent Unicode/word measurements. Legacy scalar-profile editions remain readable without reinterpretation. Eight current CLI fixture tests and22library tests pass in both profiles; no actual collection write or scholarly review is implied by these tests.

### 2026-11-01 · publication-boundary

Finite evidence on 2026-11-01: 18 publication controls and all 26 binary tests pass debug/release. Historical dispatch/library receipts remain historical; refreshing source bindings and byte ranges does not rerun or reinterpret their earlier measurements. The old no-policy form still consumes the current zero-global negative catalog.

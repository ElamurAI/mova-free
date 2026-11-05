# modelcheck-library

## descriptant

### Modelcheck library groupant

Forward approximant: for repository snapshot S and registry R, discovery gives
sorted Rust paths P. For each p, read source bytes and an optional adjacent JSON
manifest, compute SHA256(source), and collect structural diagnostics D(p).
valid equals the conjunction of empty diagnostics, including missing registered
sources. A missing sidecar produces a backlog row. Registry/tree or Git errors
return an error before report construction. This is a partial model: filesystem
races, allocation failures and platform behavior remain outside the snapshot.

Invariant: all inspected referenced files canonically lie inside the repository;
source identity equals the adjacent Rust file; baseline type accounting is
ordered behavior_complement then reverse_reconstruction. Unknown JSON fields
survive reporting. Neither checks nor model artifacts execute.

Cost approximation: O(B + P log P + N log N + A) time and O(B + P + N + A)
space for source/manifest bytes B, paths P, registry nodes N, references A,
plus O(T log N + H log P) time for traversed descendant nodes T and group source
bindings H, and O(N + P + H) retained group/report data. A deeply nested tree
can repeat descendant traversal quadratically; source bytes are hashed once.
Add Git and filesystem costs. Repeated small file reads and canonicalization
may dominate wall time. No performance improvement is claimed.

Behavior-complement approximant (id nonbehavior): domain is successful inventory
calls over fixed filesystem snapshots and valid registries. Observation is the
report/return outcome, horizon one call. Operator excludes accepting a declared
stale source hash, missing referenced file or missing required type record with
valid=true. This is a partial inner-bound hypothesis about excluded outcomes,
supported by focused tests; unsampled faults and races remain unknown.
Negative controls are evidence for this model, not the entire complement.

Reverse-reconstruction approximant (id antecedents): domain is returned reports.
Observation is file hashes, original manifests, ownership and diagnostics.
Operator maps a report to compatible snapshots with the same enumerated bytes'
hashes and declarations. It is a partial relational specification: source bytes,
untracked ignored files, timing and filesystem state cannot be uniquely recovered.
SHA-256 identity is an operational comparison, not an injectivity proof.
Evidence is preservation/determinism tests; converse execution is not implemented.

Combiner: G=(source identity, AST gap, forward projection, typed family,
invariant, this descriptant, checks). Compose child file rows by conjunction of
structural validity and union of diagnostics. This does not establish semantic
correctness of mathematical artifacts or parent wiring.

Epoch memory: initial groupant inventory distinguishes absence from an invalid
existing declaration. Full AST, mathematical semantic validation and generation
remain gaps. References: ../lib.rs unit tests and ../../README.md.

The nonbehavior approximant additionally excludes valid acceptance of missing
transitive bindings, duplicate/wrong children and stale descendant hashes. The
reverse relation constrains compatible tree topology, descendant source hashes
and open group declarations, without claiming unique source reconstruction or
semantic correctness. Evidence: focused regression includes 38 typed models,
14 malformed declaration cases, source mutation, duplicate file/group IDs,
malformed JSON, unsafe IDs and symlink escapes; unchanged inputs repeat exactly.
The expected added cost is descendant traversal and JSON/reference validation;
no runtime improvement or model semantic verification is claimed.


The behavior complement now excludes valid acceptance of zero/multiple matching
catalog lines, nonstring/empty selectors, missing paths and non-UTF8 selected
artifacts. Reverse reconstruction additionally constrains compatible artifact
texts to exactly one matching line outside recognized fences; their other text
and mathematical meanings remain nonunique. The combiner applies this shared
predicate to all component records before combining file/group diagnostics.
Expected cost adds O(C) time and O(M + L) transient space for repeatedly read
selected artifact bytes C, maximum selected artifact size M and selector length
L; repeated references are intentionally not cached. No speedup is claimed.
Evidence will be focused temporary-repository positive and negative controls,
including identical repeated reports, on both file and group inventory paths.

#### File-local hierarchy model

    Dtree(v) = D(v) union Union_c (E(v,c) union Dtree(c)).
    accepted(v) iff Dtree(v) is empty.

The report retains fragment identities, parents, scopes and manifests.
Traversal does not execute the models or the named tests. A parent with children
must declare a composition component. A declaration alone cannot establish the
semantic composition relation.

The existing behavior-complement inner bound extends to successful acceptance
of stale fragment hashes, a child of another Rust file, multiply used IDs,
unsafe child paths, missing required models or out-of-scope ranges. Its universe
is one inventory call on a finite stable repository snapshot. These excluded
outcomes are claims about structural acceptance, not all Rust behavior.

The reverse relation of a valid report contains all snapshots matching the
visible source hashes, tree links, declared ranges and artifact checks.
Neither a unique source program nor a unique semantic decomposition follows.
Two different overlapping model scopes may describe the same source fragment.

For byte ranges [a,b) in a source with L UTF-8 bytes, require
0 <= a < b <= L and both endpoints at character boundaries. A range-bearing
ancestor [p,q) constrains every descendant range by p <= a < b <= q.
Sibling ranges may overlap or leave holes. The aggregate coverage is not
asserted to be complete, disjoint, or a partition of execution.

The whole-file SHA binding is deliberately conservative: changing even bytes
outside a fragment invalidates its old source binding. Refreshing a hash without
reviewing the range and model is not new semantic evidence.

Expected added cost is linear in reached fragment/edge counts, plus artifact
reads, canonicalization, source identity checks and ordered uniqueness tracking.
Actual repeated source reads and nested path lengths can make byte cost exceed
one source read per tree. Keep limits and measured wall time separate from
this structural approximation.

#### Syntax-bound fragment envelopes

The behavior-complement inner bound adds accepted outcomes with stale/foreign
projection identity, absent/duplicate/noninteger IDs, or a mismatching envelope.
The reverse relation maps an admitted interval to every selection with those
extreme endpoints. Multiple selections can share an envelope: it cannot recover
a unique function decomposition or show that all interior code is modeled.

Node selection is a structural binding. Partial projections omit semantics and
macro expansion; checking their metadata and shape does not certify their
correspondence to Rust. The validator does not reparse them. The expected
additional cost is artifact reading/validation plus selection lookup and extrema;
repeated references may repeat artifact work. No speedup is predicted.

#### Executable Rust approximant attachment structure

The forward predicate accepts budgets in {1,2,5,10} percent only and declared
finite errors e satisfying 0 <= e <= budget <= 10. Exact error zero is valid even
though zero is not a budget profile. Unknown extra record fields remain open.
All attachments pass their local predicate before the enclosing groupant can
be structurally valid. The mathematical approximant remains separately present.

Behavior complement: within a fixed filesystem snapshot, accepted declarations
exclude stale attachment/source/evidence hashes, missing evidence files, escaped
Rust paths, duplicate IDs, other budget caps, negative or over-budget errors,
and nonnumeric/nonfinite error encodings. Reverse reconstruction retains all
source/artifact/evidence byte snapshots compatible with the hashes and declared
domain/metric/budget. Neither reconstructs unique behavior from hashes or claims.

Cost adds O(A log A + B) time for A records and B repeatedly read artifact,
source and evidence bytes, with O(A + M) temporary space for IDs and the maximum
read file M. Multiple profiles intentionally repeat reads. The combiner adds
this predicate to the existing source/component validation, including fragments.
Structural acceptance does not compile or run Rust, establish analytical bounds,
certify empirical coverage or provide a speed gate. Tests exercise all four
profiles and zero error, legacy absence, stale hashes, evidence failures, escape
paths, duplicate IDs and bad budgets/errors. Real runtime evidence belongs to
each attachment; this checker verifies only the references and declarations.

#### Wave34 exact-byte decode reuse

Repeated shared projection decoding was investigated after wave33 membership timing failed to move whole-command time. A source-tree-local one-entry memo now reuses parsed JSON only after a fresh byte read and full equality. Source hashing and every artifact guard still run on each reference. Earlier epochs above are historical; current bindings and ranges are in manifests. See model.md#wave34-evidence for finite tests, controls and measurements.

#### Wave35 explicit registry composition

A base registry B must independently satisfy the existing tree validator. Overlay O may name B's root among its children. Composition C has schema 1, root O.root and nodes B.nodes concatenated with O.nodes in input order; O.schema must equal 1 and C must satisfy the same validator. No IDs or sources are silently replaced. Validation rejects duplicate ownership, unreachable nodes, unknown children and cycles. Cloning leaves both inputs intact. Its reverse relation consists of compatible ordered prefix/suffix decompositions whose first part independently validates and whose second part names C.root; output alone does not identify a unique split. The behavior complement excludes accepted unions violating these conditions, relative to schema-1 declarative registries. Cost is two tree validations plus cloning owned text: O(N log N + E) with storage O(N + E + text bytes).

Optional groupant_path applies only to group nodes and selects a normal repository-relative directory ending .groupant; omission retains models/<id>.groupant. Existing canonical file confinement checks manifest/component resolution, including symlink escape. Source nodes remain adjacent. Directory selection changes lookup, not registry ownership, source binding or mathematical validity. Missing directories retain existing explicit backlog semantics. Canonical aliases and concurrent filesystem changes are not a transaction guarantee.

A valid sidecar without the optional registry.node pointer can be inspected as unregistered evidence by a smaller registry. Composing ownership later fills its report model; IDs, source hashes, syntax paths and other manifest checks remain enforced. The strict discovery default is unchanged. No scoped-discovery flag is introduced.

## composition

### File-local composition

Each child is a scoped view of the same authoritative source and reuses its parent syntax artifact. Byte intervals need not cover tests or all helpers; this is not complete semantic coverage. Old descriptant epochs remain preserved.

- registry-tree: Registry schema, node clauses, path ownership, parent uniqueness and root reachability.
- discovery-changes: Sorted Git discovery, binary membership, inventory and changed-source gate.
- output-table: Exclusive output creation and finite Lean table extraction.
- inventory-assembly: File/groupant report assembly and registered-source absence diagnostics.
- fragment-validation: Recursive child directory admission, range inheritance and syntax-envelope validation.
- group-source-bindings: Group transitive binding, confinement, required-type projection and source manifest identity.
- component-validation: Component references, exact H2 selectors and executable-approximant declaration gates.

Registry validation is fail-fast preflight for inventory/groupants. Sorted discovery feeds ownership membership and per-file hash identity. File report assembly calls fragment/source/component validators; their diagnostics propagate to file status and global valid. Group binding traverses the registry after file hashes are collected. Output/table functions are independent tagged APIs, not runtime stages of every inventory. CLI owns their dispatch. Component and fragment scopes overlap through calls, so their costs must not be added twice.

#### Wave35 explicit registry composition

A base registry B must independently satisfy the existing tree validator. Overlay O may name B's root among its children. Composition C has schema 1, root O.root and nodes B.nodes concatenated with O.nodes in input order; O.schema must equal 1 and C must satisfy the same validator. No IDs or sources are silently replaced. Validation rejects duplicate ownership, unreachable nodes, unknown children and cycles. Cloning leaves both inputs intact. Its reverse relation consists of compatible ordered prefix/suffix decompositions whose first part independently validates and whose second part names C.root; output alone does not identify a unique split. The behavior complement excludes accepted unions violating these conditions, relative to schema-1 declarative registries. Cost is two tree validations plus cloning owned text: O(N log N + E) with storage O(N + E + text bytes).

Optional groupant_path applies only to group nodes and selects a normal repository-relative directory ending .groupant; omission retains models/<id>.groupant. Existing canonical file confinement checks manifest/component resolution, including symlink escape. Source nodes remain adjacent. Directory selection changes lookup, not registry ownership, source binding or mathematical validity. Missing directories retain existing explicit backlog semantics. Canonical aliases and concurrent filesystem changes are not a transaction guarantee.

A valid sidecar without the optional registry.node pointer can be inspected as unregistered evidence by a smaller registry. Composing ownership later fills its report model; IDs, source hashes, syntax paths and other manifest checks remain enforced. The strict discovery default is unchanged. No scoped-discovery flag is introduced.

## forward

### Current forward model

Subject: adjacent source at manifest SHA-256. Fixed immutable snapshots and adequate resources are assumed; errors/races and allocation limits remain explicit.

#### registry-tree

Build ID and source sets; reject malformed nodes before parent analysis. Insert each child edge only once, reject a parented root, then DFS from the root and require reached count = node count. Registry deserialization separately denies unknown fields.

#### discovery-changes

Git ls-files -z cached/others exclude-standard feeds a BTreeSet. Membership returns binary_search success on that sorted list. Inventory verifies registered paths, confines and hashes files, and accumulates ancestor directory counts. Changed inventory rejects option-like base before inventory, then unions surviving diff/untracked .rs paths.

#### output-table

write_new opens create_new, writes all bytes, syncs and propagates the first error. extract_table rejects proof-error markers anywhere, locates the first header, admits exactly five tab fields per nonempty trailing row, unique four-tuples in {0,1,2,3}^4 and a Boolean label, then requires 256 tuples.

#### inventory-assembly

Validate registry then filename-safe IDs, discover files, hash confined sources, load optional sidecars, validate source/group fragments and retain original manifests. Append diagnostics in discovery/declaration order. Process logical groups in sorted ID order and return valid iff diagnostics is empty.

#### fragment-validation

For each declared child reserve a preorder report slot, require one real immediate .groupant directory and regular manifest, validate same source and kind, check ID, range and optional syntax envelope, recurse and propagate errors. Reject nonempty child traversal at depth>=64.

#### group-source-bindings

Group manifests match immediate registry child sets and the exact transitive source set with current hashes; source manifests identify the canonical adjacent source and expected digest. Both validate components. Source AST opt-in additionally reads and validates a projection; relative artifact paths may use .. only while canonical target remains inside root.

#### component-validation

Require core component records and both baseline family types; validate IDs/status/references and optional exact unique H2 sections outside narrow fences. Optional Rust profiles check bindings, confinement, classification compatibility, evidence, budget in {1,2,5,10} and finite nonnegative declared error<=budget.

#### Wave35 explicit registry composition

A base registry B must independently satisfy the existing tree validator. Overlay O may name B's root among its children. Composition C has schema 1, root O.root and nodes B.nodes concatenated with O.nodes in input order; O.schema must equal 1 and C must satisfy the same validator. No IDs or sources are silently replaced. Validation rejects duplicate ownership, unreachable nodes, unknown children and cycles. Cloning leaves both inputs intact. Its reverse relation consists of compatible ordered prefix/suffix decompositions whose first part independently validates and whose second part names C.root; output alone does not identify a unique split. The behavior complement excludes accepted unions violating these conditions, relative to schema-1 declarative registries. Cost is two tree validations plus cloning owned text: O(N log N + E) with storage O(N + E + text bytes).

Optional groupant_path applies only to group nodes and selects a normal repository-relative directory ending .groupant; omission retains models/<id>.groupant. Existing canonical file confinement checks manifest/component resolution, including symlink escape. Source nodes remain adjacent. Directory selection changes lookup, not registry ownership, source binding or mathematical validity. Missing directories retain existing explicit backlog semantics. Canonical aliases and concurrent filesystem changes are not a transaction guarantee.

A valid sidecar without the optional registry.node pointer can be inspected as unregistered evidence by a smaller registry. Composing ownership later fills its report model; IDs, source hashes, syntax paths and other manifest checks remain enforced. The strict discovery default is unchanged. No scoped-discovery flag is introduced.

## invariant-and-cost

### Current invariants and cost

These are source-derived partial constraints. Conditional costs count repeated IO and do not certify runtime bounds or end-to-end gains.

#### registry-tree

Every successful registry is one nonempty rooted tree, source nodes are childless, groups have children, and source strings are unique. relative uses platform Path components; unlike syntax admission it does not separately reject repeated slash spellings.

For N nodes, E edges and total clause/path bytes B: O(B+(N+E) log N) comparison operations, O(N+E) auxiliary entries. String comparison length and allocations add costs.

#### discovery-changes

Discovery order is strict Rust String order and duplicate paths collapse. Membership equals the old existential scan only under sorted order. Directory registered<=total; missing=total-registered. Deleted diff paths are omitted by is_file; inventory may already fail on tracked missing files.

Discovery sorting O(P log P) comparisons; membership Q queries O(Q log P) rather than O(QP), multiplied by compared prefix lengths. Inventory hashes total bytes B and visits directory depth D per path. Changed invokes inventory plus two Git processes; no IO speed gain follows.

#### output-table

Successful extraction has 257 lines and exactly the full four-dimensional domain. Row order remains input order and labels are not mathematically verified. Existing output cannot be replaced; failures after open may leave partial or complete unsynced bytes.

For L log bytes and K rows, O(L+K log K) time plus parsing allocations, O(L+K) retained output/tuples. K=256 on success, malformed logs may be longer. Writer O(bytes) plus filesystem write/sync latency.

#### inventory-assembly

Missing sidecars are backlog rather than invalid declarations; existing malformed sidecars invalidate the report. File and fragment/group IDs share one uniqueness set. Registry membership is checked against sorted discovery with the shared helper. verification_inferred and full_system_verification are false.

O(B + Q log P) membership/hash work plus manifest/component/fragment validation and group subtree traversal. Source/artifact rereads are counted separately, not as once per source. Report retains JSON; filesystem mutation races are excluded from deterministic snapshot claims.

#### fragment-validation

IDs are globally unique; descendants under a range declare contained nonempty UTF-8 intervals. Siblings may overlap or leave holes. Syntax node selection is distinct/nonempty and its extrema equal declared range. Failed child loading remains a diagnostic report slot.

O(F log F + R + A) logical work for F fragments, selected-node counts R and repeated artifact validation A; path canonicalization and repeated JSON/source reads dominate bytes. Stack depth<=64 for declared recursion; report retains F manifests and nested diagnostics.

#### group-source-bindings

Each expected source has one binding; no extra binding is admitted. Group IDs used as filenames contain only ASCII alphanumeric ._- excluding . and ... Baseline required-family rows are emitted in fixed complement/reverse order, without proving model truth.

For groups g, sum subtree sizes T_g can be quadratic in registry depth. Binding checks use ordered sets/maps O((T+H) log N), plus repeated component/AST IO and validation. Confinement performs canonicalization for each reference.

#### component-validation

Unknown nested fields remain opaque. Sections require explicit path and one exact plain H2; @document has no special meaning. Profile acceptance validates declarations only, with no compilation, runtime or accuracy certificate.

For C component references and repeated total referenced bytes B, O(B+ID sorting+section scans) plus filesystem calls. Repeated section references reread/rescan the same catalog; source/artifact/evidence files may be hashed per profile. Cache optimization remains unimplemented.

#### Wave34 cost correction

Syntax artifact JSON decoding may now share one exact-byte memo per source tree. Filesystem confinement, fresh reads, source rehash and full node validation remain repeated; Markdown section reads and hashed evidence are unchanged. Old no-cache statements above describe wave33. No cache survives a source-tree invocation.

#### Wave35 explicit registry composition

A base registry B must independently satisfy the existing tree validator. Overlay O may name B's root among its children. Composition C has schema 1, root O.root and nodes B.nodes concatenated with O.nodes in input order; O.schema must equal 1 and C must satisfy the same validator. No IDs or sources are silently replaced. Validation rejects duplicate ownership, unreachable nodes, unknown children and cycles. Cloning leaves both inputs intact. Its reverse relation consists of compatible ordered prefix/suffix decompositions whose first part independently validates and whose second part names C.root; output alone does not identify a unique split. The behavior complement excludes accepted unions violating these conditions, relative to schema-1 declarative registries. Cost is two tree validations plus cloning owned text: O(N log N + E) with storage O(N + E + text bytes).

Optional groupant_path applies only to group nodes and selects a normal repository-relative directory ending .groupant; omission retains models/<id>.groupant. Existing canonical file confinement checks manifest/component resolution, including symlink escape. Source nodes remain adjacent. Directory selection changes lookup, not registry ownership, source binding or mathematical validity. Missing directories retain existing explicit backlog semantics. Canonical aliases and concurrent filesystem changes are not a transaction guarantee.

A valid sidecar without the optional registry.node pointer can be inspected as unregistered evidence by a smaller registry. Composing ownership later fills its report model; IDs, source hashes, syntax paths and other manifest checks remain enforced. The strict discovery default is unchanged. No scoped-discovery flag is introduced.

## wave33-catalog

# Wave33 additional typed models

Subject/revision: adjacent source at current manifest hash. Preserve the original baseline families and historical catalogs. These additions are source-derived partial constraints on one call at a fixed snapshot, with adequate resources. Full mathematical correspondence, races and automatic inverse execution remain gaps.

## sorted_membership_contract

Sorted Git discovery is the precondition for existential membership by binary search. Linear-reference differential checks cover fixed edge cases, 4096 synthetic paths and actual repository path metadata; IO and report order are unchanged.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## diagnostic_effect_order

Registry and unsafe-ID preflight fail before report assembly. Local validation errors accumulate; missing sidecars remain backlog. CLI can persist an invalid report before returning nonzero.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## repeated_io_accounting

Count source rereads for opt-in AST artifacts, per-fragment projection validation, repeated catalog sections and profile source/evidence hashes. Earlier once-per-source shorthand applies only initial inventory hashes, not complete validation.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## typed_gate_separation

Required family presence and declaration status do not execute models or prove complement/converse relations. Approximation classification/budget validation does not establish error or timing claims.

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

## wave34-evidence

### Wave34 finite validation evidence

#### Prospective model and implementation

Repeated nested references decoded one shared whole-source projection every time. A source-tree-owned one-entry memo now reuses parsed JSON only after a fresh read and full byte equality. All caller confinement, source reread/hash and every node check remain. MOVA_MODELCHECK_REFERENCE_JSON=1 preserves uncached decoding. Retained serialized bytes are capped at 8 MiB; parsed Value allocator overhead and transient old/new allocations are additional. No validation result or failed decode is cached. No state survives a source-tree invocation.

#### Differential and measured cost

The exact-base debug executable built before edits produces a byte-identical whole report to the candidate. Three alternating complete-command pairs with reference switch also produce byte-identical reports. Reference wall times: 2.899, 2.875, 2.895 seconds; memo: 1.393, 1.378, 1.399 seconds. First pair CPU: reference user 2.572s/system 0.323s; memo user 1.148s/system 0.235s. Rust Instant and Linux getrusage RUSAGE_CHILDREN measured these actual commands. Reports have only four expected stale ancestor source bindings, owned by integration.

A 163740-byte synthetic projection (160 functions, 1601 nodes) is reread and fully validated twelve times per sample. Release run 1: reference 12.779–12.955 ms versus memo 3.173–3.213 ms, including exact Value comparisons and fresh source validation. Four samples per profile repetition. Gains are workload-specific; root independently repeats integration measurements.

#### Compiled controls and boundaries

Three compiled production mutations reject: bypass byte equality (named differential assertion), bypass fresh source hash (named stale-source assertion), weaken parent<index to <= (runtime vector-bounds panic). All restore exact backup hashes and rerun green. No compile-only kills or survivors. Panic rejection is distinct from assertion rejection.

Observed boundaries include same-length edits, malformed JSON, invalid UTF-8, deletion, directory replacement, oversized retention, stale source with unchanged cached JSON, invalid schema/path/digest/node ID/parent/range. Existing selected integration checks cover symlink confinement, evidence hashes, duplicate IDs, recursion depth, UTF-8 ranges, H2 fences and open extensions. No atomic filesystem snapshot, semantic AST certification, numerical percentage surrogate, full-system proof, teacher/network/corpus/model-loading/training claim. Raw logs, numeric statuses, exact commands, source bindings, sorted outcomes and mutation backups are retained in external wave34 run receipts.

## wave35-evidence

### Wave35 registry composition evidence

All 22 native modelcheck tests pass twice in debug and twice in release. Sorted named outcomes are identical across all four runs. Three new synthetic tests exercise group directory selection/confinement, composition preconditions and union-tree rejection, and strict unregistered-sidecar inspection followed by composed ownership. Existing tests retain their original finite scopes.

Seven actual CLI cases pass twice per profile (28 process invocations): composition, deterministic recomposition, existing-output refusal without byte changes, invalid union rejection before output creation, invalid overlay schema rejection, wrong arity rejection, and groupants discovery of the selected group directory. Synthetic fixture inputs remain unchanged.

Four compiled production mutants fail named assertions: force default group directory, bypass override path/type guards, omit composed-tree validation, and omit base-tree validation. Exact original bytes are restored and checked before the final positive runs. These are feature controls, not evidence of a previously claimed runtime bug. No assertion-only mutant is counted.

Package all-target Clippy completes with existing warnings; strict -D warnings fails on prior library lints. The initial escape fixture used an incorrect diagnostic substring; correcting that assertion required no production change. Source projections repeat byte-identically, all owned manifest hashes and byte scopes are refreshed, and ancestor bindings are left to integration. Raw logs and controls are under the external wave35 registry run directory. Structural declarations and finite synthetic checks do not establish semantic proof, all-source coverage, whole-system performance or an approximation bound.

Final structural inspection reports no diagnostics in either owned file tree. Its only four failures are stale integration-owned source hashes in model-infrastructure and mova ancestor manifests, for the two changed sources. The changed-source gate passes with exactly lib.rs and main.rs registered; focused rustfmt and diff-whitespace checks pass. Integration must refresh those ancestor bindings before claiming a globally valid tree.

## History

### 2026-10-29 · descriptant

Subject: adjacent lib.rs at the manifest source hash. Epoch: 2026-10-29.
The library inventories a declared model tree, changed Rust files and adjacent
groupant artifacts, including group/root manifests discovered from registry IDs.
It also extracts a bounded Lean-generated policy table.

Group epoch, 2026-10-29: the forward projection includes one row per registry
group/root, sorted by ID. Its declaration is accepted structurally only when
immediate children match as sets without duplicates, and source bindings cover
exactly every descendant registered source at its current hash. Group and file
manifest IDs share one uniqueness set. Missing manifests are backlog; malformed
existing groups contribute diagnostics and invalidate the report. Safe filename
IDs are required before discovery. Component artifacts retain canonical
repository confinement and open typed-family semantics.

Section epoch, 2026-10-29 (prospective): optional component section selectors
should fail closed for absent, duplicate, fenced-only or unreadable catalog
entries, equally for files, groups, typed members and extra component records.
The forward projection adds UTF-8 loading and a linear fence-aware line scan
of each selected path, accepting precisely one exact plain H2 line. No section
keeps the previous reference contract; unknown nested fields remain opaque.
The invariant adds nonempty single-line selectors with no surrounding whitespace
and an explicit path. The narrow catalog grammar is specified in the README;
this is not a full Markdown parser or mathematical adequacy check.

Observed 2026-10-29: cargo test -p mova-modelcheck passes all five tests;
both inventory regressions include section controls with real temporary files.
Positive file/group reports repeat identically. Missing/duplicate/fenced headings,
bad selectors, absent paths/files and non-UTF8 artifacts all produce invalid
reports; no-section binary references and nested open extensions remain valid.
This supports the predicted structural gate only, with no semantic verification.

Syntax epoch, 2026-10-29: the library exports syntax::project for selected Rust
AST structure. Its separate file groupant owns parsing/coordinate models.
File manifests opting into ast.format rust_syntax_projection additionally
validate artifact schema/kind, current source identity, lengths, parent order
and UTF-8 range integrity. The baseline complement excludes acceptance of stale
artifacts even after manifest rebinding; the reverse relation now constrains
compatible artifact metadata and node bounds. Validation does not reparse or
prove AST/source correspondence. Components without the format keep the earlier
contract, and group AST composition is still a gap. Added cost is linear in
artifact/source bytes and nodes. Twelve tests pass, including stale/wrong-source
and malformed-range controls; the five prior library tests remain passing.

Prospective contract, 2026-10-29. A registry file owner may have a separate
finite tree T of fragment declarations in its adjacent sidecar. A fragment
binds the same full source snapshot and carries its own scope and core models.
Let D(v) be local schema, source, artifact, type and range diagnostics, and let
E(v,c) check child ownership, containment and inherited range constraints.
The structural forward relation combines these diagnostics:

Observed 2026-10-29: all thirteen modelcheck tests pass. Nested controls include
27 malformed-declaration variants, valid overlapping siblings, shared IDs across
source files, malformed/missing descendants, stale and malformed child AST
artifacts, Unicode range boundaries and a real 65-level attempted traversal.
The validator admits at most 64 fragment levels and diagnoses deeper children.
Declaration preorder is preserved; unchanged reports compare identically in
the temporary-repository checks. The learning example contains four fragments
and twelve additional typed models; the Rust learning source is unchanged.

Prospective model, 2026-10-29. For a nonempty finite selection S of distinct
node IDs in a current source-bound partial projection, define its interval
envelope H(S)=[min_{i in S} start_i,max_{i in S} end_i). The optional syntax_nodes
scope is admitted only when its declared byte_range equals H(S), in addition
to the existing source, UTF-8 and ancestor-range predicates.

Observed 2026-10-29: all fourteen modelcheck tests pass (0.21 s test execution).
The new integration fixture checks single and disjoint node selections, reversed
selection order and overlapping root/item selection. It rejects malformed IDs,
missing/unconfined paths, mismatched ranges, malformed/foreign/stale artifacts
and a source edit despite rebound manifests. Removing the optional anchor keeps
the prior range-only contract. These are structural acceptance checks.

Epoch 2026-10-29. Optional components.rust_approximants is an array; absence
preserves the previous contract. Each record has a unique nonempty id, path,
sha256, source binding, budget_percent, error_percent, metric, domain,
assumptions, bound_kind, evidence and status. Rust artifact paths resolve from
the manifest directory and must canonically remain inside that sidecar with an
.rs extension. Its SHA-256 must match current bytes. Source is exactly the
manifest source object and must identify an existing repository-confined file
with a current hash. Each evidence entry likewise has path and sha256; the
evidence array is nonempty. Assumptions is an array of nonempty strings (empty
means no additional assumptions declared). Metric, domain and status are
nonempty strings. Bound kind is analytical or empirical. Separate profile IDs
may share a Rust artifact.

Observed 2026-10-29: all fifteen modelcheck tests pass (0.56 s test execution),
including the new attachment gate and existing nested/AST/section controls.

### 2026-10-30 · descriptant

Classification epoch, 2026-10-30. The forward predicate now requires the three
classification fields enumerated in the modelcheck README. Empirical bounds
must carry finite_observation assurance; analytical bounds carry analytical or
per_input_certificate assurance. Extra classification fields remain opaque.
The complement excludes missing/unknown classifications, mislabeled assurance,
budgets outside {1,2,5,10}, and declared errors exceeding 10 percent even when
an invalid larger budget is supplied. The reverse relation additionally
constrains compatible declarations to those labels and the tighter cap; it
cannot infer their mathematical truth. The combiner conjoins these predicates
with existing reference checks. Added work and memory per record are constant.

Observed 2026-10-30: all fifteen modelcheck tests pass (0.49 s execution).
Controls cover all 48 supported metric/method/assurance combinations, all four
inclusive tier boundaries, 11/15/25 and just-over-10 budgets, independently
over-10 errors, absent/malformed/unknown classifications and all three
assurance mismatches. Legacy manifests without attachments and additional
classification fields remain accepted. No runtime approximant accuracy is
established by these structural tests.

Interval-certificate epoch, 2026-10-30: method_kind additionally accepts interval_certificate. The independent metric/method/assurance cross-product now tests 60 combinations, preserving the four old methods. Unknown method labels still fail; budget, evidence and assurance predicates are unchanged. Forward and reverse declaration relations include the new label; their complement excludes other labels. Added per-record cost is constant. All fifteen tests pass. This is schema validation, not certificate proof.

### 2026-10-31 · descriptant

#### Wave33 current source review — 2026-10-31

Preserved all earlier descriptions and typed models; current model.md#forward, model.md#invariant-and-cost and model.md#composition refine scope, effect order and repeated IO costs. The current syntax.json binds exact source bytes; scoped children share it and retain nonempty byte intervals. Current tests and mutation limits are in model.md#wave33-evidence. The membership optimization replaces only existential scans over the already sorted Git path vector; its reference equivalence and paired kernel timings do not claim faster whole inventories. Executable percentage approximants remain unsupported for these categorical validation decisions: no useful loss metric/domain has been established. Future caps remain 1, 2, 5 and 10 percent.

### 2026-10-31 · wave33-evidence

Heading `### Wave33 native evidence — 2026-10-31` stays in wave33-evidence as `### Wave33 native evidence`.

### 2026-10-31 · wave34-evidence

2026-10-31. Five new scoped groupants in this source; ten across library and syntax, with sixty typed views. Earlier scoped IDs and model files remain. Nine exact target-qualified tests pass twice debug and twice release (36 invocations); sorted named outcomes match all four profiles. Tests use real implementation, synthetic JSON/Rust and temporary repositories.

Source SHA-256: c50b445397b9f6cc7d132254f15838f30f733e9bcaed832912f85b639f675bcf. Syntax artifact SHA-256: 28a4eae04e2a027672b12fd10e0a8991dcb87d2928526bb7d31d961584870ffd; two installed-tool materializations are byte-identical. Source-bound partial syntax is not semantic proof.

### 2026-10-31 · wave35-evidence

Heading `### Wave35 registry composition evidence — 2026-10-31` stays in wave35-evidence as `### Wave35 registry composition evidence`.

### 2026-11-05 · public export

groupants reads projections through ArtifactReader::projection (ast and coverage.syntax_nodes) and reports syntax_projections_regenerated per file and in total. Controls in fragment_syntax tests: a removed syntax.json validates and counts as regenerated; with the source edited but the manifest hash not rebound it stays invalid (SHA-256 diagnostic). cargo test -p mova-modelcheck --lib: 41 passed, 2026-11-05.

### 2026-11-05 · absent ast projection

validate_references skips the existence check of an ast component whose format is rust_syntax_projection and whose path is an absent syntax.json: the projection reader rebuilds and validates it. Found on the public export (1 985 projections regenerated, groupants valid on the public registry alone, 11.9 s). Control: the declared ast projection removed stays valid and counted; another absent ast file name is still an error. 41 passed.

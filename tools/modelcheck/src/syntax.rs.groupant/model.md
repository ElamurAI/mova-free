# modelcheck-syntax

## descriptant

### Source-bound Rust syntax groupant

Forward approximant: admit normalized repository-relative .rs paths from Git
discovery; canonicalize inside the repository; read UTF-8 bytes once; parse;
visit selected node classes in deterministic preorder; map scalar span columns
to UTF-8 byte offsets; emit source identity, limits, nodes. Invalid admission,
parsing or spans fail before writing. The CLI uses the existing exclusive writer.

Invariant: node 0 covers all source bytes, other parent IDs precede children,
ranges are half-open UTF-8 boundaries enclosed by the nearest projected parent,
and SHA-256 binds exactly the parsed bytes. Macro token streams are opaque.
The open family begins with behavior complement U minus R and the converse
relation; model.md#catalog specifies twenty different observation scopes/operators.

Cost prediction before implementation: linear selected traversal and coordinate
mapping beyond parser and Git inventory cost, O(B+N+G+D) retained/transient
storage (bytes B, selected nodes N, Git output G, depth D). Git path sorting is
O(P log P). Deep recursion and allocation failure remain unbounded; no measured
speedup is claimed. Artifact integrity validation additionally reads source and
JSON, checks hashes/bounds, and is linear in their sizes.

Combiner: compose discovery, confinement, parsing, projection and exclusive
serialization as partial functions. AST data plus forward model, typed family,
invariants and descriptant form the file groupant. A syntax artifact supplies
structural input for future combiners; it does not produce new mathematical
models or prove the existing ones. The parent must model command dispatch and
registry/manifest wiring separately.

Epoch memory: earlier model groupants left all ASTs not_materialized. This
implementation closes the selected Rust structural projection gap only.
Attributes/comments/tokens, many fine AST fields, macro expansion, cfg
evaluation, out-of-line modules, resolution and inferred types remain gaps.
Unicode span conversion is necessary: proc_macro2 columns are scalar counts,
so direct use as UTF-8 byte offsets would be wrong. The regression includes
emoji before code, multibyte identifiers/literals and CRLF.

Span review caught an additional normalization boundary: syn strips an initial
UTF-8 BOM before forming spans. The coordinate table now applies that three-byte
base offset while preserving the original source digest/root range. A seventh
syntax test covers BOM, shebang and their CRLF combination; all twelve crate
tests pass after the correction.

#### Wave34 exact-byte decode reuse

Repeated shared projection decoding was investigated after wave33 membership timing failed to move whole-command time. A source-tree-local one-entry memo now reuses parsed JSON only after a fresh byte read and full equality. Source hashing and every artifact guard still run on each reference. Earlier epochs above are historical; current bindings and ranges are in manifests. See model.md#wave34-evidence for finite tests, controls and measurements.

## composition

### File-local composition

Each child is a scoped view of the same authoritative source and reuses its parent syntax artifact. Byte intervals need not cover tests or all helpers; this is not complete semantic coverage. Old descriptant epochs remain preserved.

- admission-parsing: Path admission, sorted Git membership, source read and syn parse projection.
- coordinates-traversal: BOM-aware Unicode scalar coordinate table and selected AST preorder visitor.
- artifact-integrity: Source-bound JSON projection identity, UTF-8 ranges and parent integrity.

Admission supplies exact text to parsing and coordinate/traversal state; parse emits a projection only if invalid_span remains false. Artifact validation is an independent consumer called by lib.rs for declared projections, not automatically part of project. The shared membership helper requires sorted rust_files output. No claim that artifact integrity proves the visitor result.

## catalog

# Syntax projection model catalog

## Behavior complement

Domain: U above. Operator: C = {(x,y) in U : y claims success and violates
normalized Rust path admission, Git discovery, confinement, UTF-8 parsing, source
identity or valid nested ranges}. Target C subset U minus R. Fixed conditions:
no concurrent mutation. Oracle: malformed/path/escape tests must reject.
This is a conjunction-derived exclusion model, not a list of negative examples.

## Reverse reconstruction

Domain: successful reports. Operator: F(y) = {x : project(x)=y}; constraints are
same source path, byte count, SHA-256, parser tree and limits. Hashes are not
proved injective. Output destination, ignored files and unrelated repo contents
remain free. Oracle: repeated unchanged source gives identical serialized y;
changing unused repository files should preserve y (latter proposed).

## Git membership complement

Domain: normalized relative .rs paths absent from git ls-files discovery.
Operator excludes success independently of whether a file physically exists.
Ignored-file rejection is evidence. This is current worktree discovery, including
nonignored untracked files, not proof of tracked ownership.

## Path normalization complement

Domain: paths with absolute roots, dot/dot-dot or empty slash components, or
non-.rs suffix. Operator excludes success before reading source content.
Oracle: path matrix test. On non-Unix systems separator semantics additionally
depend on std::path; platform equivalence is not claimed.

## Confinement complement

Domain: Git-discovered sources whose canonical target escapes the canonical
repository root. Operator excludes success. Oracle: Unix symlink escape test.
TOCTOU races are excluded from the fixed-snapshot assumption.

## Encoding complement

Domain: admitted byte strings not valid UTF-8. Operator excludes parser/report
success. Oracle: 0xff source test. No encoding detection or lossy recovery.

## Grammar complement

Domain: valid UTF-8 rejected by syn::parse_file. Operator excludes successful
projection. Oracle: unterminated function test. This does not classify Rust
compiler acceptance: cfg, semantics and compiler extensions remain outside it.

## Source identity reverse

Domain: reports with observed (path, digest, bytes). Operator returns strings s
at that path satisfying len_utf8(s)=bytes and SHA256(s)=digest. This is an outer
constraint on the full preimage, not a digest inversion algorithm. Oracle:
source-bound test recomputes the digest from exact input bytes.

## Tree topology reverse

Domain: node arrays. Operator constrains each nonroot antecedent node to its
nearest projected AST ancestor with lower ID; intervening unprojected AST
records remain unknown. Oracle: nested test checks earlier parent and enclosure.
This necessary constraint alone does not reconstruct omitted AST fields.

## Traversal order complement

Domain: emitted preorder node arrays. Operator excludes a nonroot parent ID
greater than or equal to child ID and excludes duplicate/noncontiguous IDs.
Source-derived induction: enter appends at len, pushes; leave pops after visit.
Oracle: nested parent ordering test; artifact validator rejects self-parenting.

## Range containment complement

Domain: emitted nodes and immutable source bytes. Operator excludes start>end,
child ranges outside parent, or offsets not represented in the scalar-to-byte
table. Invalid parser spans fail the whole call. Oracle: nested and Unicode
tests plus artifact out-of-bounds and split-codepoint controls.

## Unicode coordinates reverse

Domain: line/scalar positions and source text. Operator recovers byte offset
as line start plus sum of UTF-8 lengths of preceding scalars; inverse retains
only scalar boundaries. CRLF counts CR as a scalar before LF resets the line.
Oracle: emoji before a function and multibyte identifiers/literals yield exact
slices. Combining characters are not grapheme columns.

## Macro opacity complement

Domain: macro invocation bodies. Operator excludes nodes interpreting body
tokens as Rust expressions, even if the tokens look parseable. A macro node
may be nested under an opaque item/expression/statement wrapper.
Oracle: non-Rust token body parses and has no children under its macro node.

## Macro body reverse

Domain: the structural projection after hiding source identity and ranges.
Operator admits all well-delimited token bodies with the same enclosing macro
shape. This is a many-input fiber, not unique recovery; exact digest observation
would further constrain it. Oracle: token substitution experiment proposed.

## Category preservation reverse

Domain: item, impl-item, trait-item, foreign-item, statement, expression,
pattern, type and block category observations. Operator constrains antecedents
to the corresponding syn variants, with Unknown as an explicit future gap.
Oracle: nested example covers item/impl/trait/statement/expression/pattern/type;
foreign items and future variants require additional coverage.

## Attribute omission reverse

Domain: reports projected further to category/variant/parent triples.
Operator allows attribute changes when parsing and selected AST structure stay
fixed; cfg is never evaluated. Digest and offsets are intentionally hidden.
Oracle: attribute substitution comparison proposed. Doc comments are attributes
in syn and therefore share this omission boundary.

## Module boundary complement

Domain: out-of-line module declarations. Operator excludes loading external
module files or incorporating their AST nodes. Inline modules are traversed.
Oracle: nested test covers inline modules; absent external file test proposed.
The Git inventory lists paths but does not read all discovered source bytes.

## Empty source reverse

Domain: a single File node with range [0,0). Operator constrains source bytes
to the empty UTF-8 string, while repository surroundings and output destination
remain nonunique. Oracle: empty source test. A one-node nonempty file can also
contain only comments; zero bytes is the crucial additional observation.

## Artifact binding complement

Domain: opt-in ast format rust_syntax_projection declarations. Operator excludes
successful validation when artifact kind/schema/source path/digest/length differs
from the adjacent source, or node IDs/parents/byte boundaries are malformed.
Oracle: stale artifact after manifest rebinding, wrong path/kind/length and
range controls. This is integrity validation, not AST semantic correspondence.

## Resource budget reverse

Domain: byte budget B, selected node count N, Git output size G and nesting D.
Operator constrains feasible calls to storage for source/AST, scalar boundary
table, report nodes and traversal stack: O(B+N+G+D), plus syn internals.
Projection traversal/time is O(B+N) beyond parser and Git inventory costs;
rust_files sorting contributes O(P log P). Deep recursion can exhaust stack;
there is no hard resource admission bound or benchmark-based speed claim.
Oracle: size scaling and adversarial depth tests remain proposed.

## forward

### Current forward model

Subject: adjacent source at manifest SHA-256. Fixed immutable snapshots and adequate resources are assumed; errors/races and allocation limits remain explicit.

#### admission-parsing

Reject nonnormalized/non-Rust path, canonicalize root, require sorted discovery membership, confine file, read UTF-8, parse with syn, visit and reject invalid spans. Emit exact input path/hash/bytes, parser identity and limitations.

#### coordinates-traversal

Build per-line scalar boundary byte offsets, shifting base by three for leading BOM. enter converts Span endpoints, checks containment, appends sequential node ID and pushes; visitors recursively visit selected syn fields then pop. Macros enter/leave without body traversal; attributes are skipped.

#### artifact-integrity

Check schema/kind/path/digest, reread source and rehash against expected digest, then byte count/unit/parser/limitations. Iterate nodes requiring contiguous index IDs, valid ranges/categories/opacity, full-file root and earlier containing parent.

## invariant-and-cost

### Current invariants and cost

These are source-derived partial constraints. Conditional costs count repeated IO and do not certify runtime bounds or end-to-end gains.

#### admission-parsing

Projection source binds the same UTF-8 bytes passed to parse. Failed admission/parsing produces no Projection. Git discovery includes nonignored untracked files. No external module, cfg evaluation or macro expansion occurs.

Git process/sorting O(P log P), membership O(log P) comparisons, source read/hash O(B), plus syn parse and selected traversal. No claim of bounded parser recursion or whole-command speedup.

#### coordinates-traversal

Root covers original bytes; parent precedes child and contains child interval. Scalar columns are not bytes or grapheme columns. Invalid lookup sets invalid_span and parse later fails; node construction may continue internally.

Boundary table O(B) byte-equivalent storage, selected nodes O(N), stack O(D). enter offsets O(1); visitor O(N plus unprojected visited syn fields). Parsing/recursive stack exhaustion is outside finite test evidence.

#### artifact-integrity

Accepted artifact is structurally bound to current read bytes. Parent<index makes ranges[parent] safe; UTF-8 get validates boundaries. The validator does not reparse or verify category/variant corresponds to source semantics.

Read/hash O(B), node loop O(N) with O(N) stored ranges, plus already parsed artifact JSON. Repeated fragment references repeat all work; no shared cache or atomic repository snapshot.

#### Wave34 cost correction

Syntax artifact JSON decoding may now share one exact-byte memo per source tree. Filesystem confinement, fresh reads, source rehash and full node validation remain repeated; Markdown section reads and hashed evidence are unchanged. Old no-cache statements above describe wave33. No cache survives a source-tree invocation.

## wave33-catalog

# Wave33 additional typed models

Subject/revision: adjacent source at current manifest hash. Preserve the original baseline families and historical catalogs. These additions are source-derived partial constraints on one call at a fixed snapshot, with adequate resources. Full mathematical correspondence, races and automatic inverse execution remain gaps.

## sorted_admission_membership

Admission consumes sorted rust_files output and applies exact binary membership. Unicode/prefix/boundary differentials compare with the old linear reference; ignored-file and escape tests exercise complete project admission.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## attribute_shape_fiber

Holding projected parent/category/variant/opaque triples fixed, adding cfg(any()) attribute to a function preserves shape while bytes/hash/ranges may change. Executed finite omission test now replaces the earlier proposed-only example.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## external_module_opacity

An out-of-line mod absent declaration projects even when absent.rs does not exist. A foreign-item declaration is visited. Neither external-module contents nor cfg-selected semantics are loaded.

Observation and oracle: current named native checks and scoped CLI probes in model.md#wave33-evidence; future generalization remains unproved. Compatible antecedents retain all unobserved filesystem, omitted syntax and model-semantic choices.

## artifact_integrity_boundary

Node strings and interval containment are structurally checked without reparsing. Zero-length node ranges may pass, but fragment scopes must be nonempty. Fresh hash reread is not an atomic repository snapshot.

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

## wave34-composition

### File-local composition

Each child is a scoped view of the same authoritative source and reuses its parent syntax artifact. Byte intervals need not cover tests or all helpers; this is not complete semantic coverage. Old descriptant epochs remain preserved.

- admission-parsing: Path admission, sorted Git membership, source read and syn parse projection.
- coordinates-traversal: BOM-aware Unicode scalar coordinate table and selected AST preorder visitor.
- artifact-integrity: Source-bound JSON projection identity, UTF-8 ranges and parent integrity.

Admission supplies exact text to parsing and coordinate/traversal state; parse emits a projection only if invalid_span remains false. Artifact validation is an independent consumer called by lib.rs for declared projections, not automatically part of project. The shared membership helper requires sorted rust_files output. No claim that artifact integrity proves the visitor result.

### Wave34 extension

#### decode-memo

Read artifact bytes on every call. In memo mode return the immutable saved Value only on full byte equality. Otherwise decode with serde_json; on success retain one entry only at <=8 MiB, clearing on oversized/reference success. Read or parse errors propagate without replacing the slot. New reads in reference mode always decode.

The child refines the parent call at this exact source range. Shared source identity and error order remain coupled; this view does not add a second execution or independent cost. Returned Value equals decoding the just-read bytes; no validation result or filesystem metadata is cached. One source-tree reader is dropped after traversal. Byte cap bounds serialized retention, not serde Value allocator overhead.

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

## History

### 2026-10-29 · descriptant

Subject: adjacent syntax.rs at manifest source.sha256. Epoch: 2026-10-29.
This module projects syn 2 file parsing into an ordered selected AST tree. The
source stays authoritative. A projection is an inspectable artifact, not a
compiler AST, a lossless concrete syntax tree, or a verified behavioral model.

Observed checks: seven syntax/artifact tests and five unchanged library tests
pass on 2026-10-29. Negative controls include invalid syntax and encoding,
unsupported/ignored/escaping paths, exclusive output, stale AST after manifest
rebinding, wrong artifact identity, bad parent and invalid Unicode ranges.
Catalog-specific proposed experiments remain labeled and are not inferred
from passing the aggregate test suite. Opt-in artifact validation checks
structural integrity without reparsing or proving node/source correspondence.

### 2026-10-29 · document

Subject/revision: ../syntax.rs, bound by manifest source.sha256. Epoch: 2026-10-29.
These are source-derived partial relational specifications, not proofs or an
executed inverse engine. Each entry observes one completed project/parse call
on fixed filesystem bytes with adequate memory and stack. U is the Cartesian
product of admissible request snapshots and candidate success/error outcomes;
R is actual implementation behavior. Unless stated otherwise, complement
claims are proposed inner bounds of U minus R, and reverse claims constrain
compatible antecedents without claiming completeness outside this scope.
Evidence: seven syntax tests, existing library regressions, and source review.
Entries identify narrower evidence; untested generalization is a hypothesis.
Errors do not expose a partially built projection.

### 2026-10-31 · descriptant

#### Wave33 current source review — 2026-10-31

Preserved all earlier descriptions and typed models; current model.md#forward, model.md#invariant-and-cost and model.md#composition refine scope, effect order and repeated IO costs. The current syntax.json binds exact source bytes; scoped children share it and retain nonempty byte intervals. Current tests and mutation limits are in model.md#wave33-evidence. The membership optimization replaces only existential scans over the already sorted Git path vector; its reference equivalence and paired kernel timings do not claim faster whole inventories. Executable percentage approximants remain unsupported for these categorical validation decisions: no useful loss metric/domain has been established. Future caps remain 1, 2, 5 and 10 percent.

### 2026-10-31 · wave33-evidence

Heading `### Wave33 native evidence — 2026-10-31` stays in wave33-evidence as `### Wave33 native evidence`.

### 2026-10-31 · wave34-evidence

2026-10-31. Five new scoped groupants in this source; ten across library and syntax, with sixty typed views. Earlier scoped IDs and model files remain. Nine exact target-qualified tests pass twice debug and twice release (36 invocations); sorted named outcomes match all four profiles. Tests use real implementation, synthetic JSON/Rust and temporary repositories.

Source SHA-256: 35c315cc9b581bea50ea2a7dcb2ec40209f6f836e721ccf5b307764aa2b62d30. Syntax artifact SHA-256: ca9c5568df584f320f0d26c0e7c26cf76d0c09f39d0756584ad8232732193cbf; two installed-tool materializations are byte-identical. Source-bound partial syntax is not semantic proof.

### 2026-11-05 · public export

ArtifactReader::projection: an absent syntax.json (public exports omit projections) is regenerable, not an error: the projection is rebuilt in memory from the current source with parse() and validated like a stored one; the reader counts it (regenerated). Any other name, or a present file, is read as before through confined_file. Reverse reconstruction: a regenerated projection equals what mova-modelcheck syntax would write for the same bytes, so node ids selected by fragments stay valid exactly when the source hash binding is current. Behaviour complement: regeneration never makes a stale manifest hash current (the source hash check is separate).

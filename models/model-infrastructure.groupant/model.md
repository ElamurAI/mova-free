# model-infrastructure

## model


## Versioned survey measure admission

The external reader keeps historical unprofiled scalar-character surveys distinct from explicitly word-profiled surveys. It recomputes the declared unit on actual description bytes and checks metric/profile/count agreement; an unknown profile or conflicting measure cannot be interpreted as a different admitted edition. This is artifact and representation admission, not scientific verification. Algorithm repository-source pins and paper provenance-only scope retain their different binding counts. Expanded source/context limits and a 4,000-word description do not create a full-paper semantic AST, proof or original-text authentication.

## External source customization boundary

The module-overlay subtree composes a bound public base and separately retained
add/replace files into a fresh tree. Exact source bytes, output receipts and base
drift are observations; compatibility of Rust interfaces, retained knowledge and
source-bound groupants requires separate checks. Upstream remains unchanged.
The composed tool never runs arbitrary hooks or silently refreshes a stale model
into a proof. A config-only pipeline can use existing capabilities; replacement
source modules use explicit rebuild recipes and renewed compatibility evidence.

## catalog

# model-infrastructure typed model catalog

All entries are partial specifications with fixed-source and trusted-filesystem assumptions from model.md#core-model.
Evidence is source inspection, not execution of every oracle. Oracles are proposed unless model.md#tests records execution.
ASTs and executable reverse reconstruction are not materialized. Specialized views overlap; counts are not independent proofs.

## absent-accepted-infrastructure

Type: behavior_complement. Domain/observation: Successful infrastructure command input/report pairs.
Operator/approximation: For U all such pairs and R actual successes, C={accepted invalid tree, accepted stale declared sidecar, changed success with missing changed file, accepted incomplete table} is a source-derived inner bound of U minus R.
Limits/assumptions: Command-specific guards only; absent sidecars are not invalid declarations.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Malformed fixture for each disjunct must reject.

## compatible-report-inputs

Type: reverse_reconstruction. Domain/observation: Reports with ownership and source/registry hashes.
Operator/approximation: B(y) retains all repository and registry snapshots producing y under this projection.
Limits/assumptions: Hashes do not uniquely reconstruct source; unrelated files remain unconstrained.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Vary an unrelated non-Rust file and preserve inventory report.

## git-visible-domain

Type: domain_projection. Domain/observation: Tracked and nonignored untracked Rust paths.
Operator/approximation: F=sorted unique union(cached Rust paths,visible untracked Rust paths).
Limits/assumptions: Tracked deleted files can fail later reads; ignored untracked files absent.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Fixture with tracked, untracked and ignored Rust paths has expected membership.

## physical-logical-join

Type: ownership_join. Domain/observation: Git set F and registry source map L.
Operator/approximation: Require dom(L) subset F; attach owners and compute physical directory counts separately from logical tree.
Limits/assumptions: Ownership is not a call/import graph.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Move a registered source without updating node and reject.

## tree-preconditions

Type: precondition. Domain/observation: Schema, records, edges and paths.
Operator/approximation: Accept schema1, complete fields, unique IDs/source paths, one parent and full root reachability.
Limits/assumptions: Check names remain inert declarations.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Reject disconnected cycle and double parent.

## group-transitive-binding

Type: dependency_closure. Domain/observation: Immediate children and descendant sources.
Operator/approximation: Require exact child set and exact descendant path->current SHA mapping.
Limits/assumptions: No unregistered dependency is discovered.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Stale grandchild hash fails even without leaf sidecar.

## component-reference-confinement

Type: path_constraint. Domain/observation: Component path/description and registry references.
Operator/approximation: Canonicalized sidecar-relative references must name files within repository.
Limits/assumptions: Internal content is not semantic proof; arbitrary extension strings are not references.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Reject missing file and escaping symlink.

## required-family-accounting

Type: type_obligation. Domain/observation: Ordered typed family entries.
Operator/approximation: Account for behavior_complement and reverse_reconstruction; coverage output lists them in that order. Manifest array ordering is not enforced. Additional model entries need distinct IDs and status.
Limits/assumptions: Explicit status not_materialized accounts for an absent model without supplying it; a missing status is invalid.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Remove baseline type and observe diagnostic.

## missing-invalid-partition

Type: failure_partition. Domain/observation: Sidecar discovery results.
Operator/approximation: Absent is backlog; malformed/stale declared sidecar is invalid; CLI writes report then fails on invalid declarations.
Limits/assumptions: Registry/Git errors may occur before report writing.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Compare an absent sidecar directory and a sidecar with malformed JSON; an existing directory without manifest.json is invalid.

## changed-file-frontier

Type: change_projection. Domain/observation: Base revision and current visible Rust files.
Operator/approximation: Delta=existing git diff BASE paths union visible untracked Rust; gate iff Delta subset registered sources.
Limits/assumptions: Deleted paths filtered; inventory may independently reject deleted registered source.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Add unregistered untracked Rust fixture; missing_changed_files must name it.

## changed-gate-not-proof

Type: evidence_separation. Domain/observation: Changed success and repository registration.
Operator/approximation: all_changed_files_registered implies neither all_rust_files_registered nor tested behavior.
Limits/assumptions: Fresh model content and semantic correspondence are unchecked.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Empty Delta with legacy backlog passes changed and fails require-all.

## exclusive-report-write

Type: artifact_monotonicity. Domain/observation: Output absent/present and serialization.
Operator/approximation: create_new then write_all and sync_all rejects existing outputs.
Limits/assumptions: Partial new file can survive I/O failure; no atomic multi-file transaction.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Precreate sentinel output; error must preserve bytes.

## format-table-domain

Type: finite_domain. Domain/observation: Rows following exact Lean table header.
Operator/approximation: Keys cover {0,1,2,3}^4 once (256 rows), fifth column Boolean; row order preserved.
Limits/assumptions: Format does not establish truth or Lean provenance.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Delete/duplicate row, exceed domain or append malformed row and reject.

## table-truth-gap

Type: semantic_correspondence. Domain/observation: Complete table and Rust comparison.
Operator/approximation: Separately compare Rust(k) with table(k) for all finite keys; extraction itself accepts any Boolean assignment.
Limits/assumptions: Finite correspondence does not establish unbounded semantics.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Flip a Boolean: extraction accepts, correspondence must fail.

## lean-log-failure-filter

Type: failure_projection. Domain/observation: Raw Lean output log.
Operator/approximation: Reject error:, sorryAx and declaration uses 'sorry' markers before extracting table.
Limits/assumptions: Text filtering does not authenticate process exit or producer.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Append each forbidden marker to complete log and reject.

## formal-positive-negative-pair

Type: negative_control. Domain/observation: Nine installed specs and prescribed replacements.
Operator/approximation: Positive must succeed; exactly one formula substitution must fail with error and without timeout.
Limits/assumptions: Ignored harness needs installed toolchain; inventory never executes it.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Run explicit ignored harness only in authorized installed environment.

## axiom-report-preconditions

Type: proof_assumption. Domain/observation: Positive Lean axiom reports.
Operator/approximation: Require expected per-file report count, no sorry, only propext/Classical.choice/Quot.sound dependencies.
Limits/assumptions: Counts do not certify intended theorem names or Rust correspondence.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Future isolated parser fixture drops a report or adds unexpected axiom and rejects.

## formal-process-budget

Type: resource_budget. Domain/observation: Nine serial positive/negative spec pairs.
Operator/approximation: The complete two-test harness plans 20 calls, each with a 90-second timeout; per-test loops are serial, tests may run concurrently. A 1800-second summed timeout budget excludes overhead and assumes timeout/process behavior.
Limits/assumptions: Early failure stops loop; process availability and cleanup assumed.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Instrument start/end times during an explicitly authorized full harness run.

## inventory-cost-frontier

Type: cost_projection. Domain/observation: F paths, B source bytes, N nodes and group closures.
Operator/approximation: Inventory hashes O(B), ordered bookkeeping roughly O(F log F+N log N); group checks add closure and artifact reads.
Limits/assumptions: Repeated reads, path depths and reference sizes matter; no speedup measured.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Bounded Rust instrumentation counts bytes and closure visits.

## stable-report-preconditions

Type: determinism. Domain/observation: Two fixed-input runs.
Operator/approximation: Ordered sets/maps and preserved manifest arrays yield identical serialized reports.
Limits/assumptions: Filesystem is not snapshotted; concurrent edits invalidate premise.
Evidence: Source inspection of tools/modelcheck/src/lib.rs, tools/modelcheck/src/main.rs and gparse/tests/formal_neural.rs.
Oracle: Run groupants twice to distinct outputs and compare bytes.

## tests

### Evidence and checks

- groupants twice: valid=true, no diagnostics; 3 group sidecars and 1 missing group; 6 file sidecars and 525 missing file sidecars. Reports were byte-identical.
- All declared source hashes, transitive descendants, child sets, registry references and component paths passed structural validation.
- changed against ab4ba4321: changed=[] and missing=[]; no Rust source was modified by this change.
- A bounded standalone Rust probe checked 20 exact manifest section references in this catalog, one matching heading each, and domain/operator/limits/evidence/oracle fields. Both new catalogs passed (40 entries total).
- git diff --check passed before staging.

These checks establish artifact consistency, not mathematical adequacy. No live teacher, training, full Lean harness or new task-accuracy/performance measurement was run. ASTs and executable reverse reconstruction remain absent. Integration must refresh ancestor hashes after any registered source changes and rerun structural validation.

## core-model

### Model infrastructure interaction model

Subject/revision: the immediate children and transitive source digests in manifest.json. The group composes registry/CLI inspection, selected Rust syntax projection, the formal harness, mathematical timing twin, executable approximant checks and bounded mathematical correspondence discovery. Ownership closure is explicit; source semantic coverage remains partial. Finite converse/preimage evaluation exists for declared correspondence models, while automatic reconstruction of arbitrary Rust behavior remains absent.

#### Forward approximant

Let F be Git's sorted deduplicated tracked and nonignored untracked Rust paths, L the registry source->node map, T its declared tree, M discovered manifests and H hashes of actual bytes. Validate T before inventory: one reachable root, one parent per nonroot, distinct sources, required model fields. Require dom(L) subset F. Inventory joins F with L and reports both physical directory counts and logical declarations; all verification flags remain false.

Groupants further joins each discovered file sidecar with its source H and every group sidecar with exact immediate children and the transitive registered-source closure. Missing sidecars are backlog; malformed/stale declared sidecars invalidate the report. Component references must resolve to files within the repository. The CLI writes a new report before returning failure for invalid declarations. Earlier registry/Git failures may produce no report.

For changed BASE, Delta is the union of existing Rust paths reported by git diff BASE and nonignored untracked Rust files. The gate passes iff Delta subset dom(L), even with untouched legacy backlog. No Rust changes means an empty Delta, not new semantic evidence.

The formal branch is separately invoked and ignored by default. Ten installed Lean specs across two independently selectable tests each have a successful positive run, expected axiom report count and allowed dependencies; each prescribed one-occurrence formula mutation must fail with a Lean error without timing out. No downloads or live teacher work are part of this model.

extract-table joins a raw log to a finite fixture by exact header, five-column syntax, Boolean output and all 256 distinct keys in {0,1,2,3}^4. It rejects named error/sorry markers. Extraction checks coverage and format, not Boolean policy truth or authenticated Lean provenance. Assessment's finite Rust correspondence is another operation, outside these infrastructure leaves; passing extraction alone supplies no correspondence result.

#### Invariants and cost

Declared ownership and source digests bind snapshots, not semantic equivalence. Group closure is registry closure, not complete program dependency closure. References cannot escape the repository. Existing output files cannot be overwritten by write_new; a failed new write can still leave a partial file. Missing and invalid sidecars retain different meanings. Check names are never executed by inventory.

#### Descriptant and epoch memory

Earlier modeling distinguished registration from verification and made legacy missing files visible. Group validation subsequently added exact child and transitive-source bindings so a stale descendant cannot hide behind an absent leaf sidecar. This epoch retains both observations and adds twenty typed viewpoints.

Remaining gaps: expanded/semantic ASTs, semantic catalog validation, universal generation, full Rust/Lean correspondence, complete dependency discovery and snapshot isolation. A structurally complete arbitrary Boolean table can pass extraction; its truth belongs to a separate check. A changed gate can pass while require-all fails. These are intentional scope distinctions, not evidence of complete modeling.

The syntax branch emits selected preorder nodes, source hashes and byte ranges.
Its opt-in artifact gate checks source identity and basic tree/range integrity;
it does not reparse artifacts to prove syntax equivalence. Macros stay opaque.

#### Prospective timing feedback

The mathematical-twin subgroup connects an explicitly configured benchmark
forecast to measured evidence and a validated RLS update. Its cohort fingerprint
is a declared boundary, not complete dependency discovery. This branch can run
a requested command; inventory/changed/groupants remain read-only inspectors
of declarations. A timing forecast does not establish task quality, proof or
GPU numerical agreement. See ../mathematical-twin.groupant/model.md#model for source-
derived composition, numerical limits and resource costs.


#### Mathematical correspondence feedback

The mathematical-correspondence subgroup consumes validated ownership and exact source/model bytes, then compares manually declared typed finite relation ASTs. Converse exchanges endpoints, composition requires a matching intermediate sort, and complement retains its finite universe. Catalogue equality is not source-code equivalence. Native bridges, independent finite oracles and abstract Lean laws supply separate evidence; unformalized groupants remain visible in coverage. The resulting graph suggests future model reuse or measured code changes without changing ownership or replacing production code automatically. Review proceeds through docs/correspondences.md. The discovered ordinary gcd(0,0) panic and its focused repair are one source-level feedback result; mathematical similarity alone did not establish the repair.

#### Unverified source-survey reader

The external-library CLI adds a distinct source_survey_groupant branch. Artifact hashes, description shape and actual algorithm repository-path/SHA joins are separate from semantic correctness. Paper metadata receipts have zero full-text source bindings. This avoids silently inheriting canonical algorithm-bundle or paper-node claims; no automatic substitution/publication follows.

Publication adds an artifact-limited scope policy beside the independent external-library integrity reader. Exact versioned decisions, rights/evidence declarations and current hashes do not supply scientific correctness or publication side effects; model.md#publication records complement, converse and result partitions.

## publication

# Version-bound global publication admission

This composition joins caller-pinned publication policy, the existing external-library integrity reader and CLI report writer. It does not publish a site, authenticate reviewers or execute mathematical checks. Editorial scope, artifact integrity and scientific assurance remain separate observations.

## global_admission_complement

Type: behavior_complement. Domain: a supported external manifest edition m, bounded current store inventory, catalog c and caller-supplied reviews R with expected digest h. Let V require exact manifest edition and matching recomposed inventory/normalized selected decision. Let Q require H(R)=h=c.review_policy_sha256=current.review_policy_sha256. Let P be explicit independent-review/rights/evidence admission, I current reader integrity and A a nonempty exact declared narrative-artifact subset. Positive global permission requires V and Q and P and I and valid(A).

Canonical algorithm bundles, code-bound paper groupants and recognized implementation source surveys are excluded even with intact hashes or local global labels. Missing, unknown, incomplete or stale decisions never default to global. The old CLI form without caller policy/digest is negative-only. Counterexample: altering a catalog's allowed list or reason while retaining its schema/manifest pin does not supply the caller's trust root; recomposition must recover the same pinned decision. A byte-clean implementation and a licensed unreviewed survey also remain excluded. Evidence must lie under reserved reviews/, have no manifest ancestor, and have neither a path nor digest belonging to any artifact in the bounded complete inventory; copying a generated description into reviews/ is not admissible evidence.

Assumptions: trusted caller/editorial declarations, fixed local inputs during admitted reads, collision-resistant identities and recognized schema markers. Metadata disjointness does not authenticate independent human work or rights ownership. There is no atomic store snapshot or semantic discovery of a falsely labeled implementation. Evidence: source-derived branch composition and source-owner bounded positive/negative controls; proof remains false and generated/unverified mathematical status is preserved.

## artifact_allowlist_converse

Type: reverse_reconstruction. Domain: successful reports bound to exact manifest, catalog and caller-policy hashes, observing approved artifact paths/digests and separate editorial/mathematical status. Compatible antecedents satisfy those identity equalities, current inventory/decision agreement, artifact role and reader integrity, but need not have a unique reviewer or authorship history. The allowlist projects to A subset declared(m), not the whole bundle or directory. Only the declared description narrative role is supported. A selected digest equal to source.sha256 or a selected path equal to source.snapshot is excluded regardless of renaming or extension. Hash inequality does not prove originality or detect all modified copying.

Counterexample: approving description.md does not approve auxiliary.json, another bundled document or a later edited description. A renamed primary-paper copy with unchanged source digest remains excluded. Hash-pinned evidence establishes which bytes were cited, not whether their assertions are true. Retained source/manifest/artifact bytes refine compatible inputs; a small permission report cannot reconstruct omitted bytes or editorial reasoning. Editorial_status=independently_reviewed does not erase mathematical_status=generated_source_survey_unverified, and proof=false remains separate. Evidence: explicit artifact/role/digest checks and corruption controls; this is a compatible-input relation, not unique inversion, scientific correctness or a semantic-error bound.

## policy_integrity_result_partition

Type: error_partition. Domain: ordinary publication-check calls and CLI encoding. Missing/stale/unsupported scope, absent caller policy, bad caller pin or failed policy recomposition can produce a valid denied report before integrity is attempted. Valid pinned policy followed by reader failure also produces a denied report. Malformed JSON, duplicate/path-unsafe entries and I/O or changed-manifest races can instead return an error without a completed report. Thus integrity_checked=false does not uniquely distinguish not attempted from attempted and failed.

The CLI can write a denied report and exit successfully: process completion is not global admission. Consumers must inspect public_export_allowed and bound allowed_artifacts; the old no-policy form cannot positively authorize even an otherwise valid global catalog. A catalog's complete inventory/eligible-policy count is not a current reader result. Counterexample: a well-formed implementation edition returns ordinary denial, while malformed catalog JSON can prevent a report. The library reader may allow internal symlinks in unselected assets, whereas publication selection has stricter path checks; neither supplies race-free filesystem security. Evidence: branch inspection and finite reader/policy/CLI controls; no store or site effects in this model epoch.

## History

### 2026-10-29 · document

Subject/revision: this registry group at manifest.json source_bindings, inspected 2026-10-29.

### 2026-10-29 · tests

Subject: manifest-bound registered sources at the 2026-10-29 modeling epoch. Catalog oracles are proposed except the structural checks recorded here; no per-entry semantic execution is implied.

Executed 2026-10-29 with the installed mova-modelcheck on baseline ab4ba4321 plus these two groupants:

### 2026-10-29 · core-model

Inventory hashes O(B) source bytes plus ordered path/node bookkeeping approximately O(F log F+N log N). Group checks add repeated source/reference reads and closure traversal; no cache speedup is assumed. Table extraction scans log bytes and a fixed 256-key domain. The two formal tests plan twenty Lean processes with ninety-second timeouts each; calls are serial within each test, while test scheduling may run them concurrently. This is an intended timeout budget conditional on process behavior, not a hard total wall-time guarantee. The 2026-10-29 complete harness passed ten specifications, 39 axiom reports and ten rejected controls.

### 2026-11-04 · document

#### One-file groupants (2026-11-04)

Two children join the infrastructure group. `modelcheck-sections` is the section grammar shared by `modelcheck-library` (component sections, `file#Section` references, section-scoped hashes), `modelcheck-consolidate` and `mathematical-correspondence` (catalogue model bindings): one definition of section bytes, so a binding computed by consolidation is the binding the validators recompute. `modelcheck-consolidate` is a migration tool reached through `modelcheck-cli`; it writes only when invoked without `--dry-run`, never deletes, and its `consolidate-check` reverse is independent of the forward rewrite map (it reads the former files from Git). Coupling assumption: the validators and the correspondence scanner must accept every reference form the migration writes; the integration gate is `groupants` plus `mova-correspondence scan` on the migrated tree.

### 2026-11-05 · document

#### JSON maintenance tool (2026-11-05)

The `mova-json-tool` subtree edits, merges and rebinds the JSON files the other children validate. Its writes keep every unedited byte when the file's layout is reproducible and refuse otherwise, so a registry or manifest edit made with it changes only the edited values. Coupling assumption: hashes it refreshes (rebind, settled merge conflicts, rebind-groupant) are bindings, not re-verification; `groupants` and `changed` remain the gates, and TEXT CHANGED fragments need a model review.

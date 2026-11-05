# world-groupant-joins

## model

# Supplied-input API joins

## forward

The harness constructs six finite input families, calls actual public APIs through adapters, and asserts their joint observations. Its observation is the six named pass/fail results plus assertion diagnostics on failure. A successful test means these particular inputs agree with the declared oracle. It does not establish universally correct wiring or runtime behavior outside the fixture domain.

The numerical join preserves candidate operand order and grouping when emitting exact quantity scripts. The state join preserves entity identity, version and sentence anchor through parsing, execution and Markdown projection. The coverage join preserves question multiplicity through lexical matching. The book join uses selected body text, not wrapper metadata, as gate corpus. The lexical join uses actual animacy output as a dependency for QUD eligibility. The CSV join uses identical serialized bytes in two readers before table projection.

## invariant

Fixture loops are bounded: 64 triples in 0..3, 512 tree/multiset combinations, 25 two-field CSV records, four lexical cases, eleven book-prefix lengths and one five-command world program. Exact numerical results use rational q(candidate.value). Candidate index tuples retain distinct indices; commutative operations canonicalize first-pair order. A rejected world command must not erase accepted history. Coverage counts query occurrences, while qlemmas is compared as a set. CSV empty cells become unknown in attributes. These are scoped assertions, not global API contracts.

## cost

Let C be total generated numerical candidates over the 64 triples and P the cost of parsing/running one short quantity script. Candidate checks require O(C log C + C*P) ordered-set and interpreter work, plus the bounded independent enumeration. The code asserts C>1000 without publishing an exact total. Coverage performs 512 helper pairs over at most nine query occurrences and three supplied words. CSV performs 25 writes and paired reads, with input-byte parsing and map costs. World and lexical checks are fixed small fixtures. External dictionary initialization, allocator/filesystem latency and called-library costs are not benchmarked. No performance or approximation-error guarantee follows.

## composition

Each join constrains shared data that isolated leaf tests could miss: operand orientation, source-body boundaries, version IDs, token multiplicity, animacy classification and serialized CSV bytes. Relations compose only while these shared values agree. Child complement unions cannot describe the parent complement; lost intermediate values create ambiguous inverse candidates. No executable approximant is attached because test pass/fail has no useful tolerated-error metric here.

Semantic dependencies include opchoice, quant, lang, world, md, qframe, tree, reader, events, bookans, sense, qud, tabular, math and en. ftqa::csv is still an unregistered external dependency: calling it here does not model or cover its source file. Native dictionary use is not live annotation. Supplied words and frames bypass parser acquisition, so parser quality is unknown.

## epoch-memory

The initial integrated debug run passed all six tests. An operand-reversing adapter mutation compiled and produced a quantity division-by-zero error, falsifying the adapter's order-preservation claim. The runner checked named panic, diagnostic and exit 101, then restored source bytes. Initial evidence is finite, and repeated final debug/release profiles belong to the integrating parent task. This sidecar does not preclaim those later results.

## checks

Run cargo test -p world --test groupant_joins -- --test-threads=1; release uses --release before -p. Supplied fixture domains and named assertions are described in the ten views. See model.md#evidence for observed initial execution, control and source hashes. Two source syntax generations are byte-identical. Structural validation checks references and source identity, not test correctness or dependency completeness.

## behavior-complement

Universe U pairs the six admissible fixture families with observations of their joined APIs. R is the conjunction of the scoped assertions below. The inner exclusion hypothesis contains pairs with wrong rational result, leaked rejected state, wrong multiset coverage, wrapper-based copy rejection, QUD attachment inconsistent with actual sense output, or unequal CSV decoding. Each is excluded under the stated source-derived wiring model; finite success is evidence, not a proof of U minus actual implementation behavior. Out-of-domain errors and external semantics remain unknown.

## reverse-reconstruction

Given all six pass observations, retain every implementation/dependency state that satisfies the fixture relations. A pass vector cannot uniquely recover intermediate candidates, entity histories, original corpora or lexical semantics. Given an individual numeric observation, retain candidate index/operator tuples and all triples producing it. Equal values do not identify which equal-valued mention was used. Reconstruction is a relation, not an implemented complete solver.

## quantity-join

Type: encoding_relation. Domain: triples (a,b,c) in {0,1,2,3}^3. Independent i128 arithmetic admits nonnegative subtraction and exact division with nonzero divisor, then converts to i64. Candidate tuples are compared as sets and vector length forbids duplicates. Encoding is E(x op y)=(E(x) symbol E(y)); an optional second operation wraps the first expression on the left. Parse/run must yield no errors, answer q(value), and empty state. Evidence: operation_candidates_join_exact_quantity_scripts, 64 triples and more than 1000 interpreted candidates. This validates bounded integer-to-rational wiring, not unrestricted arithmetic overflow or expression parsing.

## history-render-join

Type: state_projection. Domain: fixed five-command program creates Hall, Garden and Alice, moves Alice to Garden, then attempts missing L9. Four commands accepted, one rejected. Snapshot version 1 locates Alice in L1; latest locates her in L2. Old rendering contains Hall without Garden, latest contains Garden. Combined Markdown contains acceptance counts, all three anchors and title; uncovered set empty. Evidence: parsed_world_history_joins_markdown_observations. No general claim about every world gate or arbitrary Markdown escaping.

## coverage-join

Type: multiplicity_relation. Domain: three invented labels, each present/absent in supplied Tree; question counts each 0..3. For total n and present-token occurrence count h, cover=0 if n=0 else h/n. qlemmas projected to a set equals labels with positive count. The 8*64=512 cases distinguish occurrence weighting from accidental deduplication. Evidence: question_frame_reader_join_preserves_multiset_coverage. Dictionary fallback behavior for arbitrary natural words remains outside this oracle.

## book-gate-join

Type: boundary_relation. Domain: artificial Gutenberg-marked text with ten-word body and distinct nine-word prefix/suffix. book_body must retain body and remove wrappers. Gate built from that body rejects literal prefixes precisely when length>=COPY_RUN for lengths 0..10; prefix/suffix metadata must pass. Four-word unrelated quotation must still reject. Evidence: selected_book_body_is_the_copy_gate_input. It checks body-to-gate wiring and the current threshold, not collision freedom, legal compliance or all marker spellings.

## sense-qud-join

Type: dependency_relation. Domain: supplied Who-runs question and subject answer for person, dog, stone and zzunclassified. Let A be the actual sense::animacy result classified Animate. Then QUD step 1 explicitly attaches to question 1 iff A, and stack emptiness iff A; A additionally implies Complete. Evidence: sense_animacy_joins_who_question_eligibility. This oracle deliberately depends on the same semantic service: it checks wiring, not independent animacy truth or classification quality. Nonanimate answer status is not otherwise constrained.

## csv-join

Type: encoding_relation. Domain: ordered two-cell products of empty, plain, comma-containing, quote-containing and accented strings: 25 records. csv_field quotes cells and doubles embedded quotes. tabular::read_csv and ftqa::csv receive the same single-line CSV bytes and must agree on header/rows and original cell strings. read_world maps empty attributes to unknown, preserves others and yields no links. Evidence: tabular_and_fairytale_csv_agree_on_single_line_domain. Multiline fields and malformed CSV are excluded; ftqa source remains unmodeled.

## adapter-negative

Type: counterfactual_relation. Change only expression adapter output to reverse operands while keeping candidate enumeration and oracle fixed. Addition/multiplication alone cannot detect this, but subtraction/division can. Observed control reaches (1 / (0 + 0)), where quant reports division by zero and no ask result; operation_candidates_join_exact_quantity_scripts fails its run.errors.is_empty assertion. Compiled assertion failure establishes sensitivity to this wiring defect, not all possible adapter defects. Restored source hash is bound in evidence.

## effect-boundary

Type: effect_relation. Scratch allocates a unique temporary directory using process ID and atomic serial, writes one CSV path repeatedly, and attempts recursive removal on Drop. Removal errors are ignored, so cleanup success is not guaranteed. Create/read/write failures panic through unwrap and are outside successful semantic observations. Other joins use in-memory fixtures and compiled lexical dependencies. No teacher/network/parser acquisition is exercised. A failure or pass cannot identify ambient filesystem state or certify absence of all library initialization effects.

## evidence

### Join evidence

`join-control.log` records that the reversed-operand adapter compiled, failed the quantity error assertion and was restored. `join-mutant.log` supplies the failed named test and division-by-zero diagnostic for `(1 / (0 + 0))`; the integrating runner checks exit 101, named panic and diagnostic. Source hash above binds the restored adapter. This is a compiled adapter control, not a mutation of opchoice or quant production code.

At sidecar authoring, final repeated profiles were pending. The integrated result below supersedes that pending state. No duplicate test execution was needed for sidecar authoring. Isolated structural validation uses exact copied source/sidecar and strict one-node registry; whole-project parent bindings belong to integration.

No live parser, teacher, network or training. QUD agreement uses actual sense dependency rather than independent animacy truth. ftqa::csv is invoked on a single-line domain but remains unregistered/unmodeled. Ten typed views provide scoped observations and exclusions, not comprehensive correctness claims.

## History

### 2026-10-30 · document

Source-bound harness model, 2026-10-30. The ten typed views below concern this harness and the supplied finite fixtures, not comprehensive models of each called source. All use the manifest source revision, the invariant below, and the evidence in model.md#evidence. Ordinary test function bodies participate in the partial syntax projection; macro token bodies such as assert_eq! remain opaque. No parser or teacher is acquired.

### 2026-10-30 · evidence

Heading `### Join evidence — 2026-10-30` stays in evidence as `### Join evidence`.

Source SHA-256: `896934e050ceebedaa2af43785ddfd0f5643c194b30e15e9e939c6a7fc3251fc`. Partial syntax SHA-256: `79dcf188512740a3f410d4dc70c304c667f0f4743c2178d97499449dfc2c6581`. Two syntax generations match byte-for-byte. Ordinary test function bodies participate in this projection; assertion macro token bodies remain opaque. The prose model was read against actual source.

Observed integrating-parent log `octopus-wave8-integration-2026-10-30/joins-initial.log`: six tests passed in debug, none failed or ignored. Exact test identities are listed by their relation sections in model.md. Domains: 64 numeric triples, 512 coverage cases, 25 CSV records, four lexical cases, eleven prefix lengths and one five-command world. Source asserts more than 1000 interpreted numerical candidates; no exact count claimed.

#### Integrated native evidence, 2026-10-30

The integrated checkout passes 138 focused native tests twice in debug and twice in release, with identical normalized test identities/results. This includes 61 new leaf tests, six new actual-API joins and 71 selected existing regressions. The six joins all pass after the reversed-operand control was restored. Leaf source controls, the original defective branches and the late-invalid-index-filter mutation fail the named assertions in their respective evidence. All registered child identities and transitive hashes are reconciled in the final manifests. These checks do not execute a full reader/parser, teacher, school corpus or GPU benchmark, and do not constitute a whole-world lint result.

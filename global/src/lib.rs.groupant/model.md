# global-catalog

## descriptant

### global-catalog

Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

#### Gaps

AST is not materialized. Generated source and external data are explicit coverage gaps. Existing tests are bounded examples; optional expression and idiom tests can return early. Proposed oracles are not executed. No production implementation changes are included.

## catalog

# Typed catalog

All relations hold with fixed compiled tables and optional initialized hot snapshot. They are source-derived specifications with unproved correspondence, not executed inverse programs. U\R is the complement target; a claimed restriction is conditional on its listed assumptions. Each reverse relation retains all compatible antecedents. Checks below are evidence references or explicitly prospective oracles.

## nonbehavior

Type: behavior_complement. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

For fixed generated tables T and initialized hot snapshot H, let Q be tagged calls to the scoped APIs and O their typed return values; U=Q x O, R={(q,F_T,H(q))}. B=U\R. This is the exact target of the source-derived specification, not a proven complement of all Rust executions. Valid path indices and nonoverflowing arithmetic are assumptions; panics, IO initialization and allocation failure are outside U.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed oracle: compare independently enumerated typed outputs for small catalogs; reject wrong traversal and tie policies.

## antecedents

Type: reverse_reconstruction. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

F^-1(y)={q in Q:F_T,H(q)=y}, retaining API tags, all compatible strings and report states. Empty lookup results conflate missing names, no cue matches, filtered glosses and absent optional tables, so no unique original text or seed artifact is recoverable.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed oracle: enumerate a bounded query domain and compare every fiber with direct evaluation.

## lexical-forms

Type: lexical_acceptance. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

has_word lowercases and replaces each non-alphanumeric non-space character with a space, pads the text, then searches space-delimited forms of the unchanged input lemma. Forms include s/es/ed/d/ing, doubled final consonants excluding aeiouwy, y->ies/ied and eight explicit irregular families. Matching is neither general lemmatization nor whitespace collapsing; an uppercase lemma need not match.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: tests::values_from_fables; proposed punctuation, Unicode and uppercase-lemma cases.

## sense-fiber

Type: reverse_reconstruction. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

For fixed (lang,word), I is table-ordered matching concepts and score_i(t) counts stored cues unequal to word and not starting with word, using underscores replaced by spaces. Winner i requires score_i>score_j for earlier j and score_i>=score_j for later j. All texts satisfying inequalities remain antecedents, including zero-evidence ties.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: sense_tests::codes_and_senses; proposed equal-score and all-zero fixtures.

## shortest-route

Type: reachability. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

For valid source index a and valid link endpoints, path(a,b) records first-discovery incoming links in FIFO BFS scanning LINKS in order. Some([]) iff a=b; an unreachable b returns None (including b outside the concept index domain). Among multiple shortest paths only the first discovery witness is returned; arbitrary alternate routes are not outputs.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: tests::newton_apple and tests::verbs_and_units; proposed cyclic/diamond graph reference.

## chain-horizon

Type: bounded_trace. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

consequences discards category triggers, then discards any found c reachable from another distinct found o. Each survivor follows the first outgoing link at most eight times; emit only nonempty link chains. It does not enumerate all consequences. Mutually reachable found concepts can suppress every chain. self_describe similarly follows first links for at most four steps.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: tests::newton_apple; proposed branching and strongly connected fixtures.

## judgment-triggers

Type: ordered_emission. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

judge emits one judgment for each matching against cue then each matching towards cue within each principle in table order. Duplicate triggers/principles can yield repeated or opposing observations. Because includes cue, name, gist and file; no story-state predicate is evaluated here.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: tests::values_from_fables; proposed a text matching both directions.

## self-report-gates

Type: predicate_partition. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

self_judge emits steady_effort success for branches>=5, else failure for branches<=1&&!verified; verified adds self_quality. Escalated confidence<0.5 adds restraint, else !escalated&&confidence<0.2&&correct==Some(false) adds violation. Slow time/actions (>3 usual, usual positive) with correct!=Some(false) adds effort; rethought adds effort. Each event survives only if principle_by_rule finds a principle. NaN comparisons are false and action multiplication must not overflow.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: No existing self_judge test in this file; proposed Boolean/threshold oracle including NaN and duplicate-rule first selection.

## hot-overlay

Type: snapshot_precedence. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

First hot() captures GLOBAL_HOT files once. Read errors become empty text; valid u8 scores clamp to four, duplicate keys overwrite earlier ones, unknown idiom kinds become tombstone none. absurdity checks hot real then sorted cold indexes. tale first checks hot tale, then requires cold verb/noun IDs before a cold tale lookup or real fallback: missing cold IDs return None before hot-real fallback. This source-specific boundary must remain visible.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: absurd_tests::matrix_cells; fresh-process hot override and missing-cold-ID tests remain proposed.

## expression-choice

Type: greedy_selection. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

At lengths six down to one and starts ascending, expressions_in rejects used spans; tries literal phrase then one-position possessive/interior placeholder candidates. First admissible row per candidate wins. Multiword matches reserve spans; single words do not and require nonempty derived domains. Rows need >=3 alphabetic gloss characters and no [[. Forms supplement lemma results only for multiword spans not overlapping a lemma or earlier form multiword.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: expression_tests::real_meanings_and_domains is conditional on optional table presence; prospective synthetic row oracle.

## lookup-aliases

Type: information_loss. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

concept matches name OR code; concept_words matches name only. measure lowercases, strips es only when its base exists, otherwise strips s, then selects first exact-or-singular row. relation and selection use first exact matches; uses/capable/category/state queries perform exact table membership. Equal empty slices do not establish equal input names.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: tests::uses_and_capable and tests::selection_lists; proposed code-versus-name and plural collision cases.

## resource-envelope

Type: resource_bound. Observation: Compiled seed catalogs and scoped runtime catalog queries; not a durable global knowledge graph.

With C concepts, L links, text bytes N and K cue occurrences, lexical scanning is O(K*N) up to form allocation/string costs; path O(C+reached_nodes*L) time and O(C) storage. Consequences can call path for O(C^2) found pairs before bounded chain scans. Expressions have at most six span lengths and O(6) candidates per span, BTreeMap lookup O(log E) comparisons plus string work and duplicate-row scans. Lazy index construction scales with included bytes; no latency measurement or whole-engine bound is implied.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Symbolic source inspection; no performance experiment claimed.

## tests

### Evidence and checks

#### Existing checks

Run from repository root: cargo test -p global --lib --locked, with GLOBAL_HOT and GLOBAL_EXCLUDE unset, offline Cargo, two build jobs and a 180-second process bound. See the root sidecar model.md#evidence for the actual result.

The complete lib tests exercise seed chains, verb classes, uses/capabilities, selection, lexical judgments, senses, absurdity and intransitivity. placement tests cover the separate placement module. Idiom and expression tests can return early for missing optional data, so success does not establish those content-dependent assertions ran.

#### Proposed checks

Catalog oracle entries explicitly marked proposed have not run. Negative-control assertions in existing tests are examples, not exhaustive complement or reconstruction correspondence. No new production tests, training, network calls, GPU run or formal proof are claimed.

## evidence

### Evidence

#### Executed checks

The bounded CPU command `cargo test -p global --lib --locked` passed: 13 tests, zero failures, zero ignored. GLOBAL_HOT and GLOBAL_EXCLUDE were unset; Cargo was offline with two build jobs and a 180-second process timeout. Compilation completed in 4.47 seconds; the test runner reported 0.22 seconds. These timings describe this run, not a performance claim. Three tests belong to the separately modeled placement module. Both optional internal expression and idiom source tables were present and nonempty in this worktree; their content-dependent tests passed. Their contents are neither copied nor bound by this sidecar.

The modelcheck executable built offline with the same job bound. Two local groupant inventories passed without diagnostics and were byte-identical after temporarily omitting this new file's pending registry declaration; the exact original manifest was restored afterward. The final declaration, checked against the unchanged registry, has exactly one diagnostic: registry node does not match source ownership. The parent integrator must add registry-node.json under knowledge-memory and update ancestor source bindings before full validation can pass. No registry or group sidecar edits are included here.

#### Source binding and tree

The root global-catalog has 12 typed models. concept-path has four models and its sense-selection child has four. expression-catalog has four models and its matching child has four. Total: five groupants, four fragments at two levels, 28 typed models. Each family begins with behavior_complement and reverse_reconstruction. Parents have explicit composition artifacts.

Source ranges: concept-path [1683,7107), sense-selection [2907,4110), expression-catalog [23112,31515), matching [25733,30279). Fragment ranges are enclosing scopes, not a disjoint or complete source partition.

#### Limits and next evidence

No AST was materialized. The OUT_DIR generated Rust and expression TSV, direct register TSV includes, build script, seeds and hot snapshot are semantic dependencies outside the lib.rs source fingerprint. Source review found that tale lookup can return None before reaching hot-real fallback when cold IDs are absent; the model records this behavior without changing it. No production bug fix is claimed.

Existing tests supply bounded positive and negative examples. All newly proposed model-specific oracles remain unexecuted; no exhaustive complement/preimage proof, mutant test, training, live service, GPU run or durable global graph implementation is claimed. Full integrated changed/groupants checks remain the parent's responsibility after registration.

## core-model

# Scoped model

## Forward

Dispatch a tagged catalog query over build-time tables, lazy expression/register indexes and the first hot snapshot. The catalog specifies lexical lookup, sense choice, traversal, judgments and expression choice. The file declares placement as an out-of-line module; placement policy remains separately owned. Other lookup functions are reviewed but not exhaustively modeled.

## Invariants

Catalog order determines ties and first-match decisions. Seed file provenance exists in concept/link/principle records, but not every result retains it. No routine here records durable learned claims. Hot and lazy indexes are snapshots.

## Cost

See resource-envelope in model.md#catalog: full scans and lazy setup dominate; all bounds are symbolic, no timings are claimed.

## Composition

concept-path models lexical triggers and graph traversal; its sense-selection child supplies cue scoring. expression-catalog models rows and labels; its matching child consumes that index. Root additionally supplies judgments, hot overlays and other lookup behavior. Shared compiled tables and ordering wire the branches; neither child correctness nor sidecar existence proves generated data validity, full call-graph coverage or durable graph behavior.

## Boundaries

Source identity binds lib.rs only. Generated OUT_DIR/global.rs and OUT_DIR/expressions.tsv, seed contents, build.rs, direct register TSV includes and hot files are semantic dependencies not fingerprinted by this source hash. No expanded AST, durable global graph, truth inference or general semantic proof is supplied. Environment and generated tables must remain fixed for the relations.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Reviewed the complete lib.rs and build.rs plus package dependencies and existing tests. The crate has no runtime Cargo dependencies. Build-time sorted concepts and declared link order feed deterministic queries; optional internal expression data and first-use hot configuration affect actual observations. Preserve the distinction between BFS and bounded first-edge chains, strict sense tie updates, and asymmetric form/lemma merging. These catalogs record conditional models, not new verified capabilities.

### 2026-10-29 · evidence

Heading `### Evidence 2026-10-29` stays in evidence as `### Evidence`.

Rust SHA-256 over the actual lib.rs bytes is `0d9cf581b414d7cfa75ca8344e51d4ae72bd7363ff13fb15b658146d40e1f807`. A Rust helper computed all UTF-8 half-open source ranges and serialized manifests. Structural validation checked their ancestry and artifact references.

### 2026-11-04 · descriptant

#### Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

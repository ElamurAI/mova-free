# global-catalog-concept-path

## descriptant

### global-catalog-concept-path

Lexical concept discovery, sense selection, measures, code parts, shortest paths and consequence chains.

#### Gaps

AST is not materialized. Generated source and external data are explicit coverage gaps. Existing tests are bounded examples; optional expression and idiom tests can return early. Proposed oracles are not executed. No production implementation changes are included.

## catalog

# Typed catalog

All relations hold with fixed compiled tables and optional initialized hot snapshot. They are source-derived specifications with unproved correspondence, not executed inverse programs. U\R is the complement target; a claimed restriction is conditional on its listed assumptions. Each reverse relation retains all compatible antecedents. Checks below are evidence references or explicitly prospective oracles.

## nonbehavior

Type: behavior_complement. Observation: Lexical concept discovery, sense selection, measures, code parts, shortest paths and consequence chains.

For fixed concepts/links, U consists of valid lexical/path requests paired with typed results; R follows has_word, sense selection and BFS/chain rules in this range. U\R excludes a longer-than-shortest BFS witness, a category-start consequence and a >8-link consequence. These are sound intended restrictions of the specification; actual correspondence is unproved.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: tests::newton_apple; proposed BFS reference.

## antecedents

Type: reverse_reconstruction. Observation: Lexical concept discovery, sense selection, measures, code parts, shortest paths and consequence chains.

For observation (trigger indices, emitted chains), retain every text whose has_word cue incidence produces those indices and surviving starts. Normalize-equivalent texts and distinct cues can share outputs; emitted chains alone do not identify every suppressed trigger.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed finite text/cue incidence fiber enumeration.

## route-versus-chain

Type: policy_difference. Observation: Lexical concept discovery, sense selection, measures, code parts, shortest paths and consequence chains.

BFS chooses shortest target-directed paths by first discovery; consequences chooses a first outgoing edge without a target for <=8 steps. Equality holds only under additional chain topology and horizon constraints. A branch or cycle can separate them.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: tests::newton_apple supplies one agreeing chain; branching/cycle oracle proposed.

## suppressed-starts

Type: behavior_complement. Observation: Lexical concept discovery, sense selection, measures, code parts, shortest paths and consequence chains.

For found noncategory set S, starts={c in S: no o in S\{c} reaches c}. No member of a mutually reachable pair in S can start a consequence. Reachability does not preserve a representative of an SCC; predicting one is excluded by the specification.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed two-node cycle and one-way two-trigger cases.

## tests

### Evidence and checks

#### Existing checks

Run from repository root: cargo test -p global --lib --locked, with GLOBAL_HOT and GLOBAL_EXCLUDE unset, offline Cargo, two build jobs and a 180-second process bound. See the root sidecar ../model.md#evidence for the actual result.

The complete lib tests exercise seed chains, verb classes, uses/capabilities, selection, lexical judgments, senses, absurdity and intransitivity. placement tests cover the separate placement module. Idiom and expression tests can return early for missing optional data, so success does not establish those content-dependent assertions ran.

#### Proposed checks

Catalog oracle entries explicitly marked proposed have not run. Negative-control assertions in existing tests are examples, not exhaustive complement or reconstruction correspondence. No new production tests, training, network calls, GPU run or formal proof are claimed.

## core-model

# Scoped model

## Forward

Normalize text with has_word; select table-ordered concepts and cue-scored senses. BFS yields one shortest path; consequences removes categories and non-starts before following the first outgoing link up to eight steps.

## Invariants

Path edges connect consecutively and shortest distance is nonnegative. Chain iteration is bounded even with cycles; empty paths and absent paths differ. Found concepts are in table order.

## Cost

For C concepts, L links, N text bytes, K cues: lexical detection O(K*N) up to string work; BFS O(C+reached*L) time/O(C) space. Start suppression can make O(C^2) BFS calls.

## Composition

sense-selection supplies concept/sense selection inside this range; has_word is shared with broader trigger detection. concept indices flow into path and consequences. The child does not imply that all graph endpoints or seeds are valid; those rely on build.rs.

## Boundaries

Source identity binds lib.rs only. Generated OUT_DIR/global.rs and OUT_DIR/expressions.tsv, seed contents, build.rs, direct register TSV includes and hot files are semantic dependencies not fingerprinted by this source hash. No expanded AST, durable global graph, truth inference or general semantic proof is supplied. Environment and generated tables must remain fixed for the relations.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Reviewed the complete lib.rs and build.rs plus package dependencies and existing tests. The crate has no runtime Cargo dependencies. Build-time sorted concepts and declared link order feed deterministic queries; optional internal expression data and first-use hot configuration affect actual observations. Preserve the distinction between BFS and bounded first-edge chains, strict sense tie updates, and asymmetric form/lemma merging. These catalogs record conditional models, not new verified capabilities.

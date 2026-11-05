# global-catalog-expression-catalog

## descriptant

### global-catalog-expression-catalog

Expression rows, lazy exact-phrase index, register mapping and greedy sentence matching.

#### Gaps

AST is not materialized. Generated source and external data are explicit coverage gaps. Existing tests are bounded examples; optional expression and idiom tests can return early. Proposed oracles are not executed. No production implementation changes are included.

## catalog

# Typed catalog

All relations hold with fixed compiled tables and optional initialized hot snapshot. They are source-derived specifications with unproved correspondence, not executed inverse programs. U\R is the complement target; a claimed restriction is conditional on its listed assumptions. Each reverse relation retains all compatible antecedents. Checks below are evidence references or explicitly prospective oracles.

## nonbehavior

Type: behavior_complement. Observation: Expression rows, lazy exact-phrase index, register mapping and greedy sentence matching.

With fixed included TSV bytes, U is the tagged union of direct phrase lookups returning expression slices and token requests returning ExprMatch records; R is lazy row parsing and greedy matching. An inner approximant C to U minus R contains parsed rows originating from fewer than four columns. For ExprMatch outputs it additionally contains spans longer than six words, unresolved [[ glosses and unlabeled singleton readings. These matcher restrictions do not apply to direct expression() lookup slices, which return indexed rows. This is a scoped specification, not a certified runtime complement.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: expression_tests::real_meanings_and_domains; proposed malformed-row fixture.

## antecedents

Type: reverse_reconstruction. Observation: Expression rows, lazy exact-phrase index, register mapping and greedy sentence matching.

An expression match (span,entry,domains) admits every token sequence generating its literal or one-position generalized candidate and surviving longer/earlier matches. Observation loses original capitalization, which candidate variant matched, unchosen rows and unused extra TSV columns; retain these alternatives.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed finite tokens/rows fiber enumeration.

## row-index

Type: ordered_multimap. Observation: Expression rows, lazy exact-phrase index, register mapping and greedy sentence matching.

For each non-comment line with >=4 tab columns, append one row under column zero; register/variety default empty. Duplicate phrases preserve file row order. Exact expression lookup does no lowercasing itself; no caller input normalization is inferred.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed duplicate-phrase and case-sensitive direct-lookup fixture.

## label-projection

Type: lossy_mapping. Observation: Expression rows, lazy exact-phrase index, register mapping and greedy sentence matching.

Register labels affect domains only when their attached gloss equals the row meaning. Known register groups map to slang/formal/humor/hidden-meaning. Variety labels matching the main meaning map to fixed recognized names, with every other spelling mapping to variety:Wales. Sorted deduplication forgets label multiplicity and order.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed mismatched-gloss and unknown-variety fixtures.

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

Build a phrase-to-ordered-rows index once from included TSV; exact lookup feeds greedy matching. Labels whose gloss equals the main meaning map to sorted distinct domains.

## Invariants

Rows with fewer than four fields are absent; duplicate phrase rows retain order. Optional missing expression data yields an empty index, not evidence that English has no expressions.

## Cost

Initial BTreeMap build scales with TSV bytes and logarithmic key insertion comparisons; storage scales with rows and phrase keys. Subsequent phrase lookup is O(log E) string comparisons.

## Composition

matching consumes exact expression lookup and register_domain from the parent. Lazy row order controls first accepted readings, and label projection controls singleton eligibility. Parent data loss and child greedy suppression compose; reverse reconstruction must preserve both.

## Boundaries

Source identity binds lib.rs only. Generated OUT_DIR/global.rs and OUT_DIR/expressions.tsv, seed contents, build.rs, direct register TSV includes and hot files are semantic dependencies not fingerprinted by this source hash. No expanded AST, durable global graph, truth inference or general semantic proof is supplied. Environment and generated tables must remain fixed for the relations.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Reviewed the complete lib.rs and build.rs plus package dependencies and existing tests. The crate has no runtime Cargo dependencies. Build-time sorted concepts and declared link order feed deterministic queries; optional internal expression data and first-use hot configuration affect actual observations. Preserve the distinction between BFS and bounded first-edge chains, strict sense tie updates, and asymmetric form/lemma merging. These catalogs record conditional models, not new verified capabilities.

# global-catalog-expression-matching

## descriptant

### global-catalog-expression-matching

Forms/lemma merge and bounded greedy matcher, including candidate and span policy.

#### Gaps

AST is not materialized. Generated source and external data are explicit coverage gaps. Existing tests are bounded examples; optional expression and idiom tests can return early. Proposed oracles are not executed. No production implementation changes are included.

## catalog

# Typed catalog

All relations hold with fixed compiled tables and optional initialized hot snapshot. They are source-derived specifications with unproved correspondence, not executed inverse programs. U\R is the complement target; a claimed restriction is conditional on its listed assumptions. Each reverse relation retains all compatible antecedents. Checks below are evidence references or explicitly prospective oracles.

## nonbehavior

Type: behavior_complement. Observation: Forms/lemma merge and bounded greedy matcher, including candidate and span policy.

U pairs finite token vectors with lists of matches. R runs descending lengths <=6, ascending starts, candidate order and accepted-row filtering. U\R excludes overlapping chosen multiword spans in one pass and any singleton without derived domains; sentence merge rejects a form multiword overlapping an existing multiword, but can overlap existing singletons.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Conditional expression test; proposed independent span-policy oracle.

## antecedents

Type: reverse_reconstruction. Observation: Forms/lemma merge and bounded greedy matcher, including candidate and span policy.

Given the final sorted matches, recover all forms and lemmas whose separate greedy results satisfy lemma priority plus form admission. Match records do not record origin. Multiple origin assignments, discarded candidates and capitalization variants remain possible; no unique token sequence is asserted.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed exhaustive tiny forms/lemma domains.

## one-position-generalization

Type: candidate_language. Observation: Forms/lemma merge and bounded greedy matcher, including candidate and span policy.

Candidate set is literal plus individual possessive->one's replacements; for nonpossessive interior positions of spans>=3 add someone/something/somebody substitutions. No candidate substitutes two positions at once, and placeholders are never generated at edges. Duplicate candidate strings can produce repeated singleton observations because singleton matches do not mark used.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed two-placeholder and boundary cases.

## merge-priority

Type: noncommutativity. Observation: Forms/lemma merge and bounded greedy matcher, including candidate and span policy.

M(forms,lemmas)=sort(G(lemmas) ++ admitted multiwords from G(forms)). Admission sees already appended form matches and ignores singletons for overlap. Swapping forms and lemmas can alter chosen spans and entries; this merge is not a symmetric union or global maximum-length optimizer.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: expression_tests::real_meanings_and_domains checks one forms rescue; asymmetric overlap cases proposed.

## tests

### Evidence and checks

#### Existing checks

Run from repository root: cargo test -p global --lib --locked, with GLOBAL_HOT and GLOBAL_EXCLUDE unset, offline Cargo, two build jobs and a 180-second process bound. See the root sidecar ../../model.md#evidence for the actual result.

The complete lib tests exercise seed chains, verb classes, uses/capabilities, selection, lexical judgments, senses, absurdity and intransitivity. placement tests cover the separate placement module. Idiom and expression tests can return early for missing optional data, so success does not establish those content-dependent assertions ran.

#### Proposed checks

Catalog oracle entries explicitly marked proposed have not run. Negative-control assertions in existing tests are examples, not exhaustive complement or reconstruction correspondence. No new production tests, training, network calls, GPU run or formal proof are claimed.

## core-model

# Scoped model

## Forward

Compute greedy lemma matches, then append form multiwords that overlap no existing multiword; sort by start then decreasing end. The inner matcher uses lengths six down to one and first eligible candidates/rows.

## Invariants

Every span is within its input token sequence and length <=6. Each pass has disjoint multiword spans, but singletons may overlap a form-derived multiword after merging. Domains are sorted and deduplicated.

## Cost

Six fixed lengths, ascending starts, O(length) candidate alternatives, BTreeMap lookups and duplicate-row scanning. Merge overlap checks can be quadratic in match counts; sort is O(M log M).

## Composition

Leaf fragment; no child composition is declared.

## Boundaries

Source identity binds lib.rs only. Generated OUT_DIR/global.rs and OUT_DIR/expressions.tsv, seed contents, build.rs, direct register TSV includes and hot files are semantic dependencies not fingerprinted by this source hash. No expanded AST, durable global graph, truth inference or general semantic proof is supplied. Environment and generated tables must remain fixed for the relations.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Reviewed the complete lib.rs and build.rs plus package dependencies and existing tests. The crate has no runtime Cargo dependencies. Build-time sorted concepts and declared link order feed deterministic queries; optional internal expression data and first-use hot configuration affect actual observations. Preserve the distinction between BFS and bounded first-edge chains, strict sense tie updates, and asymmetric form/lemma merging. These catalogs record conditional models, not new verified capabilities.

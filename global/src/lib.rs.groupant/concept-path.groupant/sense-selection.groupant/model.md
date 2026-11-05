# global-catalog-sense-selection

## descriptant

### global-catalog-sense-selection

Exact concept lookup, ordered senses and cue-scored sense selection.

#### Gaps

AST is not materialized. Generated source and external data are explicit coverage gaps. Existing tests are bounded examples; optional expression and idiom tests can return early. Proposed oracles are not executed. No production implementation changes are included.

## catalog

# Typed catalog

All relations hold with fixed compiled tables and optional initialized hot snapshot. They are source-derived specifications with unproved correspondence, not executed inverse programs. U\R is the complement target; a claimed restriction is conditional on its listed assumptions. Each reverse relation retains all compatible antecedents. Checks below are evidence references or explicitly prospective oracles.

## nonbehavior

Type: behavior_complement. Observation: Exact concept lookup, ordered senses and cue-scored sense selection.

U={(lang,word,text,result)} with result None or a catalog concept; R uses matching language/word and earliest maximum cue score. U\R excludes unrelated-language concepts, None for a nonempty sense set and a later tied maximum. Conditional exact specification target; no semantic disambiguation guarantee.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: sense_tests::codes_and_senses; proposed tied maximum oracle.

## antecedents

Type: reverse_reconstruction. Observation: Exact concept lookup, ordered senses and cue-scored sense selection.

For winning i, compatible texts satisfy every score inequality: strictly beat earlier candidates, tie or beat later ones. Preserve all strings meeting those inequalities; cues starting with the queried word never constrain a score. For None, sense set is empty regardless of text.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed bounded cue-power-set evaluation.

## prefix-exclusion

Type: sensitivity. Observation: Exact concept lookup, ordered senses and cue-scored sense selection.

A cue equal to word OR having word as prefix is excluded, even if linguistically a different word. Adding occurrences of an excluded cue changes no score unless it also matches another eligible cue; repeated occurrences of one eligible cue still contribute one per stored cue occurrence.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed prefix and duplicated-cue fixtures; existing tests do not isolate this.

## stable-tie

Type: order_dependence. Observation: Exact concept lookup, ordered senses and cue-scored sense selection.

best updates only on strict score increase. For all-zero scores the first catalog sense wins. Permuting equal-score candidate order can change the result; permutation invariance is therefore outside the relation.

Assumptions: fixed tables and snapshot; valid UTF-8 strings, valid graph indices where required, successful allocations, no arithmetic overflow. This is a conditional source specification, not a general semantic proof.

Evidence or falsification oracle: Proposed zero-cue and reordered-catalog reference; source-derived only.

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

Find first concept by name/code; collect all matching language/word indices; count eligible matching cues and choose earliest maximum. Empty candidate set returns None.

## Invariants

Winner belongs to matching language/word candidates. Later equal scores cannot replace an earlier winner. Cue membership does not count textual frequency.

## Cost

O(C) candidate collection plus eligible-cue text matching; O(C) index storage in the worst case. Every has_word call allocates normalized text and forms.

## Composition

Leaf fragment; no child composition is declared.

## Boundaries

Source identity binds lib.rs only. Generated OUT_DIR/global.rs and OUT_DIR/expressions.tsv, seed contents, build.rs, direct register TSV includes and hot files are semantic dependencies not fingerprinted by this source hash. No expanded AST, durable global graph, truth inference or general semantic proof is supplied. Environment and generated tables must remain fixed for the relations.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Reviewed the complete lib.rs and build.rs plus package dependencies and existing tests. The crate has no runtime Cargo dependencies. Build-time sorted concepts and declared link order feed deterministic queries; optional internal expression data and first-use hot configuration affect actual observations. Preserve the distinction between BFS and bounded first-edge chains, strict sense tie updates, and asymmetric form/lemma merging. These catalogs record conditional models, not new verified capabilities.

# translation-flow-blocks

## descriptant

### translation-flow-blocks

make sums selected verse words per chapter, greedily joins contiguous same-book chapters while sum<=max and emits oversized chapter alone. make_rt transports chapters into roundtrip::blocks::group. text renders chapter/layout/verse lines for target keys. layout_code charges source/target key symmetric differences and target layouts differing from available source, then reconstructs precisely target-key-restricted layouts using stored overrides or source defaults.

## approximant

### Forward relation

make sums selected verse words per chapter, greedily joins contiguous same-book chapters while sum<=max and emits oversized chapter alone. make_rt transports chapters into roundtrip::blocks::group. text renders chapter/layout/verse lines for target keys. layout_code charges source/target key symmetric differences and target layouts differing from available source, then reconstructs precisely target-key-restricted layouts using stored overrides or source defaults.

Scope: source-derived conditional transition model at the manifest SHA. Valid book/chapter keys, representable word sums/chapter+1. Empty target produces empty reconstruction. Layout map absence means empty list. make and make_rt equivalence needs compatible grouping semantics and contiguous chapters; sparse-chapter equivalence not asserted.

## composition

### Composition

Scoped children are projections of this file, not independent executables. Main population filtering feeds aggregation; order/text helpers feed measure's exact decoder; many's agreement and storage read the same MTok with intentionally different punctuation treatment, streaming accounting consumes reader costs, clustering consumes normalized PairM. Uncovered branches remain source-derived; finite helper checks do not establish orchestration wiring.

## catalog

# Typed models

All entries bind the source SHA and optional byte range in manifest.json. Observation is one admitted helper call or the explicitly described orchestration trace. Relations are partial source-derived models, not universal proofs. Finite test names and unexecuted effects are in model.md#tests.

## behavior_complement

layout_code output never contains source-only keys or keys outside block. Changes inside a verse are not retained by read_layout's pending-before-verse model. make never combines different books or nonadjacent chapters.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## reverse_reconstruction

Given source layouts, target keys and stored overrides, target layout is determined; returned layout alone admits multiple source layouts and key differences. Rendered block text loses original USFM markup/comments and formatting; many files produce same pending layout.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## greedy_capacity

Chapter indivisibility permits word count>max for a singleton; max is a merge threshold, not an output size invariant.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## key_restriction

Symmetric difference cost sees in-block keys only, but pos_bits uses max total source/target set cardinality, including outside-block keys.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## literal_override

Different target layout, including empty vector deleting source marks, is stored; missing source defaults empty. No difference means source copied.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.

## pending_layout

Chapter marker clears pending; valid verse consumes pending; heading footnotes removed before clean; text-bearing continuation poetry marks dropped. read_layout source-derived only.

Domain and held-fixed assumptions: the invariant/forward documents for this exact scope. Status: partial source-derived relation; complement is a conditional inner exclusion, reverse retains ambiguity, other types specify the named projection/intervention. Oracle: relevant named synthetic tests when listed; otherwise source inspection only.


## tests

### Evidence and gaps

#### Native checks

- `blocks::wave32::layout_reconstruction_and_ranges`

#### Controls

Five compiled source-behavior mutants: main gate counter suppressed; many strict chapter comparison relaxed; measure noncanonical-gap recording disabled; pages Meta filter reversed; blocks literal-change predicate reversed. All failed the corresponding assertion and passed after original bytes/hash restoration. Zero assertion-only mutants; zero surviving exploratory mutants.

#### Limitations

CLI argument/effect paths, live corpus/model/annotation/training, full Run::verse, page filesystem and reports remain source-derived only. Source syntax projections generated twice and byte-compared. No corpus-quality, persistent-learning, throughput or universal semantic coverage claim.

## invariant-and-cost

### Domain, invariant and cost

#### Invariant

Valid book/chapter keys, representable word sums/chapter+1. Empty target produces empty reconstruction. Layout map absence means empty list. make and make_rt equivalence needs compatible grouping semantics and contiguous chapters; sparse-chapter equivalence not asserted.

#### Cost

Grouping O(Vlog C+text bytes) plus chapter scan; layout_code O((S+T)log(S+T)+literal bytes), retains changed target layouts; text scans corpus keys O(V) plus rendered bytes. read_layout files/footnote repeated removal may cost quadratic heading bytes; no live filesystem parsing tested.

Integer additions/casts require representability; finite floating inputs where comparisons are modeled. These symbolic costs are not measurements of corpus throughput. No useful cheaper executable percentage surrogate established; future supported caps only 1/2/5/10%.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective predictions saved before edits in the external wave32 tr_flow receipts. Original production bytes preserved; only appended synthetic test modules added. Selected actual helpers exercised; five compiled behavior controls rejected with exact byte restoration and positive reruns. Source syntax projection is partial, not expanded semantic Rust. Diagnostic reconstruction with residual is not semantic understanding or persistent learning.

Valid book/chapter keys, representable word sums/chapter+1. Empty target produces empty reconstruction. Layout map absence means empty list. make and make_rt equivalence needs compatible grouping semantics and contiguous chapters; sparse-chapter equivalence not asserted.

### 2026-10-31 · tests

These selected names ran twice debug and twice release with exact filtering and one test thread; raw logs and numeric status files are in the external octopus-wave32-tr_flow-2026-10-31 receipt directory. All test bodies inspected before selection. Inline cases use only synthetic in-memory records; the existing blocks filesystem test was not selected. Tests listed at file level do not execute every function or this entire scoped fragment.

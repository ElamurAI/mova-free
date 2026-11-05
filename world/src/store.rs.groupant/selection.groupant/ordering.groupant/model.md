# world-store-query-order

## descriptant

# ordering

## Forward

Selection folds terms left to right. Feature/book filters preserve order; sampling sorts selected IDs; prefix intersects current membership with global block-prefix order. Reverse composition preserves all compatible intermediate vectors.

## Invariant

Each stage only removes IDs. Stage order affects membership and cardinality. Prefix can change order and thereby change later positional sample ranks.

## Cost

One scan per filter plus stage-specific sampling/prefix cost. Overlaps candidate-coupling scope; views are not disjoint costs.

## Epoch memory

## catalog

# world-store-query-order

## nonbehavior

Type: behavior_complement.

Universe: valid finite query terms x ordered ID vectors, with fixed index and prefix order. Inner complement excludes execution that samples the final filtered candidates when sample is textually before that filter. Terms act left to right. Output observations alone may coincide; trace interpretation preserves the stage-order constraint.

## antecedents

Type: reverse_reconstruction.

For ordered stages T1 through Tm, preimages retain all intermediate vectors Cj with C0=0..n, Cj=Tj(Cj-1), Cm=Y, plus every fixed-input index satisfying those constraints. Reverse composition runs the converse relations in reverse stage order. It cannot replace sample and filter with an unordered conjunction.

## noncommutation

Type: operator_order.

A pointwise filter P preserves order. Sample S sorts selected IDs and depends on current candidates. In general S(P(C)) differs from P(S(C)); after-sample filtering may reduce cardinality below k. Prefix Q replaces candidate order with a global block-prefix intersection and thus can also change later positional sample ranks. Oracle: existing two-book source fixture and standalone six-ID ordering oracle enumerate 262144 old/new-mask, k, seed cases; legacy carry fails in 40928 feature-before-sample cases and zero sample-before-feature cases. The standalone model is not production execution; prefix-specific correspondence remains prospective.

## scope-of-conservatism

Type: policy_scope.

Sampling before all tree terms can be version-independent if metadata/corpus/order remain fixed, and subsequent pointwise changes are covered by C. The repaired policy may still reselect any stale sampled tree query: a conservative sufficient condition avoiding fragile reordering analysis. No general commutativity or minimal invalidation claim.

## History

### 2026-10-29 · Epoch memory

2026-10-29: A two-book fixture witnesses sample/filter noncommutation. Prefix-specific sampling correspondence remains prospective. Conservative invalidation avoids relying on a minimal reordering analysis.

### Undated · document

Source SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Conditional models over successful finite operations; I/O failure, malformed offsets and changing dependency state are excluded unless stated. Algebra is exact only for its declared abstract relation. Source correspondence remains bounded.

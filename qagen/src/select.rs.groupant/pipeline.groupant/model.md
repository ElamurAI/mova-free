# pipeline

## forward
Sum all paragraph sentence counts per book before filters. Eligible unskipped indices form dialogue/narration pools and caps. Apply quotas, assign d=min(floor(k/2),D), n=min(k-d,N), d=min(k-n,D), rank each pool with seeds seed and seed xor 0x5eed, then sort selected indices.

## behavior-complement
Exclude ineligible/skip-key paragraphs and duplicate indices. Odd quota initially favors narration; deficits fill from the other pool. Supplied Para words/dialogue are authoritative; select does not normalize them.

## reverse-reconstruction
All Para/Pick/skip triples with the same selected index set form the preimage; arbitrary title/author changes are compatible antecedents. Output indices alone do not recover text, rejected rows, seed or quota weights.

## invariant
Source-bound operational relation over supplied finite inputs; deterministic observations under fixed order and seed. No corpus/distribution assumption. Integer quota arithmetic assumes 32/64-bit usize.

## cost
Combine quota cost with O(P log B+P log P+key bytes) ranking/grouping and O(P+B) temporary storage. Addressable stored sentence vectors bound aggregate sentence counts.

## specialization
Type selection_observation: scoped observation above, exact intended relation, no approximation-error budget asserted.

## checks
selection_order_filter_skip_balance_and_rounds. See ../model.md#evidence for actual profiles and negative controls.

## epoch-memory

Model written from the existing mechanism before numerical repair. Source hash "(see History)". Finite witnesses constrain this relation, not a general proof.

## History

### 2026-10-30 · epoch-memory

2026-10-30: model written from the existing mechanism before numerical repair. Source hash "0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554". Finite witnesses constrain this relation, not a general proof.

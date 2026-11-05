# ranking

## forward
Group supplied pool indices by document; sort within each by (seeded key hash,index), label rank r; globally sort (r,document hash,index), take k. Sorting resolves randomized HashMap traversal.

## behavior-complement
Exclude later per-document rank before an earlier available rank. No random iteration-order tie decides the result. Duplicate pool indices are outside the production pool construction invariant.

## reverse-reconstruction
Output prefixes permit all compatible pools whose first k sorted rank triples are those indices. Unselected suffixes and hash collisions prevent unique reconstruction; fixed supplied all/pool/seed reconstruct the output exactly.

## invariant
Source-bound operational relation over supplied finite inputs; deterministic observations under fixed order and seed. No corpus/distribution assumption. Integer quota arithmetic assumes 32/64-bit usize.

## cost
P pool indices: O(P log P) sorting plus key hashing, O(P) temporary storage.

## specialization
Type round_robin_prefix: scoped observation above, exact intended relation, no approximation-error budget asserted.

## checks
selection_order_filter_skip_balance_and_rounds. See ../model.md#evidence for actual profiles and negative controls.

## epoch-memory

Model written from the existing mechanism before numerical repair. Source hash "(see History)". Finite witnesses constrain this relation, not a general proof.

## History

### 2026-10-30 · epoch-memory

2026-10-30: model written from the existing mechanism before numerical repair. Source hash "0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554". Finite witnesses constrain this relation, not a general proof.

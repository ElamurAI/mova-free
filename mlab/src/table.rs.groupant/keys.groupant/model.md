# mlab-table-keys

## descriptant

### mlab-table-keys

Ascending cmp orders NaN/None last and equal to themselves. Stable sorted_rows preserves tie indices; groups sorts by keys and restores appearance within each group. row_key returns None for any missing and canonicalizes signed zero.

## approximant

### keys: forward relation

Ascending cmp orders NaN/None last and equal to themselves. Stable sorted_rows preserves tie indices; groups sorts by keys and restores appearance within each group. row_key returns None for any missing and canonicalizes signed zero.

Domain: Well-shaped public Table/Mat/StrArr, in-range zero-based internal indices, bounded i64 civil arithmetic. Malformed direct structs may panic.

Executable surrogate gap: no measured cheaper implementation with a useful scoped metric is supplied. Categorical/error/effect relations are exact obligations, not percent losses. Future numeric caps only1/2/5/10%.

## invariant

### keys: invariant and cost

NaNs group together but produce no hash key; +0 and -0 share numeric key0.

One-key numeric fast path adds original index as tie breaker; multi-key stable sort relies on sort stability.

Scope-specific cost: Sorting O(n log n * key-comparison cost); grouping stores O(n) indices; row_key O(k + copied text bytes).

## models

## behavior_complement

Missing grouping does not imply missing join equality; join keys exclude missing. Numeric/text keys remain distinct tagged atoms.

## reverse_reconstruction

Sorted output admits any permutation of unequal-key source rows with corresponding index mapping; groups discard original inter-group order but retain member indices.

## missing_relation

NaNs group together but produce no hash key; +0 and -0 share numeric key0.

## stable_ties

One-key numeric fast path adds original index as tie breaker; multi-key stable sort relies on sort stability.

## branch_relation

Ascending cmp orders NaN/None last and equal to themselves. Stable sorted_rows preserves tie indices; groups sorts by keys and restores appearance within each group. row_key returns None for any missing and canonicalizes signed zero.

## resource_cost

Sorting O(n log n * key-comparison cost); grouping stores O(n) indices; row_key O(k + copied text bytes).

## tests

### Evidence boundary

See ../model.md#tests for the actual selected source test and mutant. These tests sample the file, not every fragment branch. Missing grouping does not imply missing join equality; join keys exclude missing. Numeric/text keys remain distinct tagged atoms. Other branches here are source-derived and unexecuted unless the parent test explicitly calls them.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: isolated source-bound fragment over bytes [11897,15045); includes no claim beyond this relation. Source hash 7af3a65aabe33226d39f741909fd08d74c3b3e43390417c64e617a10704ac26b.

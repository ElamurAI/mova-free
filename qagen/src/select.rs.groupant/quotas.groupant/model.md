# quotas

## forward
Initialize q=0 and L=min(n,sum caps). Each pass opens unsaturated size keys, sets W=sum weights, and if W>0 computes a_i=floor(L*w_i/W), r_i=L*w_i mod W. Award L-sum(a) extra seats in decreasing r then lexical key order, clip to room, subtract awards and repeat.

## behavior-complement
Exclude cap excess, keys absent from size, negative assignments and nonlexical ties. Positive weights exhausted implies termination even if zero-weight capacity remains. Missing caps are zero; cap-only keys never receive assignments.

## reverse-reconstruction
Fiber of q consists of all (size,cap,n) whose successive Hamilton rounds yield q. Positive common scaling of all weights preserves q when inputs remain representable. Clipped capacities and unused extra keys destroy injectivity; no unique recovery is claimed.

## invariant
Source-bound operational relation over supplied finite inputs; deterministic observations under fixed order and seed. No corpus/distribution assumption. Integer quota arithmetic assumes 32/64-bit usize.

## cost
Let B=|size| and C=|cap|, including cap-only keys. Initialization builds B zero quotas (conservatively O(B log(B+1)) key comparisons), clones all size keys and scans C capacities, even when n=0 and R=0. Each of R passes adds O(B log(B+C+1)) comparison/map work, including cap lookups; thus O(B log(B+1)+C+R B log(B+C+1)) unit-comparison work. String comparison costs and key-clone bytes are additional, not constant-time assumptions about arbitrary keys. Storage is O(B) entries plus cloned key bytes. R<=B+1 because a continuing clipped pass saturates a key. u128 division cost is not benchmarked.

## specialization
Type hamilton_remainder_order: scoped observation above, exact intended relation, no approximation-error budget asserted.

## checks
quota_small_exhaustive_rational_oracle; quota_exact_above_f64_integer_precision; quota_capacity_and_weight_sums_exceed_usize; quota_missing_extra_zero_and_capped_domains. See ../model.md#evidence for actual profiles and negative controls.

## epoch-memory

Model written from the existing mechanism before numerical repair. Source hash "(see History)". Finite witnesses constrain this relation, not a general proof. Independent review qualified the cost model: initialization survives zero rounds, cap-only keys contribute their own scan and lookup size, and strings are not unit-cost byte sequences. Source, native receipts and syntax hashes remain unchanged.

## History

### 2026-10-30 · epoch-memory

2026-10-30: model written from the existing mechanism before numerical repair. Source hash "0c35caf4eda91c0a5d36bce728f3846b360b483a66fa4ca3c884964839cd2554". Finite witnesses constrain this relation, not a general proof. Independent review qualified the cost model: initialization survives zero rounds, cap-only keys contribute their own scan and lookup size, and strings are not unit-cost byte sequences. Source, native receipts and syntax hashes remain unchanged.

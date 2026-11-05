# world-store-coupling

## descriptant

# coupling

## Forward

Sampling advances a seed-derived xorshift stream once per ordered candidate, combines the state with ID salt, sorts key/ID pairs, takes k and sorts retained IDs. Filtering first changes stream positions and competitors.

## Invariant

Sample output is sorted, unique and a subset of duplicate-free input candidates, of length min(k,n). Changed eligibility can change unchanged output membership. Reverse fibers retain candidate ordering and losing candidates.

## Cost

O(n) pair allocation and O(n log n) sorting. This is the same sampling work modeled by the parent and must not be added again.

## Epoch memory

## catalog

# world-store-coupling

## nonbehavior

Type: behavior_complement.

Universe: ordered duplicate-free candidate vectors, valid k and seed, and output vectors. Candidate inner complement includes sampled outputs longer than min(k,n), unsorted outputs, duplicate outputs, and IDs absent from candidates. This is a conditional source model of the sample stage, not the complement of the full query.

## antecedents

Type: reverse_reconstruction.

Fix k and seed. A sampled observation Y has every ordered candidate vector C whose first min(k,|C|) pairs in sorted(key(C position,seed) xor id salt,id) have numerically sorted selected IDs equal to Y as compatible antecedents. Different candidate orders and losing candidates remain possible. No unique inverse; k=0 maps every candidate vector to empty.

## positional-coupling

Type: dependency_closure.

The xorshift stream advances per candidate position. Removing one eligible ID changes ranks of later unchanged IDs as well as competition for k slots. Changed-ID dictionaries therefore do not bound output membership changes. Historical witness eligible [0,1,2] to [0,2], C={1}, pred>=1 sample=1/2 gives old [1], fresh [2], legacy merge []; repaired load must return [2].

## rank-boundary

Type: counterfactual.

Hold query order and seed fixed, replace legacy delta-merge by complete target selection for stale sampled queries. Output equals F even if O\C differs from F\C. This intervention fixes candidate coupling but does not validate feature extraction, mutable same-hash indexes or forged stable stamps. Oracle compares repaired witness and rejects historical empty result.

## History

### 2026-10-29 · Epoch memory

2026-10-29: The removed-ID witness disproved that exact eligibility deltas suffice for sampled carry. Fixed-seed determinism does not imply candidate independence.

### Undated · document

Source SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Conditional models over successful finite operations; I/O failure, malformed offsets and changing dependency state are excluded unless stated. Algebra is exact only for its declared abstract relation. Source correspondence remains bounded.

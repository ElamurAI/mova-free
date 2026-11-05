# world-store-carry

## descriptant

# carry

## Forward

Order stamps check key/base; ordinary stable/current stamps return saved parsed rows. Stale sampled/prefix queries recompute once, persist and return that vector. Remaining ancestor queries merge O outside C with F inside C; unrelated versions recompute.

## Invariant

Successful recomputation persists and returns one selected vector. Pointwise carry equals full selection iff O outside C equals F outside C. Stable stamps remain trusted even with manually incompatible queries.

## Cost

Full selection plus serialization on recompute. Ancestor carry also performs full selection and dictionary reads. Changed-ID count does not count evaluated candidates.

## Epoch memory

## catalog

# world-store-carry

## nonbehavior

Type: behavior_complement.

Universe: valid acyclic stores, successful calls, valid query and saved-stamp state paired with returned IDs and rewritten bytes. Candidate inner complement: a stale sampled tree set taking the ancestor delta-merge branch; a current or stable non-order stamp rewriting on load; an order stamp ignoring a changed key. These exclusions depend on branch priority, not merely the final vector. Oracle: repaired witness, stamp fixtures and source branch inspection.

## antecedents

Type: reverse_reconstruction.

Fix query, target version and filesystem dependencies. The preimage of returned Y is the union of admitted saved files over all stamp branches: current/stable parsed rows equal Y; invalidated sampling/order/unrelated branches have select(target)=Y; pointwise ancestor branch satisfies O minus C = Y minus C and F intersect C = Y intersect C. Values O intersect C are unconstrained. Branch evidence cannot be recovered from IDs alone; saved bytes and writes refine this fiber.

## carry-equivalence

Type: membership_transfer.

For sets O,F,C, M=(O\C) union (F intersect C). Exactly M=F iff O\C=F\C. SampleCarry.lean proves this for arbitrary sets, equivalently symmetric difference of O and F is contained in C, and proves the complete compatible-predecessor relation. Its independent false universal equality control fails. This is an abstract proof, not verified Rust correspondence. Exhaustive finite oracle enumerates every triple over four IDs and every candidate output; it checks equality, complement and reverse fibers. This algebra is exact for the merge operator, not a proof that production dictionaries cover all relevant feature deltas.

## invalidation

Type: policy_partition.

Priority is order-stamp handling, then stable/current shortcut, then stale sampled/prefix recomputation, then ancestor discovery and pointwise merge or unrelated recomputation. Recompute discards old membership, including duplicates and stale omissions. A stable stamp is trusted even if manually paired with a tree query; malformed saved-state authentication is outside this repair. Exact source and fixture checks bind the policy.

## cost (catalog)

Type: resource_preimage.

Recompute selects once, then persists and returns that same vector. For stale sampled queries full selection costs O(n log n) ranking per sample term; it avoids ancestor dictionary scanning. Bounded calibration is recorded separately in ../../model.md#current-evidence; it does not establish a general speedup. Pointwise ancestor carry still computes full selection then restricts merge to changed IDs. Refreshed prefix stamps use the shared recompute path. Cache, I/O and repeated sampling terms are held explicit.

## History

### 2026-10-29 · Epoch memory

2026-10-29: The baseline witness returned [] versus full [2]. The repaired regression requires [2]. Finite algebra checks the merge relation and its converse/complement; it does not certify feature extraction.

### Undated · document

Source SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Conditional models over successful finite operations; I/O failure, malformed offsets and changing dependency state are excluded unless stated. Algebra is exact only for its declared abstract relation. Source correspondence remains bounded.

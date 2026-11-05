# algorithm-cartesian-tree

## descriptant

### cartesian_tree mod.rs

Retained observation: Tie orientation, residue normalization and first-error order are part of the public behavior, not incidental implementation choices.

The reference attribution is in production module documentation. No copied solution text or implementation is used. No approximate surrogate is useful for the exact API here, and no performance gain is claimed. Partial syntax is generated twice identically; macro bodies remain opaque and semantic AST/proofs are absent. These small files have no nested fragments.

## model

# cartesian_tree mod.rs model

## Forward

For each interval [lo,hi), choose the least index r attaining its minimum, recursively attach [lo,r) left and (r,hi) right. Empty interval gives None. The stack implements this tree: pop strictly greater suffixes, attach the last popped root as the new left child, then replace the surviving top's right child. Return explicit root/parent/left/right indices.

## Invariant

Inorder is exactly 0..n. Each nonroot has one reciprocal parent link, every child has value >= its parent, and equal-value ancestry selects the smaller index first. Stack indices increase and values do not decrease. n<=2^20.

## Cost

At most n pushes and n pops, O(n) lawful-Ord comparisons and O(n) storage: three link vectors and one stack. Comparison cost is external to this count; no timing gain is asserted.

## behavior_complement

Universe: admitted input sequences paired with candidate four-vector trees. Absent outputs include cycles, missing indices, nonreciprocal links, roots other than the leftmost global minimum, and any subtree root differing from that interval's leftmost minimum. These recursive interval constraints characterize the unique tree on lawful Ord. Over-cap input paired with Ok is absent; allocator refusal may yield AllocationFailed.

## reverse_reconstruction

Given an observed tree retain all sequences whose leftmost-minimum recursive decomposition is that tree. Monotone value embeddings preserve the tree, so [1,2] and [8,9] share a right chain; [1,1] does too. Shape loses magnitudes and some equality information. Empty output has length zero; errors retain broad input/allocator fibers.

## tie_orientation

Strict greater-than popping preserves equal predecessors. For all-equal n>0 the root is 0 and right[i]=i+1 until the last node. Replacing > with >= changes this fiber and is rejected by the recursive oracle.

## stack_amortization

Potential Phi=stack.len: each pop lowers Phi by one, each input pushes once. Total pops <=n, although one step may pop n-1. The full-length equal chain test exercises heap vectors without recursive production traversal.

## allocation_partition

TooLong has priority over all allocation attempts. Within cap three link vectors and stack use try_reserve_exact; later refusal drops local allocations and leaves caller input untouched. Allocation refusal is modeled but not injected.

## evidence

### Evidence

Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

Raw logs are external. Bound evidence hashes:

No whole-repository test, allocator injection, general proof, live parser/teacher/GPU, approximation-error or runtime improvement claim.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model selected bounded exact computation before implementation. At most n pushes and n pops, O(n) lawful-Ord comparisons and O(n) storage: three link vectors and one stack. Comparison cost is external to this count; no timing gain is asserted.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `cba32da7416cb71429b8d492de8f57fd8d116bfdc9ea30c0922264478e4d0dd9`. Twelve focused named tests (three per module) pass twice in debug and twice in release with identical normalized outcomes. Strict all-target Clippy and narrow rustfmt pass. Each production module has a compiled negative control rejected by its exhaustive assertion; test false-oracle controls are separately rejected. Exact source bytes are restored and positives rerun.

- debug1.log: `021da34b29ae9f5cc1d3751645c5bb3a2c09a62eeda74ea8c442c94a93c53c25`

- debug2.log: `350882b59fe92db00ef7c743664a7433915b683196ef7b2b3ce4bd8f5d8f187e`

- release1.log: `c6cb5024abb354e23a1375199f33038203b09305a89fb1352f60ae20b2515f87`

- release2.log: `ec4f6efdc7bd8210b565e8cbe9cb4007b7cbde559d25e5bdfca3f24f18b6e975`

- clippy.log: `d47f40a950cf3add1187a3c07f8b1127360005b8cb35d241c708c19d6a88de72`

- controls.log: `d5acd772f862cfc084f5ea74402e308c05b6d234805115044bc7885eaa012c8e`

- restored.log: `04a4e8576898dcaaf398f5d69f3023d009bc6764fbbca286ff6de0d3436626b7`

- normalized.txt: `f018bb079b167e41fcff7b40a8afa77507f8eba332c86b50064506e1cb318d19`

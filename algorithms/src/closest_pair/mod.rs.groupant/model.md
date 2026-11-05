# algorithm-closest-pair

## descriptant

### closest_pair mod.rs mathematical combiner

AST and source hash bind the mathematical relations to this file. Forward behavior, behavior complement and reconstruction fibers constrain different observations; none replaces the others. Composition requires the parent to connect public calls, shared Point representation and tests. Partial syntax omits semantic resolution and macro expansion.

Epoch memory: coordinates are i32 deliberately; full-domain i64 would require wider cross/distance handling. Duplicate points must be handled before closest-pair packing arguments. Collinear hull boundary points are deliberately discarded. No runtime acceleration or universal correctness claim follows from finite tests.

## model

# closest_pair mod.rs

## Forward

Return None for n<2; sort indexed points, select canonical zero pair if duplicates exist, else recursively combine half minima and inclusive y-sorted strip candidates using (distance,indices) order.

## Invariant

Duplicates return early, so recursive best distance is positive. At initial delta²=best, each half has internal separation >=delta. A strip of width 2delta and height delta partitions in each half into constant many cells of diameter <delta; hence constant many candidates per point, even with boundary ties. Re-sorting strips gives T(n)=2T(n/2)+O(n log n), O(n log²n); sequential recursion peaks at O(n) vectors. Difference magnitude <=2^32-1; square <2^64; sum <=2(2^32-1)^2<2^65. Widen before subtraction/squaring, then convert nonnegative terms to u128. Original index enumeration preserves identity.

## Cost

O(n log²n) time, O(n) peak auxiliary memory, O(log n) stack; unmeasured wall-clock cost.

## nonbehavior

Type: behavior_complement. Exclude None with n>=2, Some with n<2, indices outside 0<=i<j<n, incorrect squared separation, existence of a closer pair or a lexicographically smaller tied pair, and input mutation.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## preimages

Type: reverse_reconstruction. For result (i,j,d), compatible inputs have length >j, separation d at (i,j), and every pair key (distance,(a,b)) >= (d,(i,j)). None admits exactly sequences of length zero or one. Translations, rotations and varying distant points yield ambiguous coordinates and lengths.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## isometry

Type: metamorphic_relation. Translation and signed coordinate permutations preserving i32 bounds preserve every distance and therefore selected original indices. Input permutations preserve minimum distance but can change index ties.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## scaling

Type: metamorphic_relation. Positive integral scaling k with representable images multiplies squared distance by k² and preserves indices; k=0 collapses all points and need not preserve indices.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## duplicate

Type: duplicate_fiber. Zero distance iff equal coordinates. Sorted (point,index) runs start with the smallest two indices of each run. Minimum over adjacent equal-point pairs selects the globally least pair. All-equal input returns (0,1,0) without recursion.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## split

Type: decomposition_relation. For x-sorted unique inputs, the global minimum is the minimum of two half minima and cross pairs. Cross pairs no farther than current best have both x distances to split_x <=sqrt(best); inclusive strip and y boundaries retain all distance/index ties.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## packing

Type: cost_bound. Duplicates return early, so recursive best distance is positive. At initial delta²=best, each half has internal separation >=delta. A strip of width 2delta and height delta partitions in each half into constant many cells of diameter <delta; hence constant many candidates per point, even with boundary ties. Re-sorting strips gives T(n)=2T(n/2)+O(n log n), O(n log²n); sequential recursion peaks at O(n) vectors.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## arithmetic

Type: range_bound. Difference magnitude <=2^32-1; square <2^64; sum <=2(2^32-1)^2<2^65. Widen before subtraction/squaring, then convert nonnegative terms to u128. Original index enumeration preserves identity.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## evidence

### Evidence

Source-bound validation: debug and release each run twice with normalized test outcomes compared; strict Clippy and rustfmt checks; two identical installed mova-modelcheck syntax projections. Focused compiled negative control must compile, fail a real assertion and then be restored. See the external geometry run directory for raw logs. Finite checks do not establish universal correctness or timing bounds.

nonbehavior, preimages, isometry, scaling, duplicate, split, packing, arithmetic are source-derived observation models. Tests reside in adjacent tests.rs with an independent oracle.

## prospective

### Prospective model

Before validation: predict exact agreement with independent certificates and pair enumeration; canonical ties survive inclusive strip boundaries. Full i32 differences and squared sums fit the stated widened bounds. Debug and release normalized assertions should agree. Changing squared-distance sum to one coordinate should disagree with exhaustive pair assertions. No performance speedup is predicted.

## History

### 2026-10-30 · descriptant

Original std-only code and fixtures for Mova, 2026-10-30. The implementation combines exact coordinate widening, ordering and local geometric predicates. Shamos and Hoey (1975), FOCS, 151–162 describes classical closest-point divide-and-conquer; this version re-sorts strips. No external implementation was copied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Measured 2026-10-30: four full algorithms runs each passed 175 unit tests plus one doctest (176 normalized passing lines), identical across debug/release twice. Eight new geometry tests passed. Strict Clippy with -D warnings and rustfmt check passed. Each source projection matched byte-for-byte twice. Both compiled mutants were rejected by assertion failures; restored full suite passed. Hull mutant changed <=0 to <0 in stack popping; closest-pair mutant dropped the y-square distance term. These controls establish local test sensitivity, not proof.

# algorithm-convex-hull

## descriptant

### convex_hull mod.rs mathematical combiner

AST and source hash bind the mathematical relations to this file. Forward behavior, behavior complement and reconstruction fibers constrain different observations; none replaces the others. Composition requires the parent to connect public calls, shared Point representation and tests. Partial syntax omits semantic resolution and macro expansion.

Epoch memory: coordinates are i32 deliberately; full-domain i64 would require wider cross/distance handling. Duplicate points must be handled before closest-pair packing arguments. Collinear hull boundary points are deliberately discarded. No runtime acceleration or universal correctness claim follows from finite tests.

## model

# convex_hull mod.rs

## Forward

Sort and deduplicate points, construct lower/upper strict-left-turn chains, concatenate without repeated endpoints. Return canonical extreme vertices counterclockwise.

## Invariant

Each processed sorted prefix has a lower-boundary stack with strict positive consecutive turns. Each point is pushed once and popped at most once per chain. Reversal builds the upper boundary; dropping terminal endpoints prevents repeated joins. Each coordinate difference magnitude <=2^32-1; product <=(2^32-1)^2; cross magnitude <=2(2^32-1)^2<2^65. Widening before subtraction gives exact i128 predicates. Allocation failure is not a Result.

## Cost

O(n log n) comparisons, O(n) stack work and auxiliary memory; unmeasured wall-clock cost.

## nonbehavior

Type: behavior_complement. Exclude absent-source vertices, nonminimal first point, repeats, clockwise/non-strict polygon turns, or input strictly right of an edge. For a segment exclude off-line points or points beyond endpoints. Empty output requires empty input.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## preimages

Type: reverse_reconstruction. For canonical strict convex polygon H, compatible inputs are finite sequences containing every H vertex, with remaining points inside/on H, arbitrary order/multiplicity. Segment output admits points on its closed segment containing both ends. Singleton admits nonempty repetitions; empty admits only empty. Length and interior data are not recoverable.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## permutation

Type: metamorphic_relation. Permutation or duplication preserves output since sorted distinct support is unchanged.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## affine

Type: metamorphic_relation. Positive uniform integer scaling k and translation t, with all images in i32, maps H to kH+t and preserves its canonical start. Reflection requires reversing orientation and recanonicalization.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## idempotence

Type: fixed_point. H(H(P))=H(P); its extreme vertices have identical convex support, and collinear interior points cannot return.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## support

Type: support_relation. For every integer direction d, max dot(d,P)=max dot(d,H(P)) on nonempty input. Strict turns, source membership, supporting edges and minimal start certify polygons; degenerates require separate conditions.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## stack

Type: state_invariant. Each processed sorted prefix has a lower-boundary stack with strict positive consecutive turns. Each point is pushed once and popped at most once per chain. Reversal builds the upper boundary; dropping terminal endpoints prevents repeated joins.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## arithmetic

Type: range_bound. Each coordinate difference magnitude <=2^32-1; product <=(2^32-1)^2; cross magnitude <=2(2^32-1)^2<2^65. Widening before subtraction gives exact i128 predicates. Allocation failure is not a Result.

Domain: Finite slices of i32-coordinate points under ordinary allocation-success Rust execution. Source-derived relation, not a universal proof.

## evidence

### Evidence

Source-bound validation: debug and release each run twice with normalized test outcomes compared; strict Clippy and rustfmt checks; two identical installed mova-modelcheck syntax projections. Focused compiled negative control must compile, fail a real assertion and then be restored. See the external geometry run directory for raw logs. Finite checks do not establish universal correctness or timing bounds.

nonbehavior, preimages, permutation, affine, idempotence, support, stack, arithmetic are source-derived observation models. Tests reside in adjacent tests.rs with an independent oracle.

## prospective

### Prospective model

Before validation: predict exact agreement with independent certificates and pair enumeration; canonical ties survive inclusive strip boundaries. Full i32 differences and squared sums fit the stated widened bounds. Debug and release normalized assertions should agree. Changing <=0 turn popping to <0 should retain forbidden collinear vertices. No performance speedup is predicted.

## History

### 2026-10-30 · descriptant

Original std-only code and fixtures for Mova, 2026-10-30. The implementation combines exact coordinate widening, ordering and local geometric predicates. Andrew (1979), Information Processing Letters 9(5), 216–219 describes the classical monotone-chain method. No external implementation was copied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Measured 2026-10-30: four full algorithms runs each passed 175 unit tests plus one doctest (176 normalized passing lines), identical across debug/release twice. Eight new geometry tests passed. Strict Clippy with -D warnings and rustfmt check passed. Each source projection matched byte-for-byte twice. Both compiled mutants were rejected by assertion failures; restored full suite passed. Hull mutant changed <=0 to <0 in stack popping; closest-pair mutant dropped the y-square distance term. These controls establish local test sensitivity, not proof.

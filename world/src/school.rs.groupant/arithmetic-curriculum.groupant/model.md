# world-school-arithmetic-curriculum

## forward

Op::apply uses checked Add/Mul, checked Sub only when a>=b, and Div only for exact representable quotients. Problem::answer maps None to legacy zero. Fixed xorshift and template filtering assemble the elementary curriculum; revision sorts last.

## invariant

Sub never returns negative; Div rejects zero, nonzero remainder and MIN/-1. Signed Add/Mul/Div results are allowed. PRACTICE and TEST have distinct complete strings, not disjoint vocabulary. Rng range requires a valid small inclusive range and pick a nonempty slice.

## cost

Arithmetic O(1); template generation linear output bytes; lesson sorting O(L log L). Checked scalar arithmetic keeps the original asymptotic cost. No performance optimization claimed.

## composition

Problem::answer and Student::answer share apply. Public Problem admits arbitrary i64, unlike bounded generators. The native oracle uses i128 for 25x25x4 rows; five extreme cases reject overflow. Template separation and revision order are constant-only checks.

## epoch-memory

Initial school source-bound epoch. Prospective model: keep O(1) arithmetic/partition cost, widen aggregation scalars, and expose actual codecs without filesystem effects. Old native witnesses found unchecked operation, score, marginal vote, partition and duplicate-vote arithmetic; legacy serialization synthesized zero accumulators. Review found that dropping explicit zero accumulators also changed the prediction table. Both activation directions now have named tests. No speedup, approximate-error profile or general semantic proof is claimed. Unchecked update/world arithmetic retains representability premises.

## checks

cargo test -p world --lib school::wave12_models -- --test-threads=1. Root ../model.md#evidence binds exact source/evidence hashes, repeated debug/release outcomes and a compiled production mutation. Named fixture domains appear in composition. Syntax is a source-bound partial projection; macros remain opaque. No semantic AST or live school execution.

## behavior-complement

Subject/revision: manifest hash and byte scope. Universe: supplied finite inputs, fixed dependencies, successful allocation and the explicit arithmetic premises of this scope. Inner absent-behavior relation: Some(z) is absent for overflow, a<b subtraction, zero divisor, inexact division and MIN/-1. The model excludes results different from the exact representable equation. Invalid Rng inputs remain outside scope. These are source-derived constraints with finite named witnesses where indicated; external behavior remains unknown.

## reverse-reconstruction

Subject/revision: manifest hash. Operator: compatible preimage/converse, holding dependencies and admissibility fixed. For fixed op,z retain all i64 pairs satisfying the operation equation and its guards. Add(z=0) includes (0,0),(1,-1); Mul(z=0) has both zero-axis fibers. Problem::answer(0) also admits invalid operations, so its scalar result cannot recover success. Partial relational description, not a complete inverse search or a unique causal history.

## coupling

Subject/revision: manifest hash. Constraint relation on stage identities and ordered intermediate states: Problem::answer and Student::answer share apply. Public Problem admits arbitrary i64, unlike bounded generators. The native oracle uses i128 for 25x25x4 rows; five extreme cases reject overflow. Template separation and revision order are constant-only checks. Supplied native joins and source inspection are distinct evidence.

## observation-fiber

Subject/revision: manifest hash. Domain: scoped inputs with the declared observation. The information-loss relation equates inputs whose observation agrees: Invalid arithmetic collides with genuine zero through Problem::answer's legacy fallback. Retain Option from apply when successful zero must remain distinguishable. Finite tests do not establish unrestricted completeness.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial school source-bound epoch. Prospective model: keep O(1) arithmetic/partition cost, widen aggregation scalars, and expose actual codecs without filesystem effects. Old native witnesses found unchecked operation, score, marginal vote, partition and duplicate-vote arithmetic; legacy serialization synthesized zero accumulators. Review found that dropping explicit zero accumulators also changed the prediction table. Both activation directions now have named tests. No speedup, approximate-error profile or general semantic proof is claimed. Unchecked update/world arithmetic retains representability premises.

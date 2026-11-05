# algorithm-extended-gcd

## descriptant

### algorithm-extended-gcd descriptant

The combiner relates source-bound partial syntax, forward recurrence, typed output-preimage and behavior-exclusion relations, invariants and finite observations. It preserves disagreement as an open obligation; structural consistency does not prove Rust semantics.

The bounded oracle enumerates common divisors without Euclidean recurrence. Full-width checks use checked Bezout identity, divisibility, coefficient bounds and gcd symmetry; those checks certify each observed gcd independently.

Prospective Euclidean row preservation and widened coefficient model survived bounded checks. Replacing old_x - quotient*x by old_x + quotient*x compiled and tests::bounded_divisor_oracle rejected it (one failure). Terminal zero-row coefficient bounds are distinct from returned-row product bounds; the loose square of u64 maximum would not justify i128 product safety.

## model

# algorithm-extended-gcd

## Forward

Define E(a,b) by the Euclidean recurrence with initial remainder rows (a,1,0),(b,0,1), updating (r,s,t),(r1,s1,t1) to (r1,s1,t1),(r%r1,s-q*s1,t-q*t1), q=floor(r/r1). Return the first row when r1=0; explicitly E(0,0)=(0,0,0). This fixes deterministic coefficients, rather than admitting every Bezout solution.

## Invariant

Each row satisfies r=a*s+b*t over mathematical integers. gcd of remainder pairs is preserved and the second nonzero remainder strictly decreases. After initial zero coefficients, successive coefficients alternate signs and their absolute values grow to the terminal zero row (b/g,-a/g), up to sign; hence each coefficient magnitude is at most max(a,b). In q*s1=s-s_next, the triangle bound is at most 2*max(a,b), strictly below i128::MAX. The same holds for t, proving coefficient update safety without the invalid max(a,b)^2 argument. The returned row differs from the terminal zero row. For positive distinct inputs the returned x is at most b/(2g) in magnitude when b/g>1 (otherwise the unit coefficient case is handled directly), and analogously y; identity test products therefore fit i128. Zero and equal inputs have unit/zero coefficients.

## Cost

O(1+log(max(a,b)+1)) word divisions, O(1) storage. Symbolic cost; no timing claim.

## nonbehavior

In U of input pairs and candidate triples, U minus graph(E) contains wrong gcds, violated integer Bezout identities and coefficient triples unequal to the deterministic Euclidean result even if their identity holds. This exact recurrence target has finite correspondence evidence, not a formal Rust proof.

## coefficient_fibers

Given (g,x,y), reconstruct all u64 pairs (a,b) satisfying E(a,b)=(g,x,y). Necessarily ax+by=g and gcd(a,b)=g, but those conditions alone do not enforce the coefficient policy. For g=0 only (0,0) with x=y=0 qualifies; (g,1,0) for g>0 can admit several b. Preserve the recurrence constraint and ambiguity.

## gcd_symmetry

The gcd projection is unchanged by swapping a,b. Coefficients need not swap under tie and zero conventions; the tests compare only gcd under swapping.

## zero_boundary

For a>0 E(a,0)=(a,1,0); for b>0 E(0,b)=(b,0,1); E(0,0)=(0,0,0).

## linear_certificate

For a returned g>0, divisibility of a and b by g plus ax+by=g establishes greatest common divisor over integers: every common divisor divides the displayed combination. Runtime evaluates no potentially large input-coefficient products. Tests check their products with checked i128 operations.

## Limits

Exact implementation; mathematical models are not formal proofs. Partial syntax does not expand macros or resolve types. No numerical approximation profile or unsupported error budget is introduced.

## affine_solution_family

For (a,b)!=(0,0), let E(a,b)=(g,x0,y0). All integer solutions to ax+by=g are exactly x=x0+k*(b/g), y=y0-k*(a/g), k in Z. This describes certificate solutions, not the graph of the deterministic coefficient policy: only k=0 is returned. For (0,0), every integer coefficient pair solves the equation but only (0,0) is returned. Arbitrary k and products require mathematical integers or checked arithmetic; i128 is not a universal family domain.

## divisibility_lattice

For g>0, positive d divides both a and b iff d divides g. Thus g is the greatest common divisor in the divisibility order and ax+by=c has an integer solution iff g divides c. Sufficiency scales the returned certificate by c/g; necessity follows by divisibility of every linear combination. At (0,0), only c=0 is solvable, and every positive d divides both inputs; no finite positive maximum exists.

## modular_inverse

For modulus m>=2 and a:u64, gcd(a,m)=1 iff a has an inverse modulo m. The unique representative r in [0,m) is E(a,m).x rem_euclid m. Other Bezout solutions project to the same r. No inverse is inferred when gcd>1; modulus zero is undefined and modulus one is excluded. For verification convert a,r,m to u128: a*r fits u128 for u64 operands; the same product may overflow i128 even though the signed coefficient identity is safe.

## evidence

### Evidence

Three tests in this algorithm pass in debug and release. The worktree crate passes 103 unit tests plus one doctest in both modes; strict all-target clippy with -D warnings passes. 9,409 bounded pairs (0..=96 squared), 49 extremal pairs, explicit zero conventions, consecutive Fibonacci pairs up to u64 overflow, and 4,096 deterministic full-width pairs.

The bounded oracle enumerates common divisors without Euclidean recurrence. Full-width checks use checked Bezout identity, divisibility, coefficient bounds and gcd symmetry; those checks certify each observed gcd independently.

No throughput measurement, semantic AST proof, full-system inference, or executable approximation claim is made. Root integration separately registers these leaves and checks parent bindings.

## prospective

### Prospective model

Before implementation: Euclidean remainder pairs preserve gcd and each remainder's linear combination of the original u64 inputs. Update both signed coefficient pairs with the same quotient. Remainders decrease strictly. Coefficient magnitudes including the terminal zero row are bounded by max(a,b), so i128 has ample room for intermediate quotient products (bounded by twice u64::MAX). Return canonical zero coefficients at (0,0). Expect logarithmic divisions and constant storage. Check independent divisor enumeration on a bounded grid, signed Bezout identities, extrema and consecutive Fibonacci inputs. No performance gain or universal formal proof is claimed.

## History

### 2026-10-30 · descriptant

2026-10-30: Define E(a,b) by the Euclidean recurrence with initial remainder rows (a,1,0),(b,0,1), updating (r,s,t),(r1,s1,t1) to (r1,s1,t1),(r%r1,s-q*s1,t-q*t1), q=floor(r/r1). Return the first row when r1=0; explicitly E(0,0)=(0,0,0). This fixes deterministic coefficients, rather than admitting every Bezout solution.

#### Relation epoch, 2026-10-30

The combiner now separates full mathematical certificate/preimage relations from implementation selection and machine-resource premises. Implementation families contain eight typed relations; test families retain five with strengthened executed correspondence. Three targeted tests augment each prior suite, preserving earlier useful boundary observations. Finite tests and source projections leave general Rust semantics and allocator exhaustion unproved.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: ec9a6f634b6ba2fdb7dc4f1258bfcbf07799ad979dc2f43761a958440d778b3d.

Replacing old_x - quotient*x by old_x + quotient*x compiled and tests::bounded_divisor_oracle rejected it (one failure). The mutation is applied to copied production source and compiled with rustc --test, not emulated by a different implementation. Logs and the Rust harness are external under MOVA_DATA/runs/octopus-alg3-number-2026-10-30. No huge allocation or actual OOM probe was performed.

#### Relation epoch, 2026-10-30

Current source SHA-256: ec9a6f634b6ba2fdb7dc4f1258bfcbf07799ad979dc2f43761a958440d778b3d. Six tests in this module pass twice in debug and twice in release; single-thread result records match within each mode. Strict all-target Clippy passes. A copied production-source mutant compiles and is rejected by a newly added relation test. Partial syntax regenerated twice with identical bytes. External logs and Rust runner: MOVA_DATA/runs/octopus-next-number-2026-10-30. Earlier evidence above remains historical; no timing or universal semantic claim follows.

Current reproducible evidence SHA-256 bindings:

- extended_gcd-debug-records.txt: d82df702d6539776f163156eb3ad9ed2fb7c319eacc3ad7c17533d8e51b99c1f
- extended_gcd-release-records.txt: d82df702d6539776f163156eb3ad9ed2fb7c319eacc3ad7c17533d8e51b99c1f
- extended_gcd-mutant-build.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
- extended_gcd-mutant-rejected.log: 6f30b78b26ea5aed235ec3f0e39e0bdd162053b9ffbbdb0ba4ad4ba4c9af5432

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

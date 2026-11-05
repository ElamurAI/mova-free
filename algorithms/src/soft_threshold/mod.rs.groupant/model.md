# algorithm-soft-threshold

## descriptant

### Modeling

Coordinatewise y=sign(x)*max(|x|-lambda,0), the real proximal minimizer of 0.5||y-x||2^2+lambda||y||1. The f64 subtraction is rounded; all finite scalar differences remain finite because both operands are nonnegative before subtraction.

Retained observation: real convex uniqueness does not make floating-point projection exact. Common large offsets require shifting for simplex; nonfinite intermediates are rejected rather than silently accepted. PAVA sums and L1 norms conservatively reject overflow. Underflow can prevent simplex mass certification. Normal-cone/subgradient fibers preserve many-to-one behavior; negative input tests alone do not model it. No timing or global objective-error claim is inferred.

## model

# soft_threshold mod.rs

## Forward

Coordinatewise y=sign(x)*max(|x|-lambda,0), the real proximal minimizer of 0.5||y-x||2^2+lambda||y||1. The f64 subtraction is rounded; all finite scalar differences remain finite because both operands are nonnegative before subtraction.

## Invariant

Length preserved and values finite. Magnitudes do not increase; nonzero signs agree with input. At exact zero output |x|<=lambda; otherwise y-x+lambda sign(y)=0 in real arithmetic, within rounding in concrete arithmetic.

## Cost

O(n) validation and coordinate writes, O(n) result storage, cap n<=1,000,000. No convergence loop or optimization iteration budget required.

## behavior_complement

Within valid inputs and real candidate outputs, complement contains infeasible or KKT-incompatible pairs and incorrect error outcomes. This is a relational complement of the unique real solution graph, not just a list of negative examples. For the concrete implementation the complement additionally excludes outputs differing from rounded evaluation and includes arithmetic/resource error partitions; real KKT alone is not a full f64 acceptance predicate.

## reverse_reconstruction

At fixed lambda, nonzero y has the unique real antecedent y+lambda sign(y); zero y has every x in [-lambda,lambda]. Whole-vector fiber is the Cartesian product. lambda0 is identity mathematically and concretely. Rounded nonzero fibers can include neighboring representable inputs.

## subgradient_stationarity

0 belongs to y-x+lambda partial|y|. For nonzero y the subgradient is sign(y); for zero y it ranges over [-1,1]. This is sufficient for the unique real minimizer by strong convexity.

## zero_plateau

Every |x|<=lambda maps to a signed zero matching x. Threshold equality belongs to the zero plateau. At lambda0 signed zero input remains signed zero.

## composition_and_contraction

Real maps satisfy S_a(S_b(x))=S_{a+b}(x) and are nonexpansive in Euclidean norm. These identities concern representable finite parameters and exact arithmetic; extra rounding can break bitwise composition.

## arithmetic_resource_partition

Length, finite scalar/vector and nonnegative lambda validate before fallible reservation. Finite max-magnitude subtraction cannot overflow. Underflow and cancellation remain, but no iterative convergence failure or Arithmetic error variant is needed.

## Limits and references

Partial syntax only: no name resolution, semantic AST, macro expansion or formal proof. No executable cheap surrogate is attached: a useful cheaper error-certified kernel has not been established; exact wrappers would not provide approximation evidence. No error budget above 10% or universal accuracy claim is made. Finite tests measure additive tolerances; simplex separately checks relative mass, with zero radius handled exactly. Citations: Barlow et al., Statistical Inference under Order Restrictions (1972); Duchi et al., Efficient Projections onto the l1-Ball for Learning in High Dimensions, ICML (2008); Parikh and Boyd, Proximal Algorithms (2014), section 6.5.2. Original implementation and fixtures, no copied code.

## evidence

### Executed evidence

All 497 library tests passed twice in debug and twice in release; sorted per-test result records match across all four. Eight owned test functions exercise independent oracles, domain boundaries and nonunique fibers. Strict algorithms Clippy all-targets passed. Four compiled production mutants failed their named tests; original bytes restored and full debug suite passed. No allocator failure injection and no universal numerical guarantee.

## prospective

### Prospective projection models
Finite f64 vectors, length <= 1,000,000; scalar radius/lambda finite nonnegative. Fallible allocation; overflow/underflow affecting certificates may return Arithmetic. PAVA projects onto nondecreasing cone with unit weights, O(n) time/space, merged block means. Simplex projects onto nonnegative sum=r via descending sort and shared threshold, O(n log n)/O(n); L1 restores signs after simplex of magnitudes when outside ball. Soft threshold is coordinate prox of lambda L1, O(n)/O(n). Unique exact-real solutions do not imply bit-exact or universal f64 accuracy. Independent enumeration of contiguous partitions/support sets and KKT stationarity will test finite grids; boundary, ties, zero, nonfinite and extreme arithmetic explicit. No executable cheap surrogate: exact kernels are already cheap, no validated cheaper metric-preserving profile. Reverse fibers are normal cones; complement excludes KKT-incompatible candidates in real model. Tests repeated in both profiles and compiled mutants must fail named checks.

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch 2026-10-30` stays in descriptant as `### Modeling`.

### 2026-10-30 · evidence

Heading `### Executed evidence, 2026-10-30` stays in evidence as `### Executed evidence`.

Source SHA-256: 44bfa53a12c010db3f574857fc5a4a28491c7d5f8f34d4c5dec9849477892bc3.

External logs: $MOVA_DATA/runs/octopus-wave19-projections-2026-10-30/.

- debug1.log: 295ce741ceadf13f81530f0c54bba8127ad87d2564e25440eeffc306e03b379d

- debug2.log: ab7c4a15e0d6a5e69f97f82abb191570654c81b4ff00b26644966374e1d35134

- release1.log: 2ac35cb57366af4c04df2fd89e156aba6d5754de09537cf6b2f53e9ba5627e9f

- release2.log: b4e8d171373bf4f066ec13d4bf4548f91a9abd8332101ce878f341acc741ad96

- clippy.log: 21dded34a5924937a217ba2f8c5df5d6339903a33925df0ee04c644de7533461

- restored.log: 072f31b0d0d781b3fcafe1f54ae1844ce6a172fee956f839bbe4ef93cd375a6f

- normalized.txt: 9823784f5cfb78cb5cbafeeacf4f8c9c3d144d42a95895675ac88e8a4e2c5a9f

- mutant-soft_threshold.log: 017fc37c218247ff368c2a7359975d28281f59798d3609141792d72dacc136ca

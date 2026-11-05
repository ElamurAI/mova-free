# algorithm-simplex-projection

## descriptant

### Modeling

Project x onto {y>=0,sum y=r}. Shift x by max(x), sort offsets descending, scan prefix thresholds (prefix_sum-r)/k and retain the last whose kth offset is greater. Restore original order by thresholding each original offset. Real arithmetic solves KKT uniquely; rounded implementation additionally checks relative mass.

Retained observation: real convex uniqueness does not make floating-point projection exact. Common large offsets require shifting for simplex; nonfinite intermediates are rejected rather than silently accepted. PAVA sums and L1 norms conservatively reject overflow. Underflow can prevent simplex mass certification. Normal-cone/subgradient fibers preserve many-to-one behavior; negative input tests alone do not model it. No timing or global objective-error claim is inferred.

## model

# simplex_projection mod.rs

## Forward

Project x onto {y>=0,sum y=r}. Shift x by max(x), sort offsets descending, scan prefix thresholds (prefix_sum-r)/k and retain the last whose kth offset is greater. Restore original order by thresholding each original offset. Real arithmetic solves KKT uniquely; rounded implementation additionally checks relative mass.

## Invariant

For r>0, successful values are finite and nonnegative; checked |sum(y)-r|/r<=64*n*EPSILON. Exact KKT has x_i-y_i=theta on positive coordinates and x_i<=theta on zero coordinates. The checked mass is a rounded sequential sum.

## Cost

O(n) validation and shifting, O(n log n) unstable in-place sorting comparisons, O(n) threshold scan and writes; O(n) vector storage, n<=1,000,000.

## behavior_complement

Within valid inputs and real candidate outputs, complement contains infeasible or KKT-incompatible pairs and incorrect error outcomes. This is a relational complement of the unique real solution graph, not just a list of negative examples. For the concrete implementation the complement additionally excludes outputs differing from rounded evaluation and includes arithmetic/resource error partitions; real KKT alone is not a full f64 acceptance predicate.

## reverse_reconstruction

For fixed r>0 and feasible y, choose any theta; positive coordinates require x_i=y_i+theta, zero coordinates permit x_i<=theta. This nonunique normal-cone fiber includes arbitrary constant shifts. r=0 maps all valid vectors of matching length to zero. Concrete fibers further constrain rounded arithmetic success.

## active_support

Each nonempty support S yields theta=(sum_{i in S} x_i-r)/|S|. It is valid exactly when active x_i>theta and inactive x_i<=theta. Exhaustive small support enumeration does not repeat sorting or the prefix scan.

## translation_invariance

Real projection is invariant under adding a common constant. Shifting by maximum avoids loss of a small radius against a common large offset; [MAX,MAX] with radius2 yields [1,1]. The subtraction can overflow for mixed extreme signs.

## mass_and_zero

Radius zero returns all positive zeros after validation, including empty input. Positive radius with empty input is EmptySimplex. Relative mass divides by strictly positive radius; smallest subnormals may be unrepresentable as equal shares and return Arithmetic.

## arithmetic_resource_partition

Length cap and finite/nonnegative scalar validation precede reservation. Nonfinite offsets/threshold/output/mass and excessive relative residual return Arithmetic. Cancellation may affect support and objective without tripping mass. This is not an analytical accuracy guarantee.

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

Source SHA-256: 410b6027ffb061a2eb575eb877918ccf9f878f93abafa31d03dc1151c62fdf73.

External logs: $MOVA_DATA/runs/octopus-wave19-projections-2026-10-30/.

- debug1.log: 295ce741ceadf13f81530f0c54bba8127ad87d2564e25440eeffc306e03b379d

- debug2.log: ab7c4a15e0d6a5e69f97f82abb191570654c81b4ff00b26644966374e1d35134

- release1.log: 2ac35cb57366af4c04df2fd89e156aba6d5754de09537cf6b2f53e9ba5627e9f

- release2.log: b4e8d171373bf4f066ec13d4bf4548f91a9abd8332101ce878f341acc741ad96

- clippy.log: 21dded34a5924937a217ba2f8c5df5d6339903a33925df0ee04c644de7533461

- restored.log: 072f31b0d0d781b3fcafe1f54ae1844ce6a172fee956f839bbe4ef93cd375a6f

- normalized.txt: 9823784f5cfb78cb5cbafeeacf4f8c9c3d144d42a95895675ac88e8a4e2c5a9f

- mutant-simplex_projection.log: 260b5d53be5ff539636dd8c04a58d8e215fe9738263ec5e92f0c9a540086d8f3

# algorithm-isotonic-regression

## descriptant

### Modeling

Minimize sum_i (y_i-x_i)^2 over nondecreasing y. PAVA pushes singleton (sum,count) blocks and merges the last two while their rounded means decrease. Return each block mean repeated count times. Real arithmetic gives the unique least-squares projection; f64 branching and rounded means are a separate deterministic implementation.

Retained observation: real convex uniqueness does not make floating-point projection exact. Common large offsets require shifting for simplex; nonfinite intermediates are rejected rather than silently accepted. PAVA sums and L1 norms conservatively reject overflow. Underflow can prevent simplex mass certification. Normal-cone/subgradient fibers preserve many-to-one behavior; negative input tests alone do not model it. No timing or global objective-error claim is inferred.

## model

# isotonic_regression mod.rs

## Forward

Minimize sum_i (y_i-x_i)^2 over nondecreasing y. PAVA pushes singleton (sum,count) blocks and merges the last two while their rounded means decrease. Return each block mean repeated count times. Real arithmetic gives the unique least-squares projection; f64 branching and rounded means are a separate deterministic implementation.

## Invariant

Blocks partition each processed prefix, counts are positive and sum to its length. Final rounded means are nondecreasing. Real KKT: residual prefix sums sum_{i<=k}(x_i-y_i)>=0, total residual zero, and prefix residual vanishes at every strict increase.

## Cost

O(n) validation, pushes, merges and output writes, O(n) auxiliary blocks and output. Each merge removes one block; at most n-1 merges. n<=1,000,000 bounds resource policy, not timing.

## behavior_complement

Within valid inputs and real candidate outputs, complement contains infeasible or KKT-incompatible pairs and incorrect error outcomes. This is a relational complement of the unique real solution graph, not just a list of negative examples. For the concrete implementation the complement additionally excludes outputs differing from rounded evaluation and includes arithmetic/resource error partitions; real KKT alone is not a full f64 acceptance predicate.

## reverse_reconstruction

For fixed monotone y, the real fiber is y+N_C(y): residual prefix sums nonnegative, total zero, complementary slackness at strict rises. [2,0] and [1,1] both map to [1,1]. Rounded arithmetic fibers can be larger or differ near rounding boundaries.

## block_conservation

For exact arithmetic, every block sum/count equals its input block average. Merging preserves total sum and count. The concrete sum is rounded and can overflow; no claim of exact sum preservation in f64.

## kkt_prefix

The polar cone is represented by nonnegative residual prefixes with total zero; a positive prefix multiplier may occur only across an equal fitted pair. Enumerated contiguous partitions independently select the minimum squared loss.

## ties_and_idempotence

Already nondecreasing finite vectors pass through singleton blocks, including ties. They are fixed points in both real and concrete models. Equal adjacent means need not merge, but give the same real output.

## arithmetic_resource_partition

Validate length and finiteness before two fallible reservations. Nonfinite merged sums return Arithmetic even for a representable real mean. Counts <=1e6 are exact in f64. Subnormal mean division can underflow; finite success is not a universal objective certificate.

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

Source SHA-256: 7396eea35ccf248b4e2c944b10bf857e728c733af8a883faaade2fc54cf59116.

External logs: $MOVA_DATA/runs/octopus-wave19-projections-2026-10-30/.

- debug1.log: 295ce741ceadf13f81530f0c54bba8127ad87d2564e25440eeffc306e03b379d

- debug2.log: ab7c4a15e0d6a5e69f97f82abb191570654c81b4ff00b26644966374e1d35134

- release1.log: 2ac35cb57366af4c04df2fd89e156aba6d5754de09537cf6b2f53e9ba5627e9f

- release2.log: b4e8d171373bf4f066ec13d4bf4548f91a9abd8332101ce878f341acc741ad96

- clippy.log: 21dded34a5924937a217ba2f8c5df5d6339903a33925df0ee04c644de7533461

- restored.log: 072f31b0d0d781b3fcafe1f54ae1844ce6a172fee956f839bbe4ef93cd375a6f

- normalized.txt: 9823784f5cfb78cb5cbafeeacf4f8c9c3d144d42a95895675ac88e8a4e2c5a9f

- mutant-isotonic_regression.log: 8dd1968a1c5f9d4d1508108860f84e291cd8fbf0af07bff46fa2752171dc04e5

# algorithm-l1-ball-projection

## descriptant

### Modeling

Validate and form absolute magnitudes with sequential L1 norm. Inside ball return input exactly; outside call simplex projection on magnitudes and restore original signs. Radius zero returns all positive zeros. Real model is unique Euclidean L1-ball projection.

Retained observation: real convex uniqueness does not make floating-point projection exact. Common large offsets require shifting for simplex; nonfinite intermediates are rejected rather than silently accepted. PAVA sums and L1 norms conservatively reject overflow. Underflow can prevent simplex mass certification. Normal-cone/subgradient fibers preserve many-to-one behavior; negative input tests alone do not model it. No timing or global objective-error claim is inferred.

## model

# l1_ball_projection mod.rs

## Forward

Validate and form absolute magnitudes with sequential L1 norm. Inside ball return input exactly; outside call simplex projection on magnitudes and restore original signs. Radius zero returns all positive zeros. Real model is unique Euclidean L1-ball projection.

## Invariant

Successful output preserves coordinate signs unless zero; inside-ball path returns unchanged input under computed norm comparison. Outside-ball magnitude mass satisfies simplex relative check, so rounded feasibility is tolerated, not exact. Mathematical KKT uses one nonnegative shrinkage threshold.

## Cost

O(n) validation, magnitude formation and sign restoration. O(n log n) worst-case work via simplex, O(n) additional magnitude and simplex output storage; inside/zero paths O(n). Length cap inherited from simplex.

## behavior_complement

Within valid inputs and real candidate outputs, complement contains infeasible or KKT-incompatible pairs and incorrect error outcomes. This is a relational complement of the unique real solution graph, not just a list of negative examples. For the concrete implementation the complement additionally excludes outputs differing from rounded evaluation and includes arithmetic/resource error partitions; real KKT alone is not a full f64 acceptance predicate.

## reverse_reconstruction

For interior y (||y||1<r), real preimage is uniquely x=y. On the boundary, x-y=lambda*s with lambda>=0, s_i=sign(y_i) when nonzero and |s_i|<=1 when zero. Thus [3,-1] and [4,-1] both map to [2,0] at r2. r0 has every valid x in the zero fiber; rounded fibers differ.

## simplex_composition

Outside the ball, magnitudes satisfy positive-radius simplex domain; sign restoration is an isometry for the relevant orthant. Child mass/rounding failures propagate unchanged; a valid child is necessary but not a universal parent accuracy proof.

## proximal_relation

For exterior inputs a shared lambda satisfies y=soft_threshold(x,lambda) in real arithmetic and sum |y|=r. KKT grid independently checks coordinate stationarity and zero subgradient intervals without calling the soft kernel.

## interior_and_zero

Computed norm<=radius returns input bit pattern unchanged, including signed zeros. Zero radius yields all positive zeros even on extreme finite inputs. Empty vector is feasible for every finite nonnegative radius.

## arithmetic_resource_partition

Validation uses simplex helper and cap. Norm overflow rejects conservatively even if projection exists; radius0 bypasses norm safely. Rounded norm can misclassify a near-boundary point. Failed magnitude or child reservation returns AllocationFailed; no allocation refusal injection.

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

Source SHA-256: 1c839d1d16d63a0336ef255a783d3141c5e070844b8f2607fbb230214f4fa650.

External logs: $MOVA_DATA/runs/octopus-wave19-projections-2026-10-30/.

- debug1.log: 295ce741ceadf13f81530f0c54bba8127ad87d2564e25440eeffc306e03b379d

- debug2.log: ab7c4a15e0d6a5e69f97f82abb191570654c81b4ff00b26644966374e1d35134

- release1.log: 2ac35cb57366af4c04df2fd89e156aba6d5754de09537cf6b2f53e9ba5627e9f

- release2.log: b4e8d171373bf4f066ec13d4bf4548f91a9abd8332101ce878f341acc741ad96

- clippy.log: 21dded34a5924937a217ba2f8c5df5d6339903a33925df0ee04c644de7533461

- restored.log: 072f31b0d0d781b3fcafe1f54ae1844ce6a172fee956f839bbe4ef93cd375a6f

- normalized.txt: 9823784f5cfb78cb5cbafeeacf4f8c9c3d144d42a95895675ac88e8a4e2c5a9f

- mutant-l1_ball_projection.log: f3814abceea056777f4bf43ee2642c50bc7e32067f4a575f2ebcd46d15fd9d06

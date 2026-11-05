# algorithm-isotonic-regression-tests

## descriptant

### Modeling

Run deterministic finite grids and boundary/fiber fixtures. Independent partition/support enumeration or KKT/subgradient stationarity compares production outputs to the mathematical model. Success is conjunction of named assertions; no universal numerical theorem follows.

Retained observation: real convex uniqueness does not make floating-point projection exact. Common large offsets require shifting for simplex; nonfinite intermediates are rejected rather than silently accepted. PAVA sums and L1 norms conservatively reject overflow. Underflow can prevent simplex mass certification. Normal-cone/subgradient fibers preserve many-to-one behavior; negative input tests alone do not model it. No timing or global objective-error claim is inferred.

## model

# isotonic_regression tests.rs

## Forward

Run deterministic finite grids and boundary/fiber fixtures. Independent partition/support enumeration or KKT/subgradient stationarity compares production outputs to the mathematical model. Success is conjunction of named assertions; no universal numerical theorem follows.

## Invariant

Fixtures and loop bounds are fixed; production errors fail ordinary-domain oracle checks. Expected invalid-domain errors are checked separately. A compiled production mutant must cause the named production test to fail, then restored bytes must pass.

## Cost

Bounded finite tests: isotonic 625 length4 vectors times 8 partitions; simplex 125 length3 vectors times 3 radii and 7 supports; L1 625 length4 vectors times 4 radii; soft 201 scalar inputs times 4 thresholds times 241 candidate objectives. Boundary allocation fixtures have 1,000,001 elements. Oracle work is test-only, not production complexity.

## behavior_complement

Within the fixed test transcript universe, pass excludes executions containing any failed assertion; fail excludes executions satisfying all checks under normal completion. Outside the finite schedules no acceptance restriction is established. Panic, allocation and infrastructure failures have distinct causes although all prevent a pass.

## reverse_reconstruction

An observed pass has a nonunique preimage: all implementations that satisfy every finite fixture, including implementations wrong elsewhere. A failed named assertion narrows the fiber to executions violating that check but cannot uniquely locate production versus oracle defects. Compiled mutants witness distinct failing implementations.

## independent_oracle

Partition enumeration minimizes squared loss without PAVA; support masks solve simplex KKT without sorting; L1 stationarity uses a recovered common multiplier; soft compares subgradients and a separate candidate-objective grid. Each is independent of its production control flow.

## adversarial_domain

Fixtures cover ties, empty input, zero parameters, nonfinite values and extreme scales. Length caps are directly tested except L1 which delegates tested validation. Allocation refusal is not forced; finite sample success is not universal numerical certification.

## mutation_observability

Mutants reverse the PAVA comparison, bias simplex threshold, erase L1 signs, or add soft threshold. Named partition_oracle, active_set_oracle, kkt_grid and subgradient_grid respectively detect them. Positive restoration and source hashes bind observations.

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

Source SHA-256: b5e9fe99507292b158b37c15aaa109b4b709c5826677be891d2ff1aed0af7f93.

External logs: $MOVA_DATA/runs/octopus-wave19-projections-2026-10-30/.

- debug1.log: 295ce741ceadf13f81530f0c54bba8127ad87d2564e25440eeffc306e03b379d

- debug2.log: ab7c4a15e0d6a5e69f97f82abb191570654c81b4ff00b26644966374e1d35134

- release1.log: 2ac35cb57366af4c04df2fd89e156aba6d5754de09537cf6b2f53e9ba5627e9f

- release2.log: b4e8d171373bf4f066ec13d4bf4548f91a9abd8332101ce878f341acc741ad96

- clippy.log: 21dded34a5924937a217ba2f8c5df5d6339903a33925df0ee04c644de7533461

- restored.log: 072f31b0d0d781b3fcafe1f54ae1844ce6a172fee956f839bbe4ef93cd375a6f

- normalized.txt: 9823784f5cfb78cb5cbafeeacf4f8c9c3d144d42a95895675ac88e8a4e2c5a9f

- mutant-isotonic_regression.log: 8dd1968a1c5f9d4d1508108860f84e291cd8fbf0af07bff46fa2752171dc04e5

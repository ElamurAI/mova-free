# algorithm-divide-conquer-dp

## descriptant

### divide_conquer_dp mod.rs

Original std-only implementation and independent fixtures. Yao (1980), Efficient dynamic programming using quadrangle inequalities; monotone divide-and-conquer recurrence.

## approximant

### Forward model

D[0,0]=0, other states unreachable; D[g,j]=min_{g-1<=k<j}(D[g-1,k]+cost(k,j)). Divide midpoint and restrict each child's candidate range by leftmost optimum. Caller guarantees monotone leftmost split in each row; pure deterministic cost callback; assumption is not checked.

### Cost

O(layers*n*log(n)) evaluations, O(n) storage. Budget checked before callback; all visited candidate sums checked (overflow fails even for a losing candidate). Impossible partitions handled before dimensions/allocation. g=0 yields 0 only for n=0.

### Invariants and limits

Owned input slices remain unchanged. Deterministic ties and traversal; callback panics propagate and purity/monotonicity remain caller obligations. No allocator-refusal injection, no semantic AST proof, no universal finite-test proof. Checked arithmetic reports errors rather than wrapping.

### Executable approximant gap

No executable surrogate is attached: categorical reachability/argmin has no justified relaxed metric here. Objective-only surrogate speed/error has not been measured. Existing knapsack surrogate is not automatically transferable. No 1/2/5/10% profile or approximate substitution is claimed.

## models

## behavior_complement

Nonempty n with zero layers excludes any finite result. For squared interval length, n=8,g=3, result 21 is excluded; optimal lengths 2,3,3 cost 22.

## reverse_reconstruction

A partition objective does not recover intervals or costs: for n=g=1 every callback with cost(0,1)=v maps to v regardless of other entries. For n=2,g=1 only cost(0,2) is observed.

## constant_shift

Adding b to every interval cost raises the feasible optimum by layers*b, provided all evaluated arithmetic fits; split ordering remains unchanged.

## positive_scale

Multiplication by positive integer preserves left argmins and scales result when all checked candidate sums fit.

## lawful_cost_family

For nonnegative prefix sums P, cost(i,j)=(P[j]-P[i])^2 is Monge; its DP layers have monotone leftmost optimal splits. Exhaustive fixtures use this premise.

## budget_prefix

At most max_evaluations callbacks occur; unreachable predecessor candidates incur no callback and no arithmetic.

## tests

### Checks

cargo test -p mova-algorithms divide_conquer_dp::tests

## combiner

### Composition

Raw mod.rs syntax and mathematical recurrence are separate evidence. The partial syntax projection binds the exact source bytes but does not certify the recurrence. Oracle tests compare observable outputs; typed relations constrain lawful transformations and compatible preimages. For divide_conquer_dp, resource caps and checked failures remain separate from mathematical objective equivalence. Prospective expectations remain unchanged; finite validation cannot turn monotonicity assumptions into runtime validation.

## validation

### Validation

Compiled production controls: zero initial subset bit, forced zero MITM objective, right-tie SMAWK reduction, forced zero layered-DP objective and forced zero BST objective all fail named equality assertions. Byte restoration followed by full positive library rerun passes. A separate SMAWK interpolation tie mutation fails lawful_matrices after adding half-integer squared-distance centers; the restored focused test passes. Receipts: controls.log, mutant-*.log, restored.log, smawk-restored.log in the same external run directory. The original constant-matrix fixture alone did not distinguish interpolation ties; the stronger fixture closes that observed gap.

Every source/test file has two independently generated identical syntax projections and current source SHA. An isolated ten-leaf modelcheck groupants run reports valid=true, diagnostics=[], ten sidecars, zero missing source sidecars; the temporary collection parent has no sidecar and makes no composition claim. Root integration owns global registry and ancestor binding checks.

No timing speedup, machine-independent resource guarantee, allocation-failure injection, monotonicity validation, or end-to-end Mova improvement is inferred. Successful finite tests certify the listed observations only. No teacher, model, corpus or live database operations were used.

## History

### 2026-10-30 · descriptant

Prospective model saved before changes in $MOVA_DATA/runs/octopus-wave20-dp-2026-10-30/prospective.md. Current epoch 2026-10-30: exact optimization with explicit bounded-resource outcomes. Finite oracle agreement does not establish the caller's monotonicity premises. No timing or whole-Mova speed claim.

### 2026-10-30 · tests

Independent small oracle and resource boundary fixtures live in ../tests.rs. Repeated debug/release and compiled mutation receipts: $MOVA_DATA/runs/octopus-wave20-dp-2026-10-30.

### 2026-10-30 · validation

Heading `### Validation — 2026-10-30` stays in validation as `### Validation`.

Ten new native tests cover five exact kernels. Full mova-algorithms library: 557 tests pass in two debug and two release executions; sorted native test outcomes agree byte-for-byte across all four. Strict all-target Clippy and focused rustfmt pass. External receipts: $MOVA_DATA/runs/octopus-wave20-dp-2026-10-30/{debug1,debug2,release1,release2,clippy}.log.

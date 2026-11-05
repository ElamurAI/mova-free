# algorithm-meet-in-middle-knapsack

## descriptant

### meet_in_middle_knapsack mod.rs

Original std-only implementation and independent fixtures. Horowitz and Sahni (1974), Computing partitions with applications to the knapsack problem.

## approximant

### Forward model

Split items in halves; enumerate feasible (weight,value) subsets, right sorted by weight then value. Prefix maxima answer max_{right.weight<=capacity-left.weight} right.value. Combine each feasible left state exactly once.

### Cost

O(n*2^ceil(n/2)) time, O(2^ceil(n/2)) states. max_states bounds sum of full half powers before allocations, even if pruning would reduce counts. Checked dimensions, u128 candidate sums; weights add only after capacity subtraction guard.

### Invariants and limits

Owned input slices remain unchanged. Deterministic ties and traversal; callback panics propagate and purity/monotonicity remain caller obligations. No allocator-refusal injection, no semantic AST proof, no universal finite-test proof. Checked arithmetic reports errors rather than wrapping.

### Executable approximant gap

No executable surrogate is attached: categorical reachability/argmin has no justified relaxed metric here. Objective-only surrogate speed/error has not been measured. Existing knapsack surrogate is not automatically transferable. No 1/2/5/10% profile or approximate substitution is claimed.

## models

## behavior_complement

For one Item(weight=2,value=7) and capacity 1, any positive objective is excluded. A returned objective excludes all larger feasible subset values.

## reverse_reconstruction

Objective 0 at capacity 0 admits [] and [(1,7)]. Positive objective 7 at capacity 2 admits [(2,7)] or [(1,3),(1,4)]. No subset identity can be uniquely recovered.

## permutation

Reordering items changes the half split and enumeration but preserves optimum when both resource checks succeed.

## zero_extension

Appending (0,v) raises optimum by exactly v, provided state budget and checked sums permit.

## capacity_monotonicity

Increasing capacity cannot decrease successful objective; the resource count is capacity-independent.

## dominance_envelope

At each sorted right index, envelope value equals the maximum raw value in that prefix. Discarding a lower value at equal or greater weight preserves every feasible left query.

## tests

### Checks

cargo test -p mova-algorithms meet_in_middle_knapsack::tests

## combiner

### Composition

Raw mod.rs syntax and mathematical recurrence are separate evidence. The partial syntax projection binds the exact source bytes but does not certify the recurrence. Oracle tests compare observable outputs; typed relations constrain lawful transformations and compatible preimages. For meet_in_middle_knapsack, resource caps and checked failures remain separate from mathematical objective equivalence. Prospective expectations remain unchanged; finite validation cannot turn monotonicity assumptions into runtime validation.

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

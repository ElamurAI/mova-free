# algorithm-bitset-subset-sum

## descriptant

### bitset_subset_sum mod.rs

Original std-only implementation and independent fixtures. Bellman, Dynamic Programming (1957); word-parallel implementation is original.

## approximant

### Forward model

B[0]=1; B_next=B OR (B<<value), clipped above target. Return bit target. Descending destination words preserve the previous item layer even for in-word shifts and zero values.

### Cost

O(m*(target/64+1)) word operations; O(target/64+1) u64 words. max_words bounds storage, checked bytes before fallible reserve.

### Invariants and limits

Owned input slices remain unchanged. Deterministic ties and traversal; callback panics propagate and purity/monotonicity remain caller obligations. No allocator-refusal injection, no semantic AST proof, no universal finite-test proof. Checked arithmetic reports errors rather than wrapping.

### Executable approximant gap

No executable surrogate is attached: categorical reachability/argmin has no justified relaxed metric here. Objective-only surrogate speed/error has not been measured. Existing knapsack surrogate is not automatically transferable. No 1/2/5/10% profile or approximate substitution is claimed.

## models

## behavior_complement

For input ([2],1), true is excluded: the reachable set is {0,2}. More generally false excludes existence of any subset summing exactly to target; true excludes absence of all witnesses.

## reverse_reconstruction

true at target 3 has compatible antecedents [3] and [1,2]; false at 3 admits [2] and []. The output fiber is all arrays with/without a subset of target weight, not a uniquely recoverable array.

## permutation

Permuting values preserves the reachable-sum set, with unchanged target and sufficient budget.

## zero_extension

Appending zero preserves every reachable target, since B OR (B<<0)=B.

## scale_equivariance

Multiplying all values and target by positive k preserves reachability when usize products and resource dimensions fit.

## single_use

For [64], target 128 is false. Descending words prevent a newly written word from supplying a second use of the item.

## tests

### Checks

cargo test -p mova-algorithms bitset_subset_sum::tests

## combiner

### Composition

Raw mod.rs syntax and mathematical recurrence are separate evidence. The partial syntax projection binds the exact source bytes but does not certify the recurrence. Oracle tests compare observable outputs; typed relations constrain lawful transformations and compatible preimages. For bitset_subset_sum, resource caps and checked failures remain separate from mathematical objective equivalence. Prospective expectations remain unchanged; finite validation cannot turn monotonicity assumptions into runtime validation.

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

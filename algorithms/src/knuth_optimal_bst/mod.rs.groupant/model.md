# algorithm-knuth-optimal-bst

## descriptant

### knuth_optimal_bst mod.rs

Original std-only implementation and independent fixtures. Knuth (1971), Optimum binary search trees; successful-search-only specialization.

## approximant

### Forward model

Successful-search BST recurrence C[i,j]=sum(w[i..j])+min_{i<=r<j}(C[i,r]+C[r+1,j]), C[i,i]=0. Nonnegative weights give root[i,j-1]<=root[i,j]<=root[i+1,j]. Scan only that interval, choosing first equal optimum.

### Cost

O(n^2) candidate scans by telescoping Knuth ranges and O(n^2) memory. max_cells bounds each (n+1)^2 table. Empty input returns before allocation; dimensions then budget then fallible allocations then checked u128 sums. Prefix array adds O(n) storage.

### Invariants and limits

Owned input slices remain unchanged. Deterministic ties and traversal; callback panics propagate and purity/monotonicity remain caller obligations. No allocator-refusal injection, no semantic AST proof, no universal finite-test proof. Checked arithmetic reports errors rather than wrapping.

### Executable approximant gap

No executable surrogate is attached: categorical reachability/argmin has no justified relaxed metric here. Objective-only surrogate speed/error has not been measured. Existing knapsack surrogate is not automatically transferable. No 1/2/5/10% profile or approximate substitution is claimed.

## models

## behavior_complement

A singleton of weight 9 excludes costs other than 9, since root depth is one. Cost below sum(weights) is impossible; all nodes have positive depth.

## reverse_reconstruction

Cost 0 admits any length all-zero weight vector, including empty. Cost 2 admits [2] and [0,1]. Output alone determines neither key count nor optimal tree.

## positive_scale

Multiplying weights by integer k>=0 scales optimum by k when input and u128 sums fit; k=0 may change chosen roots but not cost law.

## reversal

Reversing weights mirrors each feasible BST and preserves optimum, although left-tie chosen roots need not mirror.

## weight_monotonicity

Increasing one weight by delta cannot decrease optimum; for nonempty n the increase lies between delta and n*delta when arithmetic fits.

## root_interval

For nonnegative weights, neighboring optimal roots bound the current leftmost optimum; the sum of candidate interval lengths per diagonal is O(n).

## tests

### Checks

cargo test -p mova-algorithms knuth_optimal_bst::tests

## combiner

### Composition

Raw mod.rs syntax and mathematical recurrence are separate evidence. The partial syntax projection binds the exact source bytes but does not certify the recurrence. Oracle tests compare observable outputs; typed relations constrain lawful transformations and compatible preimages. For knuth_optimal_bst, resource caps and checked failures remain separate from mathematical objective equivalence. Prospective expectations remain unchanged; finite validation cannot turn monotonicity assumptions into runtime validation.

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

# algorithm-smawk

## descriptant

### smawk mod.rs

Original std-only implementation and independent fixtures. Aggarwal, Klawe, Moran, Shor and Wilber (1987), Geometric applications of a matrix-searching algorithm.

## approximant

### Forward model

Reduce columns using a stack; strict improvements pop while ties retain earlier columns. Recursively solve odd rows, then interpolate even rows between adjacent odd minima. Pure deterministic matrix callback; every submatrix has nondecreasing leftmost row minima (total monotonicity), not checked.

### Cost

O(rows+cols) callback evaluations and O(rows+cols) live storage; callback cost excluded. Each attempted callback checks max_evaluations first. Allocations use try_reserve_exact; recursive row count halves. Empty rows return before column validation, then EmptyColumns, byte dimensions, allocation, evaluation budget.

### Invariants and limits

Owned input slices remain unchanged. Deterministic ties and traversal; callback panics propagate and purity/monotonicity remain caller obligations. No allocator-refusal injection, no semantic AST proof, no universal finite-test proof. Checked arithmetic reports errors rather than wrapping.

### Executable approximant gap

No executable surrogate is attached: categorical reachability/argmin has no justified relaxed metric here. Objective-only surrogate speed/error has not been measured. Existing knapsack surrogate is not automatically transferable. No 1/2/5/10% profile or approximate substitution is claimed.

## models

## behavior_complement

For constant matrix, every nonzero returned column is excluded by left tie policy. Under total monotonicity a returned row column excludes any lower-valued column or earlier equal column.

## reverse_reconstruction

Minima vector [0,0] admits every constant 2-by-c matrix and rows [0,1,...]. Off-minimum values are not recovered; lawful matrices sharing leftmost minima form the preimage.

## row_translation

Adding an exact representable constant to each row preserves its comparisons, total monotonicity and minima.

## positive_scale

Positive exact representable integer scaling of all callback values preserves strict/equal comparisons and result.

## restriction

Restricting to ordered row and column subsets preserves total monotonicity, but minima must be recomputed among surviving columns.

## budget_prefix

An evaluation cap b invokes the callback at most b times; cap failure contains no output vector. Repeated calls to the same entry still count.

## tests

### Checks

cargo test -p mova-algorithms smawk::tests

## combiner

### Composition

Raw mod.rs syntax and mathematical recurrence are separate evidence. The partial syntax projection binds the exact source bytes but does not certify the recurrence. Oracle tests compare observable outputs; typed relations constrain lawful transformations and compatible preimages. For smawk, resource caps and checked failures remain separate from mathematical objective equivalence. Prospective expectations remain unchanged; finite validation cannot turn monotonicity assumptions into runtime validation.

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

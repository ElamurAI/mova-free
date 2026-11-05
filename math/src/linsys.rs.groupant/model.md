# math-linsys

## descriptant

### linsys

linear_form first evaluates subtrees without selected variables as exact V::Q constants, otherwise builds coefficient vector/constant recursively. Products require one zero coefficient vector; denominator must be constant and nonzero. gauss zips A,b, chooses first nonzero pivot in column order, normalizes and eliminates all other rows; inconsistent reduced rows return Inconsistent, otherwise free coordinates zero and pivot values yield Unique or Infinite. check_solution first rejects mismatched A/b counts, then any row width differing from x.len, then computes every exact dot product and collects mismatches.

## approximant

### Forward and cost

linear_form first evaluates subtrees without selected variables as exact V::Q constants, otherwise builds coefficient vector/constant recursively. Products require one zero coefficient vector; denominator must be constant and nonzero. gauss zips A,b, chooses first nonzero pivot in column order, normalizes and eliminates all other rows; inconsistent reduced rows return Inconsistent, otherwise free coordinates zero and pivot values yield Unique or Infinite. check_solution first rejects mismatched A/b counts, then any row width differing from x.len, then computes every exact dot product and collects mismatches.

Gauss-Jordan O(m*n*min(m,n)) rational operations/O(m(n+1)) storage plus coefficient growth. Checker count test O(1), width scan O(m), then O(mn) exact dot products and mismatch text; shape scan adds O(1) auxiliary space. linear_form repeated variable scans/vector work can be quadratic plus delegated constant evaluation.

No new executable Rust surrogate: exact values, branching, domains and effects require an observation-specific metric before supported 1/2/5/10% profiles. Existing floating diagnostics are not such a profile; no speed claim.

## invariant

### Domain and invariant

gauss still assumes matched rectangular dimensions and may panic/truncate on malformed input; API unchanged. check_solution now rejects mismatched count first and row width second before arithmetic. A=[]/b=[] remains true for arbitrary x, and gauss([],[]) gives Unique([]). Distinct selected variable names are assumed; duplicate names select first occurrence. Rational arithmetic is allocation-limited, constant eval delegates stateful calc. No float pivot tolerance.

linear_form first evaluates subtrees without selected variables as exact V::Q constants, otherwise builds coefficient vector/constant recursively. Products require one zero coefficient vector; denominator must be constant and nonzero. gauss zips A,b, chooses first nonzero pivot in column order, normalizes and eliminates all other rows; inconsistent reduced rows return Inconsistent, otherwise free coordinates zero and pivot values yield Unique or Infinite. check_solution first rejects mismatched A/b counts, then any row width differing from x.len, then computes every exact dot product and collects mismatches.

A solution x admits every matched rectangular A,b with Ax=b; Unique additionally needs full column rank. Infinite particular/free output does not contain nullspace basis and many systems share it. Inconsistency strings describe a reduced row without original-row provenance. linear_form loses expression syntax and constant evaluation may leave Env effects; coefficient equality alone cannot reconstruct expression/history.

## composition

### Composition

linear_form first evaluates subtrees without selected variables as exact V::Q constants, otherwise builds coefficient vector/constant recursively. Products require one zero coefficient vector; denominator must be constant and nonzero. gauss zips A,b, chooses first nonzero pivot in column order, normalizes and eliminates all other rows; inconsistent reduced rows return Inconsistent, otherwise free coordinates zero and pivot values yield Unique or Infinite. check_solution first rejects mismatched A/b counts, then any row width differing from x.len, then computes every exact dot product and collects mismatches.

calc dispatches to comb/nt. poly/linsys consume calc E and stateful Env for constants; deriv consumes E and calc::approx. fcheck consumes separate legacy expr::Expr/units, with no implicit lossless E adapter. Q/Z joins use big. Parent symbolic-computation models these edges; local checks do not certify application wiring. Nested scopes reuse parent syntax, and remaining helpers/types/tests are parent-modeled.

## models

# Typed models

## behavior_complement

For fixed admitted input/state, exclude outputs and effect traces contradicting this actual relation: linear_form first evaluates subtrees without selected variables as exact V::Q constants, otherwise builds coefficient vector/constant recursively. Products require one zero coefficient vector; denominator must be constant and nonzero. gauss zips A,b, chooses first nonzero pivot in column order, normalizes and eliminates all other rows; inconsistent reduced rows return Inconsistent, otherwise free coordinates zero and pivot values yield Unique or Infinite. check_solution first rejects mismatched A/b counts, then any row width differing from x.len, then computes every exact dot product and collects mismatches. These exclusions are conditional on the stated domain. Unavailable checks, passing residuals and resource limits do not establish global correctness.

## reverse_reconstruction

A solution x admits every matched rectangular A,b with Ax=b; Unique additionally needs full column rank. Infinite particular/free output does not contain nullspace basis and many systems share it. Inconsistency strings describe a reduced row without original-row provenance. linear_form loses expression syntax and constant evaluation may leave Env effects; coefficient equality alone cannot reconstruct expression/history.

## pivot_order

Columns and rows searched in increasing order; pivot first nonzero, exact normalization, no largest-magnitude selection.

## free_coordinate

Particular starts all zeros then fills pivots; free indices ascend, and no nullspace basis is emitted.

## shape_obligation

Checker count mismatch precedes row-width mismatch and arithmetic. Valid empty equations remain vacuously true for any x. gauss itself retains rectangular input preconditions.

## linear_boundary

Selected-variable absence precedes syntax dispatch. Product rejects after evaluating both coefficient forms; division rejects nonconstant denominator before zero constant.

## tests

### Actual-code evidence

Each selected named test passed twice debug/twice release, sorted outcomes identical:

- `test linsys::tests::unique_infinite_none ... ok`
- `test linsys::wave28::free_coordinates_and_checker_scope ... ok`

Finite synthetic fixtures only; no teacher/API/corpus/GPU/heldout. No allocation/stack faults, huge sizes, deadline proof, or universal numerical bound.

Final validation: eight compiled mutants total (seven module boundaries plus linsys width guard), all compile 0 / test 101 and exact restoration / positive 0. Old-code count mismatch witness compile 0 / test 101 separately. Full-registry structural run reports only ancestor integration bindings; isolated source snapshots validate each owned sidecar. Strict Clippy: dependency invocation blocked in existing global build script; --no-deps reaches math and reports 113 existing library findings, retained externally, no clean lint claim. Added test blocks formatted independently; production layout preserved.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read before edits, bounded actual-code tests and source controls. Linsys shape defect was witnessed on old code and repaired; other semantics retained. External receipt name octopus-wave28-mathcore-2026-10-31 includes prospective.md. Registration is partial, not verification.

### 2026-10-31 · tests

Heading `### Actual-code evidence 2026-10-31` stays in tests as `### Actual-code evidence`.

Compiled source control: control-linsys.patch.txt; compiler exit0, test nonzero, exact source restoration and focused positive exit0. Full logs and numeric exits retained externally in octopus-wave28-mathcore-2026-10-31. Linsys also retains shape-old compile 0 / test 101 witness before shape repair. Seven module controls plus the additional linsys width-guard control; fragment declarations are not extra tests.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

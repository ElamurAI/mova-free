# mlab-numerics-numeric-trapezoid

## descriptant

### mlab-numerics-numeric-trapezoid

trapz chooses row-vector or column slices; optional scalar X is uniform step, other X must have slice length. Each output sums (X[t]-X[t-1])*(Y[t]+Y[t-1])/2 or default spacing1.

## approximant

### Forward relation

trapz chooses row-vector or column slices; optional scalar X is uniform step, other X must have slice length. Each output sums (X[t]-X[t-1])*(Y[t]+Y[t-1])/2 or default spacing1.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Output scalar for one slice, row for multiple; zero/one-length slices accumulate0. Nonmonotone X gives signed widths without error.

#### Cost

O(N) sampled cells, O(columns) output.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-numeric-trapezoid

## behavior_complement

Wrong non-scalar X length excludes accumulation. A single sample excludes nonzero contribution regardless scalar step. Imaginary components cannot influence this real-only implementation.

## reverse_reconstruction

For fixed X, output defines one linear weighted-sum constraint per Y slice; arbitrary perturbations in its nullspace produce same result. Uniform scaling of X and inverse scaling of Y preserve integral absent rounding.

## spacing_units

trapz(2,[1,3,5])=2*(1+3)/2+2*(3+5)/2=12.

## length_gate

X length2 with Y row length3 errors before data indexing.

## orientation

row1 uses cols as length and one output; other matrices integrate columns.

## complex_projection

Only re vectors are read, so this scope does not implement complex integration.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

# mlab-numerics-reduce

## descriptant

### mlab-numerics-reduce

Slice geometry maps dim1 to columns, dim2 to rows, greater dims to singleton cells. sum/prod fold complex values with identities0/1; mean divides by slice length. cumulative and executed nonempty differences, plus several statistics, use real data only. diff order0 returns the original cloned matrix; diff1 zero-length differentiated dimension also clones. minmax selects first strict improving magnitude/real key; sort emits stable slice positions; find emits 1-based column-major indices; unique sorts real values and maps original positions.

## approximant

### Forward relation

Slice geometry maps dim1 to columns, dim2 to rows, greater dims to singleton cells. sum/prod fold complex values with identities0/1; mean divides by slice length. cumulative and executed nonempty differences, plus several statistics, use real data only. diff order0 returns the original cloned matrix; diff1 zero-length differentiated dimension also clones. minmax selects first strict improving magnitude/real key; sort emits stable slice positions; find emits 1-based column-major indices; unique sorts real values and maps original positions.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

1-based indices and column-major slices are API coordinates. Mean of empty0x0 is NaN. Shaped empty minmax can index empty slice and is an explicit unchecked gap.

#### Cost

Slice construction/folds/cumulative/find/diff cost O(N+S), where N=numel and S=number of slices; shaped empties can have N=0,S>0. Sort/median cost O(N log(max(L,2))+S), unique O(N log(max(N,2))). Slices retain O(N+S) indices/vector headers; outputs and sorting copies also include S overhead. Repeated diff O(order*(N+S)), with order0 clone passthrough.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## composition

### Composition

Registry closures route through shared slices with default_dim. minmax's two-argument form delegates elementwise broadcasting to ops::map2; its reduction path instead returns local slice indices. Consumers must distinguish these coordinate systems.

Fragments refer to disjoint useful source scopes in the single parent lexical projection. Registration and inline tests remain parent-modeled. Source callbacks and external backend implementations are dependencies, not duplicated ownership. Local tolerances cannot be composed into a subsystem percent guarantee.

## models

# Typed source models: mlab-numerics-reduce

## behavior_complement

Equal minmax keys exclude replacing the earliest winner. Finite candidate excludes retaining initial NaN best. sum/prod empty0x0 default exclude empty output and return identity. No uniform imaginary preservation across reductions.

## reverse_reconstruction

Sum fibers contain per-slice zero-sum perturbations; extrema plus index constrain all other keys to not exceed selected key with earlier ties excluded. sort result plus index permutation reconstructs cells; unique values alone lose multiplicity/order, inverse map can reconstruct real input.

## tie_and_nan

minmax [NaN,4,4] chooses value4 at position2; sort ascending places NaNs last, descending first.

## empty_identities

sum([])=0,prod([])=1,any([])=false,all([])=true; mean/median/variance=[]→NaN.

## complex_projection

sum/prod and extrema preserve selected imaginary parts; cumulative and unique read re. diff order0 returns original cloned Mat with im; positive iterations use real subtraction unless a zero differentiated dimension causes clone passthrough.

## index_transport

find indices k+1; row k%rows+1,col k/rows+1. Sort indices are positions within each slice, not flattened global positions.


## tests

### Executed evidence

builtins::reduce::wave30_tests::geometry_empty_and_ties: column sums, empty product, NaN/first-tie index and column differences.

#### Native selection

    cargo test --offline --locked -p mlab --lib builtins::reduce::wave30_tests::geometry_empty_and_ties -- --exact

Each selected test passed twice in debug and twice in release with numeric exit 0. Eight aggregate named outcomes were extracted from actual logs, sorted, and identical across all four runs. Post-control exact-restoration runs passed both profiles with separately retained numeric exits.

#### Compiled negative control

In this source, the expression

    if is_max { x > best } else { x < best }

was temporarily replaced by

    if is_max { x >= best } else { x <= best }

Cargo compiled successfully and the selected test reached an assertion failure with exit 101. The complete original source bytes were restored and compared before positive reruns. This control validates the selected observation, not all branches.

#### Scope and limits

Parent and fragment models distinguish executed fixtures from source-only branches. No benchmark main, held-out datasets, network, teacher or GPU run. No numerical approximation surrogate, exhaustive convergence proof, allocation-failure injection or speed claim. Strict Clippy was not rerun for this test/model-only change; added test blocks were individually rustfmt-formatted without changing production formatting. External receipts retain native commands, test maps, logs, exits, mutations and structural checks.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

### 2026-10-31 · tests

Heading `### Executed evidence — 2026-10-31` stays in tests as `### Executed evidence`.

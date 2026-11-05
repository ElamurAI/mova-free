# mlab-ops

## descriptant

### Broadcast, matrix arithmetic and finite ranges

map2 uses equal-shape zip, then scalar fast paths, else singleton-dimension broadcast with k=(cols==1?0:j)*rows+(rows==1?0:i). matmul delegates scalars to EMul, checks inner dimensions, uses small loops or faer at m*n*k>=32768; complex large products combine four real products. range_spec computes n=floor((b-a)/s+3eps*max(abs(span),1))+1 with endpoint snapping and evaluates from nearer end. Concatenation discards empty pieces for shape matching, but includes classes in result class choice.

## approximant

### Broadcast, matrix arithmetic and finite ranges forward relation

map2 uses equal-shape zip, then scalar fast paths, else singleton-dimension broadcast with k=(cols==1?0:j)*rows+(rows==1?0:i). matmul delegates scalars to EMul, checks inner dimensions, uses small loops or faer at m*n*k>=32768; complex large products combine four real products. range_spec computes n=floor((b-a)/s+3eps*max(abs(span),1))+1 with endpoint snapping and evaluates from nearer end. Concatenation discards empty pieces for shape matching, but includes classes in result class choice.

Domain: Well-formed bounded Mat; finite modest range spans fitting usize count and memory; f64 overflow/NaN otherwise follows individual branch, no uniform numerical bound. Linalg callbacks can warn through Interp; no GPU required.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Arithmetic outputs double, comparisons/logical outputs logical; complex constructors narrow all-zero imaginary parts. truthy is nonempty and all complex elements nonzero. RangeSpec::at only modeled for k<n; dimension products/count conversion are unchecked. Scalar zero powers use cpow integer branch before zero-base fallback.

Cost: Broadcast O(output cells), small matmul O(mnk), matrix power O(log |p|) products plus inverse for negative p; large products delegate faer. Materialized range O(n), lazy at O(1). Concat allocates/clones linear total cells. No measured speed claim.

Failure ordering: binary handles strings before matrix extraction; left mat extraction before right; scalar division before shape/linalg. mpower scalar/scalar before square integer path. map2 equal shape/scalars before bshape rows then cols; range checks invalid direction/NaN/zero before count arithmetic.

## composition

### Parent composition

parser distinguishes matrix and elementwise operators; interp owns short-circuit &&/|| rather than this binary entrypoint. value storage and index output shape determine broadcast. linalg owns solves/inverse; core colon delegates range while reshape preserves storage rather than transpose.

Scoped fragments are projections over exact byte intervals in this source; they share one parent syntax projection. Uncovered declarations and embedded checks remain parent modeled. binary handles strings before matrix extraction; left mat extraction before right; scalar division before shape/linalg. mpower scalar/scalar before square integer path. map2 equal shape/scalars before bshape rows then cols; range checks invalid direction/NaN/zero before count arithmetic.

The relation is source-derived and finite checks cannot establish complete correctness or a composition-wide numerical error bound.

## models

## behavior_complement

Bounded broadcast excludes incompatible nonsingleton dimensions and incorrect column-major ordering. Wrong-direction/zero-step/NaN ranges exclude elements. Small real matmul skips zero RHS coefficients, therefore does not guarantee dense IEEE NaN*0 propagation. Comparisons < <= > >= ignore imaginary parts; no mathematical complex order is claimed.

## reverse_reconstruction

Broadcast result preimage includes all operand shapes allowed by singleton expansion and values satisfying per-cell operator relation; addition admits many decompositions. Matmul preimages are factorization relations, nonunique with nullspaces. Range vectors constrain start/step/endpoint up to floating rounding and snapping; empty ranges have many causes.

## representation_transport

Arithmetic outputs double, comparisons/logical outputs logical; complex constructors narrow all-zero imaginary parts. truthy is nonempty and all complex elements nonzero. RangeSpec::at only modeled for k<n; dimension products/count conversion are unchecked. Scalar zero powers use cpow integer branch before zero-base fallback.

## failure_precedence

binary handles strings before matrix extraction; left mat extraction before right; scalar division before shape/linalg. mpower scalar/scalar before square integer path. map2 equal shape/scalars before bshape rows then cols; range checks invalid direction/NaN/zero before count arithmetic.

## resource_domain

Well-formed bounded Mat; finite modest range spans fitting usize count and memory; f64 overflow/NaN otherwise follows individual branch, no uniform numerical bound. Linalg callbacks can warn through Interp; no GPU required.

## interaction_relation

parser distinguishes matrix and elementwise operators; interp owns short-circuit &&/|| rather than this binary entrypoint. value storage and index output shape determine broadcast. linalg owns solves/inverse; core colon delegates range while reshape preserves storage rather than transpose.


## tests

### Source-specific bounded evidence

`ops::wave30::arithmetic`: column-major broadcast, empty truth, small complex product, finite ranges, concat shape error, zero-skipping NaN behavior.

Unexecuted here: No large faer, solve/power, strings, nonfinite range resource execution. No corpus, GPU, live filesystem, timer or callback run. No broad Clippy claim; dependency/style checks were not repeated. Only appended test blocks were rustfmt-formatted; original production bytes are unchanged.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: parser distinguishes matrix and elementwise operators; interp owns short-circuit &&/|| rather than this binary entrypoint. value storage and index output shape determine broadcast. linalg owns solves/inverse; core colon delegates range while reshape preserves storage rather than transpose.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

### 2026-10-31 · tests

Seven named tests each passed twice debug and twice release on 2026-10-31, with actual named log lines sorted and identical across all four runs (28 selected successful executions). Each source has a compiled changed-assertion negative control; six runtime sources additionally have compiled production-body mutants that fail those unchanged tests. AST has representation evidence only. Every mutant is byte-restored and its positive test rerun. See `evidence.json` for this source's exact hash, mutation and exit statuses. External receipts retain full logs, command metadata, control records and prospective notes.

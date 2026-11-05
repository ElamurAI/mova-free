# mlab-ops-broadcast

## descriptant

### Broadcast shape and element maps

For each axis result equals equal extents, otherwise other extent when one is 1; otherwise Err. map2 chooses equal-shape zipped vectors, then scalar vector map, then j-major broadcast using bidx. cmap2 uses component tuples and narrows final complex result.

## approximant

### Broadcast shape and element maps forward relation

For each axis result equals equal extents, otherwise other extent when one is 1; otherwise Err. map2 chooses equal-shape zipped vectors, then scalar vector map, then j-major broadcast using bidx. cmap2 uses component tuples and narrows final complex result.

Domain: Well-formed bounded numeric buffers; closure may produce nonfinite output. No allocation failure certificate.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Output axes follow bshape; no input mutation. Scalar paths preserve nonscalar shape including empty dimensions.

Cost: O(output cells), real map equal-shape avoids shape loops but no speed measurement.

Failure ordering: Equal shapes before right scalar before left scalar before bshape; bshape validates rows before columns.

## models

## behavior_complement

2×3 with 4×3 excludes success; 2×1 with 1×3 excludes output shape 1×1 or row-major storage. Well-formed requirement excludes relying on zip truncation as shape validation.

## reverse_reconstruction

For addition output Y, any compatible A/B satisfying Yij=Aexpandij+Bexpandij is antecedent; singleton axes add shape ambiguity. Shape alone cannot recover scalar fast path inputs.

## representation_transport

Output axes follow bshape; no input mutation. Scalar paths preserve nonscalar shape including empty dimensions.

## failure_precedence

Equal shapes before right scalar before left scalar before bshape; bshape validates rows before columns.

## resource_domain

Well-formed bounded numeric buffers; closure may produce nonfinite output. No allocation failure certificate.

## interaction_relation

elementwise supplies closures/classes; value::Mat::complex may drop zero im after cmap2, affecting representation preimage.


## tests

### Bounded evidence

Selected actual-code Rust test: `ops::wave30::arithmetic` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: elementwise supplies closures/classes; value::Mat::complex may drop zero im after cmap2, affecting representation preimage.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

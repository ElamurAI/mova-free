# mlab-index-assignment

## descriptant

### Indexed assignment validation and growth

Deletion routes first. Nondelete one-index assignment checks RHS scalar or matching count before vector-only growth; two-index assignment converts row/col selections, verifies RHS rectangle/vector compatibility, computes merged class before resize, resizes, installs class and writes the j-major index list. Duplicate writes overwrite in order.

## approximant

### Indexed assignment validation and growth forward relation

Deletion routes first. Nondelete one-index assignment checks RHS scalar or matching count before vector-only growth; two-index assignment converts row/col selections, verifies RHS rectangle/vector compatibility, computes merged class before resize, resizes, installs class and writes the j-major index list. Duplicate writes overwrite in order.

Domain: Valid bounded matrices, supported args and representable indices; allocation failure/panic not covered by ordinary Result rollback.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Resize copies old overlap using old/new leading row dimensions. class computed before 2D resize; scalar broadcast uses RHS index zero. write clears prior imaginary cells when RHS real.

Cost: O(indices+grown area+old overlap); memory O(grown area+indices).

Failure ordering: Extra dimension rejection before 2D conversion; count/shape before growth; writes after successful resize. Empty target colon dimensions inferred from RHS.

## models

## behavior_complement

Bounded validation Err excludes target mutation; general matrix linear out-of-range cannot grow. Nonempty logical target with nonlogical RHS cannot remain logical; defined nonempty char target retains Char; empty or undefined target adopts the RHS class.

## reverse_reconstruction

Final target constrains untouched retained cells; overwritten old values are arbitrary. Repeated indices only constrain final RHS occurrence, earlier values lost. Growth zero-filled holes cannot identify whether original target already held zeros.

## representation_transport

Resize copies old overlap using old/new leading row dimensions. class computed before 2D resize; scalar broadcast uses RHS index zero. write clears prior imaginary cells when RHS real.

## failure_precedence

Extra dimension rejection before 2D conversion; count/shape before growth; writes after successful resize. Empty target colon dimensions inferred from RHS.

## resource_domain

Valid bounded matrices, supported args and representable indices; allocation failure/panic not covered by ordinary Result rollback.

## interaction_relation

Uses to_indices and Mat::narrow. Interp decides defined flag and resolves End before call; absent variable differs from existing nonempty target class.


## tests

### Bounded evidence

Selected actual-code Rust test: `index::wave30::gather_update` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: Uses to_indices and Mat::narrow. Interp decides defined flag and resolves End before call; absent variable differs from existing nonempty target class.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

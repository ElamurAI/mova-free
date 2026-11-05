# mlab-index-deletion

## descriptant

### Null assignment deletion

Linear colon clears to 0x0 with class retained. Numeric deletion builds boolean membership, gathers complement, and retains column orientation only for nonrow column source. 2D deletion accepts complete row or column selection and removes selected opposite-axis entries.

## approximant

### Null assignment deletion forward relation

Linear colon clears to 0x0 with class retained. Numeric deletion builds boolean membership, gathers complement, and retains column orientation only for nonrow column source. 2D deletion accepts complete row or column selection and removes selected opposite-axis entries.

Domain: Nonzero valid argument count; bounded dimensions and valid storage. Higher-dimensional deletion validation is incomplete.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Membership deduplicates; out-of-bounds selections return Err before target replacement by caller. Complete-axis recognition sorts/dedups numeric selections.

Cost: O(old cells+indices) and O(k log k) complete-axis checks; gather allocates survivors.

Failure ordering: Linear colon special-case precedes numeric conversion; 2D full-row branch precedes full-column branch, then failure.

## models

## behavior_complement

Duplicate deletion cannot remove a cell twice. Partial row and column selections exclude success with message one non-colon index. Zero argument deletion is not supported and can index args out of bounds.

## reverse_reconstruction

Survivor matrix plus selection constrains all retained cells; deleted cells free and removal duplicates unrecoverable. Clearing loses dimensions/values except class.

## representation_transport

Membership deduplicates; out-of-bounds selections return Err before target replacement by caller. Complete-axis recognition sorts/dedups numeric selections.

## failure_precedence

Linear colon special-case precedes numeric conversion; 2D full-row branch precedes full-column branch, then failure.

## resource_domain

Nonzero valid argument count; bounded dimensions and valid storage. Higher-dimensional deletion validation is incomplete.

## interaction_relation

assign dispatches only exact 0x0 RHS here; gather narrows im, so all-zero imaginary representation may be lost.


## tests

### Bounded evidence

Selected actual-code Rust test: `index::wave30::gather_update` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: assign dispatches only exact 0x0 RHS here; gather narrows im, so all-zero imaginary representation may be lost.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

# mlab-index

## descriptant

### One-based indexing and transactional bounded assignment

to_indices converts positive integral finite numeric x to usize(x)-1, or logical nonzero positions; colon spans extent. read validates upper bounds and gathers column-major; one vector index on vector source preserves source orientation, colon gives a column. assign validates shape then resizes with zero fill and writes ordered indices; duplicate destinations use the final RHS occurrence. 0x0 RHS selects deletion, deduplicating removal positions.

## approximant

### One-based indexing and transactional bounded assignment forward relation

to_indices converts positive integral finite numeric x to usize(x)-1, or logical nonzero positions; colon spans extent. read validates upper bounds and gathers column-major; one vector index on vector source preserves source orientation, colon gives a column. assign validates shape then resizes with zero fill and writes ordered indices; duplicate destinations use the final RHS occurrence. 0x0 RHS selects deletion, deduplicating removal positions.

Domain: Well-formed Mat, args lengths supported by each branch, finite indices representable below usize limit and bounded allocatable dimensions. Extra dimensions must be singleton; zero-argument deletion is outside modeled success domain.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

All coordinates exposed to language are one-based; internal indices zero-based, layout j*rows+i. Ordinary Err from bounded validation precedes writes. Growth preserves old rectangle and zeros new cells. Empty deletion is only shape 0x0, not every zero-numel shape.

Cost: Read O(index elements+output); resize O(new area+old overlap); assign O(index+new area+RHS), deletion O(old area+index) with row/col completeness sorting O(k log k). No fallible allocation or checked dimension products.

Failure ordering: assign routes deletion before argument arity; extra-dimensional checks precede assign_2d. Numeric validity precedes read bound checks. Linear conformity precedes growth permission; 2D index conversion then RHS conformity precedes resize. read converts first two indices before row bounds.

## composition

### Parent composition

interp turns Colon and End AST into IdxArg/extents. value supplies column-major Mat; reads narrow imaginary zeros; class merge preserves defined nonempty target Char, retains Logical only for logical/logical, otherwise Double; empty or undefined target adopts RHS class.

Scoped fragments are projections over exact byte intervals in this source; they share one parent syntax projection. Uncovered declarations and embedded checks remain parent modeled. assign routes deletion before argument arity; extra-dimensional checks precede assign_2d. Numeric validity precedes read bound checks. Linear conformity precedes growth permission; 2D index conversion then RHS conformity precedes resize. read converts first two indices before row bounds.

The relation is source-derived and finite checks cannot establish complete correctness or a composition-wide numerical error bound.

## models

## behavior_complement

Within well-formed representable bounded domains, rejected conformity excludes changes to target, and duplicate index writes exclude first-write-wins. Reads exclude silently clipping out-of-bound positions. Logical masks are not validated against extent until read. No claim excludes panic on zero-argument delete, oversized casts, malformed buffers or allocation failure.

## reverse_reconstruction

Gather output defines constraints m[idx[p]]=out[p], leaving unselected cells free; repeated index outputs must agree, and narrow may erase zero-im representation. Assignment preimage consists of prior matrices agreeing on unwritten retained cells; overwritten cells arbitrary. Deletion loses removed values and duplicate deletion multiplicity.

## representation_transport

All coordinates exposed to language are one-based; internal indices zero-based, layout j*rows+i. Ordinary Err from bounded validation precedes writes. Growth preserves old rectangle and zeros new cells. Empty deletion is only shape 0x0, not every zero-numel shape.

## failure_precedence

assign routes deletion before argument arity; extra-dimensional checks precede assign_2d. Numeric validity precedes read bound checks. Linear conformity precedes growth permission; 2D index conversion then RHS conformity precedes resize. read converts first two indices before row bounds.

## resource_domain

Well-formed Mat, args lengths supported by each branch, finite indices representable below usize limit and bounded allocatable dimensions. Extra dimensions must be singleton; zero-argument deletion is outside modeled success domain.

## interaction_relation

interp turns Colon and End AST into IdxArg/extents. value supplies column-major Mat; reads narrow imaginary zeros; class merge preserves defined nonempty target Char, retains Logical only for logical/logical, otherwise Double; empty or undefined target adopts RHS class.


## tests

### Source-specific bounded evidence

`index::wave30::gather_update`: vector orientation, shape-error rollback, zero-filled growth, duplicate writes/deletes, colon column, NaN index rejection, logical masks.

Unexecuted here: 2D growth/deletion and huge-index panic/resource cases unexecuted. No corpus, GPU, live filesystem, timer or callback run. No broad Clippy claim; dependency/style checks were not repeated. Only appended test blocks were rustfmt-formatted; original production bytes are unchanged.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: interp turns Colon and End AST into IdxArg/extents. value supplies column-major Mat; reads narrow imaginary zeros; class merge preserves defined nonempty target Char, retains Logical only for logical/logical, otherwise Double; empty or undefined target adopts RHS class.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

### 2026-10-31 · tests

Seven named tests each passed twice debug and twice release on 2026-10-31, with actual named log lines sorted and identical across all four runs (28 selected successful executions). Each source has a compiled changed-assertion negative control; six runtime sources additionally have compiled production-body mutants that fail those unchanged tests. AST has representation evidence only. Every mutant is byte-restored and its positive test rerun. See `evidence.json` for this source's exact hash, mutation and exit statuses. External receipts retain full logs, command metadata, control records and prospective notes.

# mlab-value

## descriptant

### Column-major value storage

Mat stores rows,cols,re and optional im plus class. at(i,j)=re[j*rows+i]. transpose maps destination i*cols+j from source j*rows+i, swaps dimensions and preserves class. narrow removes im iff every entry equals zero (including empty). StrArr records missing strings as None; Exact has no variants.

## approximant

### Column-major value storage forward relation

Mat stores rows,cols,re and optional im plus class. at(i,j)=re[j*rows+i]. transpose maps destination i*cols+j from source j*rows+i, swaps dimensions and preserves class. narrow removes im iff every entry equals zero (including empty). StrArr records missing strings as None; Exact has no variants.

Domain: Bounded valid dimensions/buffers and available allocation; arbitrary f64 values allowed. Char scalar decoding uses Rust casts and Unicode replacement. No filesystem, RNG or GPU.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Well-formed means len(re)=rows*cols and optional im same length; this is caller obligation. Empty Mat row has 1x0 while empty StrArr row is 0x0, and StrArr col is 0x1. numel multiplication has no checked overflow. NaN imaginary entries prevent narrowing.

Cost: Transpose, char conversion and imaginary zero materialization O(rows*cols) time/storage; narrow O(im.len); scalar/shape predicates O(1). Rc value clones share functions/tables/string arrays; Mat clones own vectors.

Failure ordering: new executes debug length assertion then construction; complex constructs then narrow without explicit length validation. Indexed access may panic on malformed buffers. Lossy string traversal is row by row over column-major storage.

## composition

### Parent composition

index and ops depend on this storage invariant; core reshape changes dimensions without buffer permutation, unlike transpose. AST Rc anonymous bodies become Func::Anon with captured Value entries; pointer sharing differs from structural equality.

Scoped fragments are projections over exact byte intervals in this source; they share one parent syntax projection. Uncovered declarations and embedded checks remain parent modeled. new executes debug length assertion then construction; complex constructs then narrow without explicit length validation. Indexed access may panic on malformed buffers. Lossy string traversal is row by row over column-major storage.

The relation is source-derived and finite checks cannot establish complete correctness or a composition-wide numerical error bound.

## models

## behavior_complement

For well-formed bounded matrices transpose excludes row-major reinterpretation, dropping nonzero/NaN imaginary data, or class change. narrow excludes Some(all-zero) afterward. Public fields and complex constructor do not exclude inconsistent lengths; new checks re length only in debug.

## reverse_reconstruction

Transpose is its own inverse on well-formed Mat, including optional-im representation. narrow's None fiber contains prior None and any all-zero imaginary buffer; allocation identity and signed imaginary zero are lost. Char decoding's replacement/saturating conversions lose original f64 payloads.

## representation_transport

Well-formed means len(re)=rows*cols and optional im same length; this is caller obligation. Empty Mat row has 1x0 while empty StrArr row is 0x0, and StrArr col is 0x1. numel multiplication has no checked overflow. NaN imaginary entries prevent narrowing.

## failure_precedence

new executes debug length assertion then construction; complex constructs then narrow without explicit length validation. Indexed access may panic on malformed buffers. Lossy string traversal is row by row over column-major storage.

## resource_domain

Bounded valid dimensions/buffers and available allocation; arbitrary f64 values allowed. Char scalar decoding uses Rust casts and Unicode replacement. No filesystem, RNG or GPU.

## interaction_relation

index and ops depend on this storage invariant; core reshape changes dimensions without buffer permutation, unlike transpose. AST Rc anonymous bodies become Func::Anon with captured Value entries; pointer sharing differs from structural equality.


## tests

### Source-specific bounded evidence

`value::wave30::layout`: rectangular transpose/involution, imaginary preservation/narrowing, NaN, missing string shapes.

Unexecuted here: Malformed public buffers and allocation failure untested. No corpus, GPU, live filesystem, timer or callback run. No broad Clippy claim; dependency/style checks were not repeated. Only appended test blocks were rustfmt-formatted; original production bytes are unchanged.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: index and ops depend on this storage invariant; core reshape changes dimensions without buffer permutation, unlike transpose. AST Rc anonymous bodies become Func::Anon with captured Value entries; pointer sharing differs from structural equality.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

### 2026-10-31 · tests

Seven named tests each passed twice debug and twice release on 2026-10-31, with actual named log lines sorted and identical across all four runs (28 selected successful executions). Each source has a compiled changed-assertion negative control; six runtime sources additionally have compiled production-body mutants that fail those unchanged tests. AST has representation evidence only. Every mutant is byte-restored and its positive test rerun. See `evidence.json` for this source's exact hash, mutation and exit statuses. External receipts retain full logs, command metadata, control records and prospective notes.

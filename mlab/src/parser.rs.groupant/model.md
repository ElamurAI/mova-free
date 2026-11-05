# mlab-parser

## descriptant

### Recursive precedence and statement parser

parse_program lexes then descends statements. Functions partition into Program.functions. LHS probing restores position when no assignment follows. Power is left-associated: 2^3^2 maps Pow(Pow(2,3),2); unary wraps power: -2^2 maps Neg(Pow(2,2)). Matrix whitespace before unspaced sign starts a new element; end is special only with positive index_depth.

## approximant

### Recursive precedence and statement parser forward relation

parse_program lexes then descends statements. Functions partition into Program.functions. LHS probing restores position when no assignment follows. Power is left-associated: 2^3^2 maps Pow(Pow(2,3),2); unary wraps power: -2^2 maps Neg(Pow(2,2)). Matrix whitespace before unspaced sign starts a new element; end is special only with positive index_depth.

Domain: Lexer-accepted token stream; recursive stack/allocation resources. No external files/evaluation. Function bodies may terminate at EOF/next function; ordinary blocks require matching terminator.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Ordered statement bodies, explicit Paren nodes, optional range step and index arguments survive parsing. Empty input gives empty Program. Anonymous body resets index_depth and restores matrix context only on successful descent; failed parser state is discarded by parse_program.

Cost: O(T) ordinary descent plus speculative LHS parse/reparse; deeply nested indexed LHS probes can add repeated subexpression work, so no uniform linear proof. O(T) retained AST plus recursion depth; no nesting cap.

Failure ordering: Whole lexing precedes syntax errors. Statement keyword/command/increment dispatch precedes LHS probing then expression fallback. Missing block terminator reports first expected term; unsupported global/persistent fails before runtime.

## composition

### Parent composition

lexer space_before and Newline feed matrix and statement boundaries; AST tags are consumed by interp/vm. end resolution belongs to interpreter index context, not a parser numeric substitution; n-dimensional shape checks remain downstream.

Scoped fragments are projections over exact byte intervals in this source; they share one parent syntax projection. Uncovered declarations and embedded checks remain parent modeled. Whole lexing precedes syntax errors. Statement keyword/command/increment dispatch precedes LHS probing then expression fallback. Missing block terminator reports first expected term; unsupported global/persistent fails before runtime.

The relation is source-derived and finite checks cannot establish complete correctness or a composition-wide numerical error bound.

## models

## behavior_complement

The accepted parse relation excludes right-associated 2^3^2, flattening [1 -2] to subtraction, accepting naked end as expression, or returning a partially built Program on Result::Err. It does not exclude semantically invalid break, unknown function, public invalid AST or runtime division by zero.

## reverse_reconstruction

P^-1(tree) contains texts sharing AST tags/literals and retained statement line/print flags; comments and many separators are ambiguous. For Pow(Pow(2,3),2), 2^3^2 is compatible; 2^(3^2) differs through Paren and tree shape. An evaluated scalar has much larger fibers.

## representation_transport

Ordered statement bodies, explicit Paren nodes, optional range step and index arguments survive parsing. Empty input gives empty Program. Anonymous body resets index_depth and restores matrix context only on successful descent; failed parser state is discarded by parse_program.

## failure_precedence

Whole lexing precedes syntax errors. Statement keyword/command/increment dispatch precedes LHS probing then expression fallback. Missing block terminator reports first expected term; unsupported global/persistent fails before runtime.

## resource_domain

Lexer-accepted token stream; recursive stack/allocation resources. No external files/evaluation. Function bodies may terminate at EOF/next function; ordinary blocks require matching terminator.

## interaction_relation

lexer space_before and Newline feed matrix and statement boundaries; AST tags are consumed by interp/vm. end resolution belongs to interpreter index context, not a parser numeric substitution; n-dimensional shape checks remain downstream.


## tests

### Source-specific bounded evidence

`parser::wave30::precedence`: left power association, unary precedence, matrix whitespace, End context, missing block terminator, empty program.

Unexecuted here: No exhaustive statement/function/control grammar. No corpus, GPU, live filesystem, timer or callback run. No broad Clippy claim; dependency/style checks were not repeated. Only appended test blocks were rustfmt-formatted; original production bytes are unchanged.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: lexer space_before and Newline feed matrix and statement boundaries; AST tags are consumed by interp/vm. end resolution belongs to interpreter index context, not a parser numeric substitution; n-dimensional shape checks remain downstream.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

### 2026-10-31 · tests

Seven named tests each passed twice debug and twice release on 2026-10-31, with actual named log lines sorted and identical across all four runs (28 selected successful executions). Each source has a compiled changed-assertion negative control; six runtime sources additionally have compiled production-body mutants that fail those unchanged tests. AST has representation evidence only. Every mutant is byte-restored and its positive test rerun. See `evidence.json` for this source's exact hash, mutation and exit statuses. External receipts retain full logs, command metadata, control records and prospective notes.

# mlab-ast

## descriptant

### Expression and statement representation

Expr is a tagged recursive sum: Num/Imag retain value and spelling, Str retains quote mode, Binary retains BinOp, Range retains optional step, Index retains argument order, AnonFn retains Rc body. Stmt stores one-based lexer line plus print/op metadata. Program partitions top-level statements and Rc function definitions.

## approximant

### Expression and statement representation forward relation

Expr is a tagged recursive sum: Num/Imag retain value and spelling, Str retains quote mode, Binary retains BinOp, Range retains optional step, Index retains argument order, AnonFn retains Rc body. Stmt stores one-based lexer line plus print/op metadata. Program partitions top-level statements and Rc function definitions.

Domain: Finite enum trees and ordered vectors; f64 payload permits NaN/Infinity. Public construction does not certify well-formed programs. No I/O or GPU effects.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Cloning Expr preserves payload and child order; Rc clones share anonymous bodies. Program::default has empty body/functions. Operator text uses != for Ne, which collapses source spellings ~= and != after lexing.

Cost: Clone traverses owned tree vectors/boxes and strings, O(owned nodes+text), but Rc body/function references clone O(1). text is O(1). No recursion depth cap.

Failure ordering: There is no fallible AST validation API. Allocation/recursive clone can fail externally; text exhaustively maps BinOp. Representation checks must not be reported as execution coverage.

## composition

### Parent composition

parser constructs these nodes; interp and vm consume them. Preserving End versus Ident(end), matrix rows and Pow versus EPow is required at that join; flattening any of them loses semantics.

Scoped fragments are projections over exact byte intervals in this source; they share one parent syntax projection. Uncovered declarations and embedded checks remain parent modeled. There is no fallible AST validation API. Allocation/recursive clone can fail externally; text exhaustively maps BinOp. Representation checks must not be reported as execution coverage.

The relation is source-derived and finite checks cannot establish complete correctness or a composition-wide numerical error bound.

## models

## behavior_complement

The representation relation excludes clone changing tags, ordered children, names or literal payload. BinOp::Pow and EPow are distinct even where scalar execution agrees. It does not exclude a hand-constructed invalid assignment or nonsensical line number; enums do not enforce grammar or evaluation.

## reverse_reconstruction

Given an Expr, compatible source strings are all texts parsed to its retained tags/payload; whitespace/comments are not uniquely recoverable. Clone's antecedent is any structurally equal tree; Rc anonymous-body cloning additionally retains pointer identity, not a fresh allocation. Runtime values cannot recover arbitrary syntax.

## representation_transport

Cloning Expr preserves payload and child order; Rc clones share anonymous bodies. Program::default has empty body/functions. Operator text uses != for Ne, which collapses source spellings ~= and != after lexing.

## failure_precedence

There is no fallible AST validation API. Allocation/recursive clone can fail externally; text exhaustively maps BinOp. Representation checks must not be reported as execution coverage.

## resource_domain

Finite enum trees and ordered vectors; f64 payload permits NaN/Infinity. Public construction does not certify well-formed programs. No I/O or GPU effects.

## interaction_relation

parser constructs these nodes; interp and vm consume them. Preserving End versus Ident(end), matrix rows and Pow versus EPow is required at that join; flattening any of them loses semantics.


## tests

### Source-specific bounded evidence

`ast::wave30::representation`: Rc clone sharing, End/Colon child preservation, operator distinction, empty Program.

Unexecuted here: Representation only; AST is not an evaluator. No corpus, GPU, live filesystem, timer or callback run. No broad Clippy claim; dependency/style checks were not repeated. Only appended test blocks were rustfmt-formatted; original production bytes are unchanged.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: parser constructs these nodes; interp and vm consume them. Preserving End versus Ident(end), matrix rows and Pow versus EPow is required at that join; flattening any of them loses semantics.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

### 2026-10-31 · tests

Seven named tests each passed twice debug and twice release on 2026-10-31, with actual named log lines sorted and identical across all four runs (28 selected successful executions). Each source has a compiled changed-assertion negative control; six runtime sources additionally have compiled production-body mutants that fail those unchanged tests. AST has representation evidence only. Every mutant is byte-restored and its positive test rerun. See `evidence.json` for this source's exact hash, mutation and exit statuses. External receipts retain full logs, command metadata, control records and prospective notes.

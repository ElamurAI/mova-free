# matrix-expression

## descriptant

### matrix-expression: wave30

lexer space_before distinguishes [1 -2] from [1 - 2]; Caret and DotCaret are both left-folded by the parser, retaining distinct Pow and EPow tags. Interp resolves End to dimensions and dispatches index/ops/core. value column-major layout must survive index gather and core reshape; transpose permutes it. ops short-circuit logic belongs to interp/vm; core callbacks, files, RNG and timer effects cross group and remain unexecuted here.

This source-bound partial group organizes existing Mova code. Native evidence and excluded effects are recorded in source sidecars and the root wave30 report. Registration is not a universal proof or whole-system runtime claim.

## model

# matrix-expression

## forward

text --lexer(context,space,line)--> tokens --parser(precedence,index_depth)--> AST --interp/vm caller--> column-major Value; index selects/mutates k=j*r+i; ops broadcasts or multiplies values; core creates/reshapes and delegates range/concat. Shape is part of output: vector selection, colon columnization, reshape and transpose differ.

## invariant

On bounded well-formed matrices len(re)=r*c and optional im same length; transpose is a permutation, reshape retains flattened data, ordinary bounded assignment validation errors precede mutation. No universal validity from public fields, no finite arithmetic guarantee, and AST representation is distinct from evaluation.

## cost

Lexing O(chars+tokens); ordinary parsing O(tokens) with speculative reparsing and unbounded nesting caveat. Matrix storage/transforms O(cells); indexing O(indices+output/growth), deletion may sort; broadcast O(output), multiplication O(mnk) or delegated faer; callbacks, allocation and effects add unbounded costs. No speed measurements.

## composition

lexer space_before distinguishes [1 -2] from [1 - 2]; Caret and DotCaret are both left-folded by the parser, retaining distinct Pow and EPow tags. Interp resolves End to dimensions and dispatches index/ops/core. value column-major layout must survive index gather and core reshape; transpose permutes it. ops short-circuit logic belongs to interp/vm; core callbacks, files, RNG and timer effects cross group and remain unexecuted here.

## domain

CPU synthetic bounded strings and well-formed matrices with finite modest indices/dimensions/range spans; no GPU, datasets, external API or live filesystem effects.

## limitations

Source-derived relations; caller enforces valid buffers and resources. f64 NaNs, infinities, casts and zero behavior follow source branches, not mathematical ideal arithmetic. Seven tests observe specific contracts, not all source branches.

## checks

- ast::wave30::representation
- lexer::wave30::context
- parser::wave30::precedence
- value::wave30::layout
- index::wave30::gather_update
- ops::wave30::arithmetic
- builtins::core::wave30::constructors

## catalog

## behavior-complement

Accepted bounded composition excludes erased matrix whitespace, right-associated unparenthesized powers, row-major reinterpretation, broadcast of incompatible non-singleton dimensions and successful shape-changing reshape with unequal numel. It does not exclude public malformed values, nonfinite arithmetic or core side effects.

## reverse-reconstruction

For observation y, compatible antecedents are text/AST/state triples whose parse/evaluation gives y. Comments and spellings, overwritten/deleted cells, broadcast operand factorizations and reshape dimension syntax cause ambiguity. Transpose is uniquely invertible only on valid recorded Mat; reshape requires original shape; indexing fixes selected cells only.

## layout-transport

Mat index k=j*r+i is preserved by reshape buffers, permuted to i*c+j by transpose and gathered by index; substituting row-major storage breaks all three joins.

## syntax-context

Lexer preserves whitespace and newline observations; parser contextual sign boundary and index_depth transform them into Matrix/Unary/End versus subtraction/identifier branches.

## validation-order

Whole lex precedes parse; index validates selected shape before growth; core reshape computes its unchecked known-dimension product before hole counting, then on bounded arithmetic checks hole count before divisibility/rank/product; error order cannot be commuted across adapters.

## effects

Most value/lexer/parser paths allocate only; index mutates target after bounded checks; ops linalg may warn; core can mutate interpreter/RNG and inspect files/time or invoke callbacks. Pure helper test evidence never certifies those effects.

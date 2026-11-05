# mlab-parser-expression-precedence

## descriptant

### Expression precedence ladder

oror→andand→or→and→cmp→range→additive→mult→unary→power establishes binding. power repeatedly folds left over power_operand; unary recursively prefixes before power. range accepts one or two colons.

## approximant

### Expression precedence ladder forward relation

oror→andand→or→and→cmp→range→additive→mult→unary→power establishes binding. power repeatedly folds left over power_operand; unary recursively prefixes before power. range accepts one or two colons.

Domain: Token stream with EOF sentinel, bounded recursive depth. Lex errors already resolved.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Operator sequence and precedence retained; comparison chains are left-folded rather than conjunctions. No evaluation or shape checking occurs.

Cost: O(consumed tokens) for ladder itself; recursion follows unary depth.

Failure ordering: Lower-precedence layers request tighter operands before folding; power_operand handles unary prefixes but calls postfix rather than power.

## models

## behavior_complement

2^3^2 cannot become Pow(2,Pow(3,2)); -2^2 cannot become Pow(Neg(2),2). In matrix context a sign with space before and no space after cannot be consumed as additive binary op.

## reverse_reconstruction

A left-fold power tree admits unparenthesized chain; inserted Paren nodes create distinct ASTs. Range without step admits a:b; explicit 1 produces Some(1), not identical AST.

## representation_transport

Operator sequence and precedence retained; comparison chains are left-folded rather than conjunctions. No evaluation or shape checking occurs.

## failure_precedence

Lower-precedence layers request tighter operands before folding; power_operand handles unary prefixes but calls postfix rather than power.

## resource_domain

Token stream with EOF sentinel, bounded recursive depth. Lex errors already resolved.

## interaction_relation

Whitespace flag from lexer determines matrix boundary; returned BinOp tags drive ops or interpreter short circuit.


## tests

### Bounded evidence

Selected actual-code Rust test: `parser::wave30::precedence` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: Whitespace flag from lexer determines matrix boundary; returned BinOp tags drive ops or interpreter short circuit.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.

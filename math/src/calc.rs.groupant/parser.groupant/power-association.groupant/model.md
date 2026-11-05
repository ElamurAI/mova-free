# math-calc-parser-power-association

## Scope

Source-derived bounded relation for the manifest byte interval. All relations observe one invocation and its stated state effects. Valid caller structures and finite values where specified are premises. No universal proof, percentage-error surrogate or production replacement is claimed.

## Forward

pow first consumes postfix; if ^ follows it advances once and parses unary as exponent, producing Pow(base,exponent). postfix repeatedly adds Fact or Index nodes after a primary. Unary may recursively reenter pow, so exponentiation associates right while leading minus wraps the power.

## Invariant

A successful postfix consumes complete index brackets; the exponent may be negative. The caller retains responsibility for trailing-token rejection.

## Cost

O(number of consumed tokens plus index-expression parsing); recursive depth follows exponent/unary nesting and is not capped. Postfix chain storage is linear in suffix count.

## behavior_complement

For input 2^3^2 the produced tree cannot be Pow(Pow(2,3),2). For -2^2 the tree cannot have Num(-2) as its power base. Missing ] excludes successful postfix completion.

## reverse_reconstruction

The Pow(2,Pow(3,2)) tree admits 2^3^2 and 2^(3^2), plus lexer-equivalent spellings. Grouping discarded by primary prevents unique reconstruction.

## unary_power_order

Unary - calls unary again before forming Neg; pow's exponent calls unary. Thus -2^2 gives -4 and 2^-1 gives an exact half after eval. This is a grammar relation, not floating precedence.

## postfix_power_order

2!^3 has Fact(2) as base. 2^3! has Fact(3) as exponent. Postfix iteration allows repeated factorials and indexes with their usual evaluator domain errors.

## index_failure_boundary

Index expression is parsed through or, then eat(]) must succeed before Index is returned. Prefix cursor effects survive an Err internally; parse exposes only the error.

## tree_value_distinction

2^3^2 and (2^3)^2 evaluate to512 and64 respectively. Numeric agreement for other values would not imply tree equivalence; fixture asserts the actual E structure.

## Tests

Actual helper fixture: `calc::wave34::syntax_zero_and_binding_boundaries`. See file-root ../../model.md#wave34-evidence for named profile/control receipts. Complement relations above are exclusions, not merely test lists; inverse relations retain compatible antecedents. Unexecuted paths remain source-derived.

## Epoch

Wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

## History

### 2026-10-31 · Epoch

2026-10-31 wave34: new descendant refining the existing parent; prior parent descriptions and evidence retained as historical snapshots. Production code is unchanged; inline synthetic tests add bounded observations.

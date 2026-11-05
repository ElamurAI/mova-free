# math-understand-pratt

## descriptant

### math-understand-pratt

expr(min) parses unary first; +,- have precedence1, *,/2, power3 right-associative, square/cube4 postfix. For a nonpercent lhs and percent rhs under +/-, construct lhs*(1 +/- rhs).

## approximant

### Scoped forward relation

expr(min) parses unary first; +,- have precedence1, *,/2, power3 right-associative, square/cube4 postfix. For a nonpercent lhs and percent rhs under +/-, construct lhs*(1 +/- rhs).

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Successful consumption advances cursor; parenthesized parsing consumes the matching RParen. Complete-stream checking belongs to chain, not expr alone.

#### Cost

Token traversal is linear for the grammar, with recursion proportional to nesting/operator association. Expr allocation and downstream exact evaluation are separate costs.

## models

## behavior_complement

A missing operand, unmatched closing/opening parenthesis, or fraction word without of cannot yield successful parse on that path. Unary-first parsing excludes treating -2^2 as -(2^2) in this grammar.

## reverse_reconstruction

Extra parentheses around a value can yield the same Expr. Both percent tokens and equivalent explicit multiplication can evaluate alike while producing different trees; value equality does not recover token form or the rewrite path.

## unary_power_order

expr calls unary before its operator loop, so unary minus becomes part of the power base. Power RHS uses min=3, while left-associative operators recurse at prec+1.

## of_dispatch

Frac requires Of; a slash-form nonpercent Val followed by Of builds multiplication; a percent Val followed by Of builds PctOf; ordinary nonpercent Val followed by Of errors.

## tests

### Scoped evidence

Precedence, right-associative power, percentages, slash-fraction multiplication and malformed parentheses/fraction.

Named actual source fixtures: understand::wave28::{pratt_precedence_and_percent_rewrite}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.

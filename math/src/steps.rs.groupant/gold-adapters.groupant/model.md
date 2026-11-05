# math-steps-gold-adapters

## descriptant

### math-steps-gold-adapters

Equation parsing respects product before sum, maps a value to first unused equal quantity (then reuses an equal quantity if necessary), and requires complete token consumption. Calculator steps first prefer the latest prior exact result, then validate reconstructed exact result equals the declared value.

## approximant

### Scoped forward relation

Equation parsing respects product before sum, maps a value to first unused equal quantity (then reuses an equal quantity if necessary), and requires complete token consumption. Calculator steps first prefer the latest prior exact result, then validate reconstructed exact result equals the declared value.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Returned trees reference accepted quantity coordinates; exact calculator validation holds per accepted step. No semantic equivalence to a general programming language is claimed.

#### Cost

Leaf matching scans quantities per numeric token; prior-result matching scans previous results. Repeated exact subtree evaluation may revisit expanded subtrees and has rational bit-complexity costs.

## models

## behavior_complement

A calculator step whose reconstructed Tree::exact differs from the declared RHS cannot enter prev. Unknown operands and leftover tokens cannot produce successful tree_from_equation. Repeated equal values are not excluded.

## reverse_reconstruction

The expression8-3 can select different physical occurrences in a quantity list containing duplicate8/3 values, but the greedy adapter chooses one. A later token5 can mean a previous8-3 subtree or a literal5; latest prior result wins, hiding the textual occurrence choice.

## intermediate_priority

prev.iter().rev().find(equal) precedes match_leaf, so a previous result shadows an equal original quantity. This preserves computed subproblems, not unique occurrence identity.

## validation_order

Parsing/full-consumption succeeds before declared RHS parsing/exact equality in calculator processing. Failure stops the entire conversion; previously accumulated prev is not returned as a partial result.

## tests

### Scoped evidence

Exact arithmetic, unknown/trailing token and wrong-calculator-result rejection.

Named actual source fixtures: steps::wave28::{equation_oracle_and_exact_replay; calculator_validation_and_script_bounds}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.

# math-deduct-gold

## descriptant

### math-deduct-gold

Postfix traversal memoizes ordered subtree keys, recursively obtains child ids, rejects child ids>255, normalizes +/* operand index order, emits a node and memoizes its new id. It returns None for0 or>8 operations; otherwise appends Stop.

## approximant

### Scoped forward relation

Postfix traversal memoizes ordered subtree keys, recursively obtains child ids, rejects child ids>255, normalizes +/* operand index order, emits a node and memoizes its new id. It returns None for0 or>8 operations; otherwise appends Stop.

This source-derived relation is partial; no executable percentage surrogate or replacement is supplied.

## invariant

### Scoped domain and invariants

Accepted gold has1..8 arithmetic nodes plus final Stop. Caller must separately establish source leaf range and search candidate reachability.

#### Cost

Recursive key generation can revisit descendants at multiple ancestors; memo lookup and retained strings add work beyond one constant-time visit per tree node.

## models

## behavior_complement

A returned sequence cannot have more than8 Comb steps or an encoded child index>255. It can contain equal child ids and leaf ids>=n, so general validity/reachability cannot be inferred from success.

## reverse_reconstruction

Two occurrences of the identical (leaf0+leaf1) subtree collapse to one pool node. Swapping the children changes the memo key even though tkey canonicalizes the commutative tree; some equivalent arithmetic trees therefore have different memoization histories.

## memo_key_asymmetry

gold_steps' internal key preserves left/right order; tkey sorts Add/Mul child strings. Equality of tkey is weaker than equality of gold memo key.

## postfix_index_guard

A newly assigned node id may itself exceed255; rejection happens if a later parent attempts to use it. The checks constrain consumed operands, not all allocated pool ids.

## tests

### Scoped evidence

Commutative tkey, reused identical subtree, leaf-only and operand256 rejection.

Named actual source fixtures: deduct::wave28::{dag_gold_reuses_subtrees_and_canonicalizes}.

Parent model.md#tests records commands and paired profiles. No additional coverage inferred from parent fixtures outside this byte range; source-scoped numeric/parse properties remain partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: replaced duplicate catalog sentences with concrete exclusions, compatible-antecedent examples and two distinct scoped behavioral views. Source and prior execution evidence are unchanged; this revision ran no tests.

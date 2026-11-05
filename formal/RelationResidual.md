# Schröder equivalences and the right residual

`RelationResidual.lean` imports `RelationComposition.lean` and reuses its
`compose` (first R, then S), `converse` and scoped `complement U R`. It adds
`disjointRel A B` (no shared pair), `subsetRel A B` and `full X Z`, the full
rectangle of an endpoint pair. All statements quantify over abstract typed
relations `R : X → Y → Prop`, `S : Y → Z → Prop`, `T : X → Z → Prop`. They do not
certify Rust extraction, the finite evaluator in `tools/correspondence`,
IEEE-754 arithmetic, approximation error or runtime substitution.

| Theorem | Statement | Axioms |
|---|---|---|
| `schroeder_converse_left` | (R;S) ∩ T = ∅ ↔ (R°;T) ∩ S = ∅ | none |
| `schroeder_converse_right` | (R;S) ∩ T = ∅ ↔ (T;S°) ∩ R = ∅ | none |
| `schroeder_three_way` | (R°;T) ∩ S = ∅ ↔ (T;S°) ∩ R = ∅ | none |
| `right_residual_full` | R;S ⊆ T ↔ S ⊆ ¬_{Y×Z}(R° ; ¬_{X×Z} T) | propext, Classical.choice, Quot.sound |
| `right_residual_scoped` | same with complements inside U and V, given R;S ⊆ U and S ⊆ V | propext, Classical.choice, Quot.sound |
| `omitted_converse_counterexample` | dropping the converse is false on Bool | none |

The disjointness equivalences are constructive: a common pair on one side is
moved to the other by reading a composition witness backwards. The residual's
reverse direction needs excluded middle on `T x z` (`by_contra`), hence the
classical axioms. With a restricted scope, the residual holds under the
two containment premises. Without `R;S ⊆ U`, a composed pair outside U is never
excluded by the complement of T inside U.

For an allowed-output relation T, keeping "first R, then S" inside T is equivalent
to a condition on S alone, obtained by sending the forbidden complement ¬T back
through the converse of R. The largest admissible S is `¬(R° ; ¬T)`. This links forward models,
behavior complements and reverse (converse) models without claiming any inverse.

Counterexample: on Bool, take R = {(false,true)}, S = {(false,false)} and
T = {(true,false)}. Then R;S is empty, so it is disjoint from T. But R;T contains
(false,false) through witness true, and that pair lies in S.

## Offline check

Same procedure as `RelationComposition.md`: cached Lean 4.35.0-rc3/Mathlib project,
no fetching or Mathlib rebuild. Copy `Complementer.lean`,
`RelationComposition.lean` and `RelationResidual.lean` into a fresh external RUN
directory. Compile them in that order with `lake env lean --root="$RUN" -o ...`
and `LEAN_PATH="$RUN"`. Require exit 0, six axiom reports without `sorryAx`, and
only the standard foundations.

Measured 2026-10-31: all three files exited 0 and the six reports match the table.
Two external negative controls exited 1 with Lean errors. One proved the
converse-free variant by the same witness swap (type mismatch: `S x z` where
`S y z` is expected). The other tried a residual missing the inner complement
(unsolved goal, type mismatch on `¬T x z`).

This extension is not registered in the `formal_neural` Rust harness.

# Abstract relation composition

`RelationComposition.lean` imports and reuses `Complementer.lean`. Composition
`compose R S x z` means first R, then S, with an existential intermediate value.
The specifications quantify over abstract relations; they do not certify arbitrary
Rust extraction, groupant source projections, finite evaluator correspondence,
IEEE-754 arithmetic, or runtime substitution.

The ten theorem reports cover converse composition and involution, composition
associativity, both identity laws, double complement with explicit scope,
conditional complement involution, the existing function-preimage composition,
a counterexample to complement distribution, and conditional error propagation.

Complement is `U x y ∧ ¬ R x y`. Double complement gives `U x y ∧ R x y`;
removing U requires the premise that R lies inside U. Converse reverses the order
of composition. Behavior complement has no such composition law: for Bool
identity relations in the full universe, the complement of their composition is
inequality, whereas composing their complements yields identity. The diagonal
pair `(false, false)` separates them with intermediate witness `true`.

The error theorem assumes a first-stage absolute error bound, a second-stage
bound at the *approximated* intermediate input, and a nonnegative downstream
sensitivity factor L. It then proves `secondError + L * firstError`. Independent
observed percentages do not establish these premises, a shared metric, domain
coverage, or a composition-wide percentage cap. The theorem reuses the triangle
inequality budget from `Complementer.lean`.

## Offline check

Use an already cached Lean 4.35.0-rc3/Mathlib project; no dependency fetching or
Mathlib rebuilding is required. Set `LEAN_PROJECT` to that project, `FORMAL` to
this directory and `RUN` to a fresh external run directory. From the cached
project, compile the imported source into RUN, then add RUN to the module path:

```sh
mkdir -p "$RUN"
cp "$FORMAL/Complementer.lean" "$FORMAL/RelationComposition.lean" "$RUN/"
cd "$LEAN_PROJECT"
timeout 90 lake env lean --root="$RUN" -o "$RUN/Complementer.olean" "$RUN/Complementer.lean"
LEAN_PATH="$RUN" timeout 90 lake env lean --root="$RUN" -o "$RUN/RelationComposition.olean" "$RUN/RelationComposition.lean"
```

The new file reports ten theorem axiom sets. Require successful Lean exit, no
`sorryAx`, and only Lean's standard foundations (`propext`, `Classical.choice`,
`Quot.sound`). An external negative-control file can import RelationComposition
and falsely claim that complement distributes over composition; applying the
counterexample above must reject it. A second control drops the sensitivity
factor and must fail for the amplifying function `g(x) = 2*x`.

Measured 2026-10-31 with the installed Lean 4.35.0-rc3 and cached Mathlib:
the imported baseline compiled, the extension exited 0 with all ten axiom
reports restricted to the standard foundations, and both controls exited 1
with Lean unsolved-goal errors (no timeout). The first control asserted the
Bool complement-distribution equivalence and attempted `simp [complement,
compose, identity]`; the second asserted `|(2 : ℝ) * 1 - 2 * 0| ≤ 0 + 1`
and attempted `norm_num`. Neither false formula was admitted.

This standalone extension is not registered in the existing `formal_neural`
Rust harness. There is no formal Lake project in the repository to update.

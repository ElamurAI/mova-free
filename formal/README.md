# formal — specifications in Lean

Lean 4 specifications of selected Mova behavior and mathematical model laws.
Properties are proved on the specification; selected executable parts also have
Rust correspondence tests, as documented below.

How the discrete specifications below are checked:

1. The function is written in Lean step for step as the Rust one (the same loop, with fuel instead of `while`).
2. A theorem states its properties and is proved by evaluation on a bounded domain (`decide`): no `sorry`, no axiom.
3. Lean prints a table of the function's values on a larger domain; the table is kept here.
4. A Rust test compares the code with the table, value by value.

Each step has a negative control: a false property must fail in Lean, and a changed table row must fail the Rust
test.

| specification | Rust | proved | compared |
|---|---|---|---|
| `PageWindow.lean`: the pager's window | `page_window` in a site pager outside this repository | every window up to 30 pages: at most eleven pages, the first, the last and the current page shown, pages in order, a gap only over two pages or more | every window up to 60 pages and of the lists of 99, 100, 101, 109, 999, 1 000 and 1 001 pages, and the deepest page's distance from the first for every list up to 1 000 pages, `the_window_is_its_lean_specification` |

Run (Lean 4.24.0 core only, no Mathlib; elan's toolchain in `~/.elan`):

```sh
cd formal && lean PageWindow.lean > page_window.table   # proves the theorem, prints the table
```

The Rust side (`the_window_is_its_lean_specification`, the code against the table) is not in this repository.

## Neural mathematics

`Softmax.lean` and `Activation.lean` use Lean 4.35.0-rc3 with Mathlib at the
matching release. Their theorems quantify over real inputs and arbitrary finite
row sizes; they are analytic proofs, rather than bounded enumeration.

- `directional_derivative` derives the softmax derivative from normalized
  exponentials, using the quotient rule and positivity of the denominator.
- `jacobian_reduction` and `vjp_reduction` factor the dense Jacobian product
  and its transpose into `p * (g - sum(p*g))`. Sharing the dot product gives
  linear work per row rather than materializing a quadratic matrix.
- `backward_zero_sum` and `upstream_shift` establish two invariants for a
  normalized probability row.
- `gated_derivative` gives the product-rule derivative of `x/2*(1+gate(x))`.
- `gate_error_bound` transfers an assumed pointwise gate error epsilon to an
  activation error at most `abs(x)*epsilon/2`. It does not establish a global
  error bound between erf and tanh.

The executable connection is `gparse/src/softmax.rs` and `model::Gelu`.
CUDA tests compare derivatives with reference autodiff and independent finite
differences; model tests compare outputs. There is no
mechanized refinement proof from Lean to Rust/CUDA or IEEE-754 error theorem.
Lean verifies the mathematical transformations; GPU timing measures cost.

The source cites Goodfellow, Bengio & Courville (2016) for softmax and
Hendrycks & Gimpel (2016) for GELU. `#print axioms` reports only standard Lean
foundations (`propext`, `Classical.choice`, `Quot.sound`), with no admitted
proofs or project-specific axioms.

Run the gate against an existing cached Mathlib project:

```sh
LEAN_PROJECT="$MOVA_DATA/lean/mathproofs" cargo test --release -p gparse --test formal_neural -- --ignored --nocapture
```

The Rust harness runs Lean with a timeout, checks all nine specifications and
rejects admitted proofs. Its controls change the Jacobian subtraction to
addition, reverse the prediction correction and school update, and falsely
claim zero activation approximation error, and add a spurious unit to a projected score. The enrichment control removes artifact gates from completion. The complementer control falsely removes the output-error budget. All nine must fail in Lean.
No network or dependency-update command is run.

The [GELU experiment](../gparse/README.md#gelu-approximation-option)
records a measured candidate that is not the default.


## Updating cost model

`Twin.lean` specifies the scalar prediction correction used by
`tools/twin`'s recursive least squares:

- `residual_identity`: after correction, the current residual is
  `(observed - predicted)/(1 + leverage)`.
- `residual_contracts`: its absolute value cannot grow for nonnegative leverage.
- `loss_derivative`: the derivative of half squared prediction error.
- `cost_derivative`: sensitivity of a local linear/quadratic cost hypothesis.

These theorems do not prove predictive accuracy on future runs or the full
matrix/IEEE-754 implementation. The Rust model checks matrix definiteness and
is compared against an independent batch ridge solution. The existing
`gparse --test formal_neural` gate also checks this file and requires its
wrong-sign correction control to fail. All four axiom reports use only the
standard foundations listed above. See [the cost-twin pilot](../docs/training.md#the-cost-twin)
for the measurements, the prospective misses and their corrections.

## School learning

[School.lean](School.lean) models the fixed-witness score gap for the pragmatic
school learner. Its three theorems prove the correction identity, monotonicity
for a nonnegative learning rate, and the derivative with respect to that rate.
They quantify over finite real feature vectors. They do not prove global
max-score convergence, language understanding or integer overflow safety.

The Rust learner in world/src/knowledge.rs uses the same active witnesses for
selection, explanations and weight updates. A regression constructs an old
witness mismatch that moves the active score gap backwards. The existing
formal_neural harness checks this specification and rejects a reversed update
sign, with only the standard Lean foundations listed above.

The harness checks nine specifications and thirty-eight axiom reports, with
nine deliberately false formula controls. It uses installed Lean/Mathlib
without downloads. See [the school audit](../docs/school.md#active-witnesses)
for the runtime measurements and the limits of transferring kernel timing
to whole-course timing.


## Lesson-local evaluation reuse

[ExerciseReuse.lean](ExerciseReuse.lean) supplies three conditional properties:

- Equal weights on all used coordinates preserve a linear feature score.
- Equal scores preserve any deterministic selection rule, including ties.
- Changes confined to inactive features have zero directional derivative.

The Rust reader currently depends on one Ask block of 119 feature weights
during an answer. world/src/knowledge/learning.rs compares that entire block,
keeps update order and discards its memo between lessons. Structural support
and the remembered world must remain fixed during each call.
The proofs do not establish those implementation dependencies by themselves;
code review, synthetic weight trajectories and real post-lesson hashes check
the connection.

The negative control adds one to a score equality and must be rejected.
[The training audit](../docs/school.md#exact-reuse-inside-lesson-training)
records forecasts, runtime evidence and the exact reference switch.


## Enrichment completion and cost surrogate

Enrichment.lean adds four properties: completion requires every artifact gate;
a rejected dictionary blocks completion; annotation alone is insufficient;
and the derivative of fixed + work/workers is -work/workers^2 away from zero.
This ideal cost model excludes quotas, contention and unequal jobs.

Rust enumerates 125 artifact-state assignments and checks actual failure/resume
paths. Enrichment ends at staged drafts. These proofs do not
establish generated content's semantic correctness or whole-system throughput.
See [the enrichment pilot](../docs/training.md#corpus-enrichment).

## Retained school state

Retention.lean specifies closed-book state preservation and answer equivalence
assuming a decode/encode round-trip law. It also proves the repeated-assessment
cost identity and derivative. Rust round-trip and real-process probes test the
implementation correspondence; the Lean assumptions do not verify serialization
or semantic correctness. The shared formal harness checks four axiom reports and
a deliberately false cost identity for this specification.

## File and interaction models

[Modeling contract and registry](../docs/mathematical-modeling.md) covers forward
relations, forbidden transitions and hierarchical interaction models.
StudyComparison.lean proves four binding/comparison properties and emits the
256-row StudyComparison.table. The actual Rust comparison is checked against
that table; a 64-case inverse binding test and a real foreign-assessment recovery
probe separately test implementation correspondence. Source hashing and parser
semantics are outside those Lean proofs. The harness rejects a deliberately
removed assessment guard as a false specification.

## Reverse models / complementers

[Complementer.lean](Complementer.lean) proves nine properties of reverse/preimage
relations: inclusion, conditional uniqueness, reverse composition, required
injectivity for a left inverse, projection ambiguity and impossibility of its
left inverse, reversible swap, unreachable outputs, and addition of forward-model
and observation error budgets. These specification laws do not extract an
inverse from Rust or identify unique real-world causes. See [the contract](../docs/mathematical-modeling.md).

The existing formal_neural harness checks all nine axiom reports and requires
a false zero-error bound to fail. No reverse-query runtime or Rust/Lean
correspondence for such a runtime is claimed.

## Relation algebra and frozen sources

- [RelationComposition.md](RelationComposition.md) (`RelationComposition.lean`, reusing `Complementer.lean`):
  composition, converse, identity and scoped complement laws over abstract relations.
- [RelationResidual.md](RelationResidual.md) (`RelationResidual.lean`): the Schröder equivalences and the right
  residual over abstract typed relations.
- `FrozenUnion.lean`: the frozen-source union preimage `A ∪ P = O ↔ A ⊆ O ∧ O \ A ⊆ P ∧ P ⊆ O` over arbitrary sets
  (A the current assessment sources, P the parent's frozen sources, O the result); `tools/enrich/src/study.rs`
  checks the finite three-ID case in its correspondence tests.

These prove set and relation laws, not Rust parsing, extraction, the finite evaluator in `tools/correspondence` or
runtime substitution.

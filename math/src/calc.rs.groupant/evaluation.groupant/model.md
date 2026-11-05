# math-calc-evaluation

## descriptant

### math-calc-evaluation

Env/eval implement ordered state changes, capped notes, exact/real coercions, lazy forms and inclusive binders. Range errors retain current variable; CRT writes modulus. Parent calls comb/nt and can duplicate evaluation. Work includes big arithmetic and every reached callee.

## approximant

### Scoped forward and cost

Env/eval implement ordered state changes, capped notes, exact/real coercions, lazy forms and inclusive binders. Range errors retain current variable; CRT writes modulus. Parent calls comb/nt and can duplicate evaluation. Work includes big arithmetic and every reached callee.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

Env/eval implement ordered state changes, capped notes, exact/real coercions, lazy forms and inclusive binders. Range errors retain current variable; CRT writes modulus. Parent calls comb/nt and can duplicate evaluation. Work includes big arithmetic and every reached callee.

Source range[17753,55308) bound by manifest digest. Parent domain applies, not arbitrary public states.

Additional source limits from independent review: range converts span through z_to_u64(...).unwrap_or(0); a positive span>u64 charges0 then loops without per-iteration ticks. Fingerprint factorial reduces its operand first: original operand P61+1 yields residue argument1 and result1, whereas mathematical factorial residue is0. Restrict original factorial operand domain; residue<=2M alone is insufficient. No huge input was executed.

## models

# Source-scoped typed relations

## behavior_complement

false and missing excludes unknown-variable error because right operand is skipped; ordinary Add evaluates both and can error. Successful normal ranged completion excludes a lingering overwritten binding; failure during body does not.

## reverse_reconstruction

Final sum0 admits an empty range or nonempty canceling body values; trace text cannot recover every iteration. A failed range with k=1 admits multiple previous k bindings because restoration was skipped.

## state_prefix

ranged saves k, ticks then inserts k then evaluates body. Body error retains inserted k; tick failure occurs before insertion of the next value. Normal/boolean early exit restores k before returning.

## argument_multiplicity

call first evaluates args; min/max/gcd/lcm and list aggregates call flat_args again. fact and at reenter eval. Therefore state/step effects can repeat even if output equals single evaluation.

## resource_accounting

range computes u64(span).unwrap_or(0), ticks once, then loops without ticks. Positive span>u64 can charge0 and still iterate. Ranged forms compare span with max_steps cast to i64 and tick each iteration; neither is a universal execution bound.

## numeric_coercion

Q/Q arithmetic exact; mixed values use f64 and real() finite gate, except exact zero shortcuts retain Q0. Traces cap 200 independently of steps. CRT additionally writes crt_modulus.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

Env/eval implement ordered state changes, capped notes, exact/real coercions, lazy forms and inclusive binders. Range errors retain current variable; CRT writes modulus. Parent calls comb/nt and can duplicate evaluation. Work includes big arithmetic and every reached callee.

## wave34-composition

# Evaluation descendant composition

The mixed-zero child runs only after eval has produced operands and arith has checked list concatenation. It returns before generic arithmetic tick and real conversion. The ranged-binding child is dispatched directly by call for binder forms, before ordinary eager argument evaluation. Its sum/prod accumulator can invoke arith and therefore the mixed-zero child after evaluating a body. A skipped body or early witness never reaches later arithmetic. Body/accumulator failure retains the currently inserted binder; normal exit restores it before result extraction.

Both relations share Env.steps, vars and approx. A Q(0) result may retain approx=true from operand evaluation, so composing only numeric values loses meaningful state. Parent inversion requires a compatible body trace and exact restoration/failure cut; it is not the Cartesian product of independent child preimages. Existing callee checks, CRT writes and repeated ordinary argument evaluation remain historical parent models.

## Cost

For a reached range sum the costs of endpoint evaluation, iteration tick/binding, body evaluation and accumulator operations. The zero shortcut avoids only the local arith tail; operand evaluation is already paid. List retention and cloned exact values add storage not captured by steps. No speed claim or hard deadline bound.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

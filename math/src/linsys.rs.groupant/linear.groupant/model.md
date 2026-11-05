# math-linsys-linear

## descriptant

### math-linsys-linear

linear_form maps supported syntax to affine Q coefficients after selected-variable absence test; constant calc eval may mutate Env. Products need a constant operand, division constant nonzero denominator. Coefficients do not recover syntax. Traversal/vector costs compose with constant evaluation.

## approximant

### Scoped forward and cost

linear_form maps supported syntax to affine Q coefficients after selected-variable absence test; constant calc eval may mutate Env. Products need a constant operand, division constant nonzero denominator. Coefficients do not recover syntax. Traversal/vector costs compose with constant evaluation.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

linear_form maps supported syntax to affine Q coefficients after selected-variable absence test; constant calc eval may mutate Env. Products need a constant operand, division constant nonzero denominator. Coefficients do not recover syntax. Traversal/vector costs compose with constant evaluation.

Source range[392,2640) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

Selected-variable product with both nonzero coefficient vectors excludes an affine result; division by a variable denominator excludes success before the denominator-zero constant check.

## reverse_reconstruction

Coefficient vector c and constant k admit all supported syntax representing sum c_i*x_i+k, e.g x+x and2*x. Variable-free expressions can share coefficients while leaving different calc Env effects.

## absence_priority

Collect free variable spellings first; no selected name means eval entire subtree through calc, requiring V::Q. Unsupported syntax can still be admitted when constant relative to selected vars.

## affine_join

Add/sub combine coefficients and constants componentwise. For multiplication one side must have all-zero coefficients, scaling the other's vector and constant. Division scales by reciprocal constant denominator.

## variable_coordinate

E::Var selects the first matching vars index and places1 there; duplicates in vars therefore do not represent distinct coordinates. Caller should supply distinct names.

## failure_effect

Recursive left branch may evaluate constants/change Env before right error. Conversion returns String without rollback; callers cannot infer unchanged Env from Err.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

linear_form maps supported syntax to affine Q coefficients after selected-variable absence test; constant calc eval may mutate Env. Products need a constant operand, division constant nonzero denominator. Coefficients do not recover syntax. Traversal/vector costs compose with constant evaluation.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

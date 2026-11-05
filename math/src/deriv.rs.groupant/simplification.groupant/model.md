# math-deriv-simplification

## descriptant

### math-deriv-simplification

simplify folds constants and identities, bounded integer powers with zero-negative guard.0/x and0*x erase singularities; different-domain trees share outputs. No domain-preserving guarantee; recursive reconstruction includes rational arithmetic.

## approximant

### Scoped forward and cost

simplify folds constants and identities, bounded integer powers with zero-negative guard.0/x and0*x erase singularities; different-domain trees share outputs. No domain-preserving guarantee; recursive reconstruction includes rational arithmetic.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

simplify folds constants and identities, bounded integer powers with zero-negative guard.0/x and0*x erase singularities; different-domain trees share outputs. No domain-preserving guarantee; recursive reconstruction includes rational arithmetic.

Source range[605,2373) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

Simplification of numeric add/sub/mul must return exact Q arithmetic, and division constant-fold only if denominator nonzero. However later0/x identity does permit0/0->0; domain preservation is explicitly excluded.

## reverse_reconstruction

Output0 admits Num0, x*0 and0/0; their original definedness differs. Output x admits x+0, x*1, x^1 and x itself, so simplified AST cannot reconstruct source or domain.

## power_guard

Constant power folding requires integral exponent fitting i64 with |k|<=64 and excludes zero base with negative exponent. Subsequent power-zero identity returns1; MIN abs remains unchecked edge.

## rewrite_order

Children simplify first, then numeric folding, then identities. Hence a compound child can become numeric before parent dispatch; this is recursive local rewriting, not polynomial canonicalization.

## negation_rule

Neg(Num x) folds sign; Neg(Neg b) unwraps b; other negations remain. Multiplication by left -1 rewrites to Neg, while symmetric right -1 has no matching dedicated identity.

## call_boundary

Calls only simplify arguments and preserve function/arity; no evaluation or domain checking is performed. Division/multiplication zero rewrites can erase unsupported or undefined calls.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

simplify folds constants and identities, bounded integer powers with zero-negative guard.0/x and0*x erase singularities; different-domain trees share outputs. No domain-preserving guarantee; recursive reconstruction includes rational arithmetic.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

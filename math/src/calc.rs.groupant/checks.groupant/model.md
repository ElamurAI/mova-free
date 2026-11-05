# math-calc-checks

## descriptant

### math-calc-checks

approx computes supported f64 expressions; fingerprint computes integer residues; independent prefers exact integer residues then finite f64. Matching is many-to-one, tolerance1e-9 max(1,|result|). Unavailable or matching checks are not global numerical guarantees.

## approximant

### Scoped forward and cost

approx computes supported f64 expressions; fingerprint computes integer residues; independent prefers exact integer residues then finite f64. Matching is many-to-one, tolerance1e-9 max(1,|result|). Unavailable or matching checks are not global numerical guarantees.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

approx computes supported f64 expressions; fingerprint computes integer residues; independent prefers exact integer residues then finite f64. Matching is many-to-one, tolerance1e-9 max(1,|result|). Unavailable or matching checks are not global numerical guarantees.

Source range[55308,62082) bound by manifest digest. Parent domain applies, not arbitrary public states.

Additional source limits from independent review: range converts span through z_to_u64(...).unwrap_or(0); a positive span>u64 charges0 then loops without per-iteration ticks. Fingerprint factorial reduces its operand first: original operand P61+1 yields residue argument1 and result1, whereas mathematical factorial residue is0. Restrict original factorial operand domain; residue<=2M alone is insufficient. No huge input was executed.

## models

# Source-scoped typed relations

## behavior_complement

For integer result x, a supported fingerprint excludes acceptance when x mod P61 differs from computed fp. Matching excludes no error by a multiple of P61. Unsupported paths return None, not Check{ok:false}.

## reverse_reconstruction

Given fingerprint r, compatible claimed integers are {r+k*P61 | k integer}. A finite f64 check admits results within its scaled tolerance. Neither relation identifies the original E or exact result.

## priority_relation

independent checks integer fingerprint first and returns it immediately. Only unavailable fingerprint reaches finite f64 comparison; a failing supported fingerprint is not rescued by a floating match.

## finite_tolerance

For Q fallback require both a and fx finite and |fx|<1e300, then |a-fx|<=1e-9 max(|fx|,1). R branch compares without this explicit finite prefilter; unsupported approx remains None.

## factorial_operand_domain

fingerprint(Fact(a)) fingerprints a first, then computes factorial of residue if <=2M. Correctness needs the original operand admitted in the small natural domain; original P61+1 maps to1! residue1 although true factorial residue0. This huge case is source-derived, not executed.

## availability_boundary

approx supports arithmetic/selected calls but not lists/booleans/index/ranged functions; mod/floor/ceil reject magnitudes>=4.5e15. fingerprint supports integer addition, subtraction, multiplication, natural powers, factorial and negation; division returns None, so independent may instead use its floating fallback. Power evaluates the exponent via a fresh Env.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

approx computes supported f64 expressions; fingerprint computes integer residues; independent prefers exact integer residues then finite f64. Matching is many-to-one, tolerance1e-9 max(1,|result|). Unavailable or matching checks are not global numerical guarantees.

## wave34-composition

# Check descendant composition

The residue-fibers child feeds independent only for an integer exact result. A supported residue comparison returns immediately even on disagreement; f64 fallback runs only when fingerprint is unavailable. The integer result is reduced by the same modulus, so accepted claims inhabit a residue class and may differ by P61. Unsupported result types yield no check. The child also evaluates power exponents in a fresh Env and the factorial argument reduction remains a documented domain gap.

Parent compatible antecedents must preserve the supported/unsupported branch, not only the numeric residue. A failed fingerprint cannot be repaired by a favorable floating recomputation in the parent. Historical approximate and finite-tolerance models remain distinct; no total numerical certificate is inferred.

## Cost

Use the reached fingerprint cost, then one result reduction and comparison. Only an unavailable fingerprint adds approximate-expression traversal. Avoid adding both branches when fingerprint succeeds. Big-integer reduction and cloned exponent variables contribute outside the modular multiplication count.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

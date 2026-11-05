# math-poly-rational

## descriptant

### math-poly-rational

rf converts expressions to reduced fractions, evaluating constant subtrees through calc Env; only exact Q constants and target-independent integer powers admitted. Divisor roots are saved before cancellation. Original syntax is not reconstructible from normalized fraction/domain. Parent Poly operations supply cost and normalization assumptions.

## approximant

### Scoped forward and cost

rf converts expressions to reduced fractions, evaluating constant subtrees through calc Env; only exact Q constants and target-independent integer powers admitted. Divisor roots are saved before cancellation. Original syntax is not reconstructible from normalized fraction/domain. Parent Poly operations supply cost and normalization assumptions.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

rf converts expressions to reduced fractions, evaluating constant subtrees through calc Env; only exact Q constants and target-independent integer powers admitted. Divisor roots are saved before cancellation. Original syntax is not reconstructible from normalized fraction/domain. Parent Poly operations supply cost and normalization assumptions.

Source range[6191,9459) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

rf division by identically zero divisor numerator excludes success. After (x²-1)/(x-1) reduction, a result omitting the recorded denominator root1 contradicts the domain trace.

## reverse_reconstruction

Reduced(x+1)/1 admits both x+1 and (x²-1)/(x-1), but dens distinguishes their admitted domains. Even num,den,dens together admit multiple syntax/evaluation histories.

## constant_evaluation

If target variable absent, evaluate through calc Env before syntax dispatch; only V::Q is accepted as constant. Approximate/string/list constants error, potentially after effects in Env.

## fraction_join

Add/sub combine n1*d2 ± n2*d1 over d1*d2; multiply n1*n2/d1*d2; division n1*d2/d1*n2 after recording nonconstant n2. reduce divides gcd and makes denominator monic.

## power_domain

Exponent must be target-independent and exact integer with |k|<=200. Negative powers swap numerator/denominator and record nonconstant numerator; no explicit identically-zero check on this negative-power branch.

## failure_order

For ordinary binary forms rf processes left child then right. Division tests right numerator zero after both; power evaluates/checks exponent before base conversion. Failed conversion can retain earlier Env effects.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

rf converts expressions to reduced fractions, evaluating constant subtrees through calc Env; only exact Q constants and target-independent integer powers admitted. Divisor roots are saved before cancellation. Original syntax is not reconstructible from normalized fraction/domain. Parent Poly operations supply cost and normalization assumptions.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

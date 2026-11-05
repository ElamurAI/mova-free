# math-poly-roots

## descriptant

### math-poly-roots

Root variants distinguish rational/surd/f64. Yun multiplicities, rational validation and product checks compose; remaining factors use500-sweep Aberth with residual tests. Root-only inverse needs leading scalar and original domain. No convergence theorem or total error bound.

## approximant

### Scoped forward and cost

Root variants distinguish rational/surd/f64. Yun multiplicities, rational validation and product checks compose; remaining factors use500-sweep Aberth with residual tests. Root-only inverse needs leading scalar and original domain. No convergence theorem or total error bound.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

Root variants distinguish rational/surd/f64. Yun multiplicities, rational validation and product checks compose; remaining factors use500-sweep Aberth with residual tests. Root-only inverse needs leading scalar and original domain. No convergence theorem or total error bound.

Source range[9459,22090) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

solve_poly zero polynomial excludes a root list and returns any-value error; nonzero constant returns no roots. An exact Q candidate failing substitution cannot be accepted by rational_roots.

## reverse_reconstruction

Root list with multiplicities fixes only a monic polynomial under exact complete roots; any nonzero leading scalar shares it. Approximate roots admit many nearby polynomials and cannot recover exact coefficients.

## squarefree_relation

Yun uses gcd(f,f'), then repeatedly gcd(b,c-b') and associates factor with multiplicity i. Loop stops after400; factor product/completeness checks must detect omissions rather than assume unbounded decomposition.

## candidate_filter

Small primitive endpoint coefficients enumerate signed divisors pn/qd, checking exact Horner zero. Large-coefficient fallback rounds numeric real root*qd and still validates exact candidate; missing candidates remain possible.

## surd_transport

Quadratic discriminant N/M becomes sqrt(N*M)/M; factorization extracts square part s and squarefree d, yielding a0±b0 sqrt(d). Exact substitution in Q(sqrt(d)) checks both coefficients zero; factor completion can include probable primes.

## numeric_residual

Aberth iterates at most500 sweeps and skips nonfinite steps. solve_poly checks |p(z)|<=1e-8 max(scale,1), plus multiplicity sum and factor rebuild; residual/completeness is not convergence, conditioning or rational-domain certification.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

Root variants distinguish rational/surd/f64. Yun multiplicities, rational validation and product checks compose; remaining factors use500-sweep Aberth with residual tests. Root-only inverse needs leading scalar and original domain. No convergence theorem or total error bound.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

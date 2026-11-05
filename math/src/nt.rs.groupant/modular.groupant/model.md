# math-nt-modular

## descriptant

### math-nt-modular

Euclid certificates and normalized modular operations compose through CRT gcd compatibility. Empty/mismatched CRT rejects but check_crt zip-truncates. Antecedents are congruence fibers; bare modulus helpers can panic on zero. Costs depend on bit widths and pairs.

## approximant

### Scoped forward and cost

Euclid certificates and normalized modular operations compose through CRT gcd compatibility. Empty/mismatched CRT rejects but check_crt zip-truncates. Antecedents are congruence fibers; bare modulus helpers can panic on zero. Costs depend on bit widths and pairs.

No executable percent surrogate claimed.

Epoch36 boundary: public gcd(0,0)=0 via explicit guard; all other signed operands use the existing dashu gcd. This agrees with ext_gcd/check_gcd. The original wrapper's zero-zero panic was observed before repair; bare modular helpers still retain their separate nonzero-modulus preconditions.

## invariant

### Scoped invariant

Euclid certificates and normalized modular operations compose through CRT gcd compatibility. Empty/mismatched CRT rejects but check_crt zip-truncates. Antecedents are congruence fibers; bare modulus helpers can panic on zero. Costs depend on bit widths and pairs.

Source range[1492,5396) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

modpow modulus0 excludes Some; modulus1 returnsSome0 even for negative exponent. CRT empty/mismatched vectors or zero modulus excludes a successful pair.

## reverse_reconstruction

CRT(x,M) admits all residue/modulus lists with lcm|mi|=M and ri congruent x modulo|mi|. Reordering, redundant congruences and modulus signs are lost; x alone also loses M.

## bezout_certificate

ext_gcd returns g>=0 and s*a+t*b=g. check_gcd also tests divisibility and matching independently computed g; at g=0 both inputs must0.

## inverse_domain

modinv requires m!=0 and gcd(a,|m|)=1; modpow negative e delegates inverse except m=1 shortcut. mod_floor alone does not guard m=0.

## crt_merge

For accumulated(x,m), g=gcd(m,mi); reject if g does not divide r-x. Solve t modulo mi/g, then x'=x+m*t normalized modulo m*(mi/g).

## checker_shape

check_crt uses zip and does not test counts; empty zip accepts and zero reached modulus panics. Its true result is not proof that producer crt admitted the arrays.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

Euclid certificates and normalized modular operations compose through CRT gcd compatibility. Empty/mismatched CRT rejects but check_crt zip-truncates. Antecedents are congruence fibers; bare modulus helpers can panic on zero. Costs depend on bit widths and pairs.

Epoch36 parent evidence: zero_pair_agrees_with_independent_certificate exposed the original public gcd zero-zero panic, then passed with the guard. signed_grid_and_large_operands_preserve_euclid checks1088 signed grid pairs and selected big operands. Both pass debug/release jobs2; parent model.md#tests records source bindings and external logs. Modular-zero and CRT shape preconditions remain distinct.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

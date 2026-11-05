# math-comb-products

## descriptant

### math-comb-products

Balanced products build factorial/permutation/binomial; signed generalization and conversion-before-symmetry determine failure ordering. Legendre reconstructs prime powers using sieve. Exact outputs exclude differing products on bounded domain; unavailable checks do not. O(k) big multiplies plus sieve.

## approximant

### Scoped forward and cost

Balanced products build factorial/permutation/binomial; signed generalization and conversion-before-symmetry determine failure ordering. Legendre reconstructs prime powers using sieve. Exact outputs exclude differing products on bounded domain; unavailable checks do not. O(k) big multiplies plus sieve.

No executable percent surrogate claimed.

## invariant

### Scoped invariant

Balanced products build factorial/permutation/binomial; signed generalization and conversion-before-symmetry determine failure ordering. Legendre reconstructs prime powers using sieve. Exact outputs exclude differing products on bounded domain; unavailable checks do not. O(k) big multiplies plus sieve.

Source range[475,4466) bound by manifest digest. Parent domain applies, not arbitrary public states.

## models

# Source-scoped typed relations

## behavior_complement

factorial(n>MAX_N) excludes Ok; binomial(k<0) excludes Err and returns0. With nonnegative n and admitted k<=n, output must equal exact product ratio, not rounded f64.

## reverse_reconstruction

Result1 admits factorial0/1 and binomial(n,0)/binomial(n,n) for many n. For fixed nonnegative n, symmetry k and n-k can share a count; negative-n generalization adds signed fibers.

## guard_order

binomial checks negative k, then k->u64, then negative n recursion, then k>n, then symmetry and MAX_N. Oversized k can fail before a small symmetric complement is found.

## product_tree

prod_range(lo>hi)=1; spans<16 multiply serially, larger spans split m=lo+(hi-lo)/2 and multiply subproducts. prod_offset uses the same balanced traversal shifted by arbitrary Z base.

## legendre_relation

Independent factorial/binomial checks use sums floor(n/p^i) from sieve primes. binomial_legendre returns None for k>n or n>1e7, while check_binomial outside admitted n labels unavailable and accepts true.

## machine_boundary

catalan computes2*n and n+1 as u64 before arbitrary precision, so direct huge calls can overflow despite binomial work caps. Tests do not establish safety at every u64.

## tests

### Evidence

See ../model.md#tests for actual checks and source mutant; this fragment adds no runtime test.

Balanced products build factorial/permutation/binomial; signed generalization and conversion-before-symmetry determine failure ordering. Legendre reconstructs prime powers using sieve. Exact outputs exclude differing products on bounded domain; unavailable checks do not. O(k) big multiplies plus sieve.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read, parent AST reused; partial finite evidence only.

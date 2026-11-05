# algorithm-baby-step-giant-step

## descriptant

### baby_step_giant_step

Prospective model saved before implementation in model.md#prospective (relative from source directory). Original standard-library code. This epoch adds explicit supported domains and independent finite checks. See model.md for typed excluded-output and preimage relations. Partial syntax omits macro expansion and semantic resolution. No executable approximation profile is attached; useful cheaper metrics and evidence remain unsupported. No timing gain is claimed.

## model

# baby_step_giant_step mod.rs

## Forward

Fixed prime998244353. Solve g^x=t; n=ceil sqrt(p-1), store least j for g^j then scan t*(g^-n)^i. First matching block gives least exponent. Nonzero group exponent range0..p-2; zero base: t=1=>0,t=0=>1, otherwise none. O(sqrt(p)log p) BTreeMap work O(sqrt(p)) memory. Shanks, Class number, a theory of factorization, and genera (1971).

## Invariant

Store first baby exponent for each residue. Scan giant i ascending; i*n+j ranges disjoint increasing blocks, hence first match with least j is globally least.

## Cost

Fixed prime998244353. Solve g^x=t; n=ceil sqrt(p-1), store least j for g^j then scan t*(g^-n)^i. First matching block gives least exponent. Nonzero group exponent range0..p-2; zero base: t=1=>0,t=0=>1, otherwise none. O(sqrt(p)log p) BTreeMap work O(sqrt(p)) memory. Shanks, Class number, a theory of factorization, and genera (1971).

## behavior_complement

Some(x) with g^x!=target or with a smaller valid exponent is excluded. None is excluded when any exponent0..p-2 solves for nonzero g. Zero base explicitly adds exponent1 for target0.

## reverse_reconstruction

For output Some(0), every base paired with target1 is compatible. For x>0 retain pairs (g,g^x) whose multiplicative order exceeds x, plus (0,0) at x1. None admits nonmembers of generated subgroups, including (1,2).

## least_block

Store first baby exponent for each residue. Scan giant i ascending; i*n+j ranges disjoint increasing blocks, hence first match with least j is globally least.

## field_inverse

p998244353 is a fixed known prime. Nonzero g inverse is g^(p-2); no caller modulus or primality test. For g0 branch before inversion.

## resource_partition

n=ceil sqrt(p-1)=31596; fixed cap40000 is unreachable on current prime but explicit ResourceLimit differs from None. Table allocation uses standard policy; not an allocation-error channel.

## finite_arithmetic

All residues<p; products below2^60 and exponents<=n²+n fit u64. Congruent u64 inputs yield identical results; finite small exponent oracles are not proof of all discrete logs.

## evidence

#### Bounded review

An independent repeated-multiplication fixture checks a 17-element cycle,
all of its least exponents and congruent raw-input pairs against the actual
API, plus one nonmember and three zero-base branches. Two executions agree.
It rejects unique raw reconstruction and the false claim that the generator
is outside its own subgroup. This is finite evidence, not a complete proof;
standard allocator exhaustion and independent primality verification were not
exercised. The prospective hypothesis, Rust probe and logs are retained in the
cycle 15 external run directory.

### Evidence

## prospective

### Prospective model

Fixed prime998244353. Solve g^x=t; n=ceil sqrt(p-1), store least j for g^j then scan t*(g^-n)^i. First matching block gives least exponent. Nonzero group exponent range0..p-2; zero base: t=1=>0,t=0=>1, otherwise none. O(sqrt(p)log p) BTreeMap work O(sqrt(p)) memory. Shanks, Class number, a theory of factorization, and genera (1971).

Validation precedes arithmetic. Modular inputs normalize before products; products fit u64. Resource caps are policy refusals, distinct from algebraic absence. Allocator exhaustion is not recoverable in this bounded std implementation. No callbacks. Independent small oracles, repeated debug/release and compiled mutants are planned. No executable approximation profile is supported: no validated useful cheaper metric yet. Expected asymptotic improvement is a hypothesis, not measured speed.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Two tests per algorithm cover independent finite oracles and boundaries. Receipts: $MOVA_DATA/runs/octopus-wave25-algebra-2026-10-30. Paired debug/release logs, compiled-mutant rejection and restored rerun logs are retained there. Exact positive counts and final verification are reported in the integration receipt. Syntax projections are generated twice and compared. Source bindings are SHA-256; typed models are conditional specifications, not proofs. Allocator failure is not injected.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-11-01 · document

#### Review observation — 2026-11-01

The API first reduces both raw `u64` inputs modulo p. For nonzero residue g
of multiplicative order d, the output fiber `Some(k)` is characterized by
`0 <= k < d` and `t = g^k`; the raw input fiber also contains every pair
of representable integers congruent to g and t. Thus the output does not
reconstruct a unique raw input. The zero-residue branches must be kept
separate from the multiplicative-order statement. `None` includes targets
outside the generated subgroup and nonzero targets other than one when the
base residue is zero. `ResourceLimit` is a distinct error, unreachable under
the fixed constant and arithmetic shown here, not an ordinary no-solution
result. These are source-derived conditional relations, not a formal proof.

For a finite check, repeated multiplication independently establishes an
order-17 fixture and its 17 distinct residues, then checks least exponents
and congruent raw inputs through the actual API. It also checks one nonmember
and the zero branches. The false unique-raw-preimage claim fails for `(g,g)`
and `(g+p,g+p)`; treating g itself as a subgroup nonmember also fails. This
fixture does not establish correctness over all field elements, allocation
failure behavior, or primality independently.

### 2026-11-01 · evidence

#### Bounded review — 2026-11-01

An independent repeated-multiplication fixture checks a 17-element cycle,
all of its least exponents and congruent raw-input pairs against the actual
API, plus one nonmember and three zero-base branches. Two executions agree.
It rejects unique raw reconstruction and the false claim that the generator
is outside its own subgroup. This is finite evidence, not a complete proof;
standard allocator exhaustion and independent primality verification were not
exercised. The prospective hypothesis, Rust probe and logs are retained in the
cycle 15 external run directory.

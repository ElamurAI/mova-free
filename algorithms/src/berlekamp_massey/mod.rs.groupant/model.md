# algorithm-berlekamp-massey

## descriptant

### Modeling

Original implementation and finite oracle. For normalized finite s, find minimum L such that s[t]=sum(c[j]*s[t-1-j]) for L<=t<n. The connection polynomial starts with1. A nonzero discrepancy is canceled using a shifted prior polynomial and its nonzero discrepancy inverse; when 2L<=t update L=t+1-L. Return negated nonconstant coefficients in lag order. The concrete update schedule chooses one compatible solution, not all solutions.

The first epoch retains ambiguity explicitly: finite recurrence prefixes can have multiple minimum-order continuations, and convolution cannot uniquely identify operands. Independent Gaussian/matrix/direct-sum checks avoid relying only on reciprocal algorithms. Arithmetic is exact; there is no approximate production profile. Partial source projections do not resolve names, expand macros or prove semantics. Resource caps bound work but do not guarantee performance; allocator refusal is unforced.

## model

# berlekamp_massey mod.rs

## Forward

For normalized finite s, find minimum L such that s[t]=sum(c[j]*s[t-1-j]) for L<=t<n. The connection polynomial starts with1. A nonzero discrepancy is canceled using a shifted prior polynomial and its nonzero discrepancy inverse; when 2L<=t update L=t+1-L. Return negated nonconstant coefficients in lag order. The concrete update schedule chooses one compatible solution, not all solutions.

## Invariant

c[0]=b[0]=1; previous is nonzero; after each prefix the current connection polynomial annihilates every required index at its current order. The Massey length update tracks minimal compatible order. Finite Gaussian checks test this claim independently.

## Cost

For n<=4096, O(n²+n log p) modular operations and three O(n) vectors. Fixed prime inversion is bounded; copying full vectors keeps worst-case quadratic work even for smaller order. Cap is a resource policy.

## behavior_complement

Within validated calls and candidate coefficients, incompatible recurrences or a compatible recurrence of nonminimal order are absent. Invalid over-cap calls paired with Ok are absent. Compatibility/minimality alone is an outer description of possible output: deterministic tie choice imposes extra constraints.

## reverse_reconstruction

For observed c, retain all prefixes whose deterministic discrepancy schedule returns c. The looser algebraic fiber contains every prefix compatible with c and no shorter order, but may include prefixes selecting another tie. All-zero prefixes of every admissible length share empty output; uniqueness is absent.

## minimality_rank

For candidate order k form rows (s[t-1],...,s[t-k]|s[t]) for k<=t<n. Feasibility is rank(A)=rank(A|b); returned L is the least feasible k. Gaussian rank is an independent finite oracle; it neither identifies future terms nor claims uniqueness.

## finite_prefix_ambiguity

Prefix [1,2,3] has minimum order2 and admits [1,1] and [0,3], predicting5 and6 respectively at the next position. A chosen recurrence is a compatible continuation, not inference of a uniquely determined infinite source.

## lag_orientation

Return c[j]=-connection[j+1] modulo p so c[0] multiplies the immediately previous term. For positive L use initial=s[..L] with linear_recurrence::nth_term and modulus p. Empty L must be handled separately because nth_term rejects empty order.

## field_inversion

Every saved previous discrepancy is nonzero and p=998244353 is prime, so previous^(p-2) is its inverse. Composite or caller-selected moduli are outside this API. Zero discrepancy performs no inversion and only increments shift.

## normalization_overflow

All raw u64 values are reduced before multiplication. For p=998244353, (p-1)^2<2^60; multiplication fits u64, and addition/subtraction intermediates are below2p. With n<=4096, n+1,2L and shifts fit usize. Congruent representatives yield identical output.

## validation_partition

n>4096 returns TooManyTerms before any allocation; otherwise fallible reservations can return AllocationFailed. Empty prefixes still allocate connection arrays and may fail allocation. No allocator-failure injection or memory exhaustion guarantee is claimed.

## evidence

#### Independent review

A source-bound standalone Rust probe exercised 320 delayed-nonzero prefixes (lengths 1 through 64 and five nonzero raw representatives), independently checked the contradictory last equation for every shorter order, and checked the concrete selected coefficients and a congruent-input fiber. Two outputs matched. An initial hypothesis that the selected vector was all zeros failed immediately on [1]: the implementation returns [1]. The all-zero vector is compatible at order one, but compatibility does not specify the deterministic tie choice. Allocator failure, other prefixes and a formal source proof remain outside this observation. Raw probe, failed exploration, hypothesis and logs are retained in the independent review cycle directory under (see History).

### Evidence

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch, 2026-10-30` stays in descriptant as `### Modeling`.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Four tests per module pass twice debug and twice release with equal normalized identities. Independent production and oracle mutations compile and fail named assertions; exact source bytes are restored and positives rerun. Strict algorithm all-target Clippy and narrow formatting are checked. Raw logs: $MOVA_DATA/runs/octopus-wave11-numeric-2026-10-30/.

- berlekamp_massey-false-1.log: `3d0c05e1f337d6eec3620a0dcd4d81014c4099882f5b71529b0bf80ad2ba737f`

- berlekamp_massey-true-2.log: `128b20a6296b8e867006955772095ce97146c8c5feee18e9cbca549573a2f8cb`

- control-berlekamp_massey-mod.rs.log: `428009d17205e1508fae49cf8ad7337ae3acca42a7cf085e3812390231acce56`

### 2026-11-01 · descriptant

The 2026-11-01 independent review separated the vacuous order-n compatibility fiber of a delayed nonzero prefix from the implementation's selected vector. Its first all-zero-output hypothesis failed on a one-term prefix; the corrected source-derived schedule and bounded probe are recorded in the model and evidence. No production implementation or approximation changed.

### 2026-11-01 · validation_partition

#### Delayed nonzero prefix (review 2026-11-01)

For n>=1 and a nonzero residue a, the prefix 0^(n-1),a has minimum order n: for each k<n its last required equation has zero lag inputs and nonzero right side, whereas k=n has no required equations. The concrete discrepancy schedule returns [0^(n-1),a], even though every length-n coefficient vector is compatible with this prefix. Before the last term all discrepancies vanish; at the last, shift=n, previous=1, and the update puts -a in connection position n. Thus compatibility and minimality do not determine the chosen coefficients. For n=3, [0,0,1] is compatible with [0,0,0] but the API selects [0,0,1]. Congruent raw inputs share the output; this is a partial converse fiber, not a complete characterization. The separate all-zero prefix returns empty. This source argument assumes successful allocation and the declared prime field; finite probes are not a formal proof of the implementation.

### 2026-11-01 · evidence

#### Independent review, 2026-11-01

A source-bound standalone Rust probe exercised 320 delayed-nonzero prefixes (lengths 1 through 64 and five nonzero raw representatives), independently checked the contradictory last equation for every shorter order, and checked the concrete selected coefficients and a congruent-input fiber. Two outputs matched. An initial hypothesis that the selected vector was all zeros failed immediately on [1]: the implementation returns [1]. The all-zero vector is compatible at order one, but compatibility does not specify the deterministic tie choice. Allocator failure, other prefixes and a formal source proof remain outside this observation. Raw probe, failed exploration, hypothesis and logs are retained in the independent review cycle directory under `$MOVA_DATA/runs/`.

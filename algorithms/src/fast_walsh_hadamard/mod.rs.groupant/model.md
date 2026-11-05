# algorithm-fast-walsh-hadamard

## descriptant

### Modeling

Original implementation and finite oracle. For length n=2^d, H[k,i]=(-1)^popcount(k&i). Butterfly stages compute Hx, inverse scales by n^-1. XOR convolution is H((Ha)*(Hb))/n after zero extension to next_power_of_two(max lengths); empty operands return empty after caps.

The first epoch retains ambiguity explicitly: finite recurrence prefixes can have multiple minimum-order continuations, and convolution cannot uniquely identify operands. Independent Gaussian/matrix/direct-sum checks avoid relying only on reciprocal algorithms. Arithmetic is exact; there is no approximate production profile. Partial source projections do not resolve names, expand macros or prove semantics. Resource caps bound work but do not guarantee performance; allocator refusal is unforced.

## model

# fast_walsh_hadamard mod.rs

## Forward

For length n=2^d, H[k,i]=(-1)^popcount(k&i). Butterfly stages compute Hx, inverse scales by n^-1. XOR convolution is H((Ha)*(Hb))/n after zero extension to next_power_of_two(max lengths); empty operands return empty after caps.

## Invariant

Butterflies preserve residues in [0,p). H²=nI because distinct binary characters cancel. n is invertible since p is odd and n is a power of two. The chosen cap n<=2^20 also gives n<p. No primitive root assumption applies.

## Cost

For padded n, three transforms perform 3*n*log2(n)/2 butterflies, n pointwise products and n inverse scalings, plus O(log p) exponentiation; O(n) space. Direct transform is O(n log n+inverse*log p) and O(1) auxiliary storage. Padding and tiny inputs can lose against naive convolution.

## behavior_complement

Within validated arrays and candidate outputs, transform values unequal to the binary character sum and convolution values unequal to XOR-index sums are absent. Shape-invalid transform calls paired with mutation or Ok are absent. Finite oracle evidence is not a universal Rust proof.

## reverse_reconstruction

For a fixed length, H and normalized inverse uniquely recover field residues, but raw u64 representatives are ambiguous modulo p. For convolution output c, retain every pair (a,b) on the declared padded domain with Ha*Hb=Hc; factoring is generally nonunique, especially when spectra contain zeros.

## character_diagonalization

Characters obey chi_k(i XOR j)=chi_k(i)chi_k(j), yielding H(a XOR-convolve b)=Ha*Hb. This diagonalizes XOR convolution specifically; ordinary index addition would require different characters and padding.

## inverse_domain

H²=nI on binary index domains; n=2^d is nonzero in an odd prime field. n^-1 exists by Fermat. No primitive roots or n dividing p-1 premise is needed. Matrix and inverse tests exercise these equations independently.

## padding_shape

For nonempty inputs length is next_power_of_two(max lengths); indices i XOR j stay inside this domain. Trailing zeros remain in output. A singleton identity can thus increase the visible length of a non-power-of-two input. Empty operands map to empty only after length caps.

## bilinearity_mass

For equal padded domain, convolution is bilinear in residues and sum(c)=sum(a)*sum(b) modulo p. Neither property implies injectivity. Delta at index0 is the identity, while delta_i convolved with delta_i is delta0.

## overflow_validation

Transform checks nonzero power-of-two before cap, then normalizes. Convolution checks both caps before empty. Errors from validation leave transform input unchanged. Add/sub intermediates<2p and products<2^60 fit u64; bounded dimensions make doubling safe. Allocation refusal remains unforced.

## cost_crossover

Prospective model contrasts quadratic pair sums with3n log2(n)/2 butterflies plus linear products. Repeated release native timing at2,8,32,128,512 tests a dense crossover only; warmup/frequency variance and imbalance prevent universal speed assertions.

## evidence

### Evidence

Prospective prediction precedes timing. Three release rounds: at n=2 FWHT219–220ns vs naive25ns; n=8 315–327ns vs159ns; n=32 431–803ns vs1156–2292ns; n=128 1693–1720ns vs18312–18442ns; n=512 7698–8048ns vs280986–283379ns. Dense sampled crossover lies between8 and32 here. CPU frequency/warmup variance is visible; no universal gain or approximation claim.

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch, 2026-10-30` stays in descriptant as `### Modeling`.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Four tests per module pass twice debug and twice release with equal normalized identities. Independent production and oracle mutations compile and fail named assertions; exact source bytes are restored and positives rerun. Strict algorithm all-target Clippy and narrow formatting are checked. Raw logs: $MOVA_DATA/runs/octopus-wave11-numeric-2026-10-30/.

- fast_walsh_hadamard-false-1.log: `c0b25e759b236426f06e5240c6734834cc8b23e2166a6c1f030d2b87e6d4a6ea`

- fast_walsh_hadamard-true-2.log: `8df21583f5192bd96912426ab889f6997560f26d9fcbffb5c7b4f6c7a0217638`

- control-fast_walsh_hadamard-mod.rs.log: `6a19a171fbcf3c356ed7cfa632d09d2f547d04f6ee7b5d7b72190c2a097034ed`

Timing SHA256 `7a59dc9fba33456706c46b664076b3ff892b8ecfe2f1c9cebd8010f921adbc1a`; prospective SHA256 `a96a5a67d9a2f4aadb45a16247fc87e009897b44650160aef00b4649e3f3e236`.

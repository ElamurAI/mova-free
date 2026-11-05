# algorithm-fast-walsh-hadamard-tests

## descriptant

### Modeling

Original implementation and finite oracle. Execute deterministic finite assertions, comparing production observations with independent mathematical oracles. Pass is conjunction of the scheduled checks; failure may originate in production or oracle.

The first epoch retains ambiguity explicitly: finite recurrence prefixes can have multiple minimum-order continuations, and convolution cannot uniquely identify operands. Independent Gaussian/matrix/direct-sum checks avoid relying only on reciprocal algorithms. Arithmetic is exact; there is no approximate production profile. Partial source projections do not resolve names, expand macros or prove semantics. Resource caps bound work but do not guarantee performance; allocator refusal is unforced.

## model

# fast_walsh_hadamard tests.rs

## Forward

Execute deterministic finite assertions, comparing production observations with independent mathematical oracles. Pass is conjunction of the scheduled checks; failure may originate in production or oracle.

## Invariant

Finite fixtures preserve independent operands, domain and modular normalization. Tests do not establish universal semantics; allocations are not fault-injected.

## Cost

BM oracle enumerates3280 ternary prefixes through length7 and solves bounded Gaussian systems; FWHT oracle enumerates14641 vector pairs through length4. These finite budgets dominate fixture time; no whole-system cost claim.

## behavior_complement

In the universe of terminating finite fixture runs and pass/fail reports, absent pairs are false acceptance or false rejection relative to the assertion conjunction. Abort and nontermination are excluded; finite observations support only this scope.

## reverse_reconstruction

A pass admits all candidate implementations agreeing on the finite fixtures, including arbitrary off-fixture differences. Failure admits defective production or defective oracle. Neither observation uniquely recovers a cause.

## oracle_independence

Naive double summation groups pairs by i XOR j in u128, independently of butterflies. A separate character-matrix oracle checks transforms; inverse roundtrip alone would permit correlated errors.

## counterfactual_oracle

Compile a purpose-specific false oracle; its named assertion must fail. Production mutants are separately compiled and rejected. Restore exact source bytes before positive rechecks and syntax generation. Sensitivity applies only to selected controls.

## finite_schedule

Enumerate121 vectors over0,1,2 of lengths0..4 and14641 pairs. Compare dense character matrices at lengths1,2,4,8,32,128; test inverse, padding, XOR identity, validation immutability, full-width inputs and bilinearity.

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

- control-fast_walsh_hadamard-tests.rs.log: `0b1edb504df0675746ca9357b248c953aed89fac4fc640ef429af4f0c3224698`

Timing SHA256 `7a59dc9fba33456706c46b664076b3ff892b8ecfe2f1c9cebd8010f921adbc1a`; prospective SHA256 `a96a5a67d9a2f4aadb45a16247fc87e009897b44650160aef00b4649e3f3e236`.

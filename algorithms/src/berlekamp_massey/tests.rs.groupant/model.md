# algorithm-berlekamp-massey-tests

## descriptant

### Modeling

Original implementation and finite oracle. Execute deterministic finite assertions, comparing production observations with independent mathematical oracles. Pass is conjunction of the scheduled checks; failure may originate in production or oracle.

The first epoch retains ambiguity explicitly: finite recurrence prefixes can have multiple minimum-order continuations, and convolution cannot uniquely identify operands. Independent Gaussian/matrix/direct-sum checks avoid relying only on reciprocal algorithms. Arithmetic is exact; there is no approximate production profile. Partial source projections do not resolve names, expand macros or prove semantics. Resource caps bound work but do not guarantee performance; allocator refusal is unforced.

## model

# berlekamp_massey tests.rs

## Forward

Execute deterministic finite assertions, comparing production observations with independent mathematical oracles. Pass is conjunction of the scheduled checks; failure may originate in production or oracle.

## Invariant

Finite fixtures preserve independent operands, domain and modular normalization. Tests do not establish universal semantics; allocations are not fault-injected.

## Cost

The Gaussian oracle enumerates 3280 ternary prefixes through length 7 and checks candidate orders; other fixtures include a maximum-length zero prefix. No whole-system cost claim.

## behavior_complement

In the universe of terminating finite fixture runs and pass/fail reports, absent pairs are false acceptance or false rejection relative to the assertion conjunction. Abort and nontermination are excluded; finite observations support only this scope.

## reverse_reconstruction

A pass admits all candidate implementations agreeing on the finite fixtures, including arbitrary off-fixture differences. Failure admits defective production or defective oracle. Neither observation uniquely recovers a cause.

## oracle_independence

Row reduction solves Hankel consistency for every candidate order, using extended Euclid inversion rather than production discrepancy updates/Fermat inversion. Direct modular dot products independently check returned coefficients; the Kitamasa join is additional evidence only.

## counterfactual_oracle

Compile a purpose-specific false oracle; its named assertion must fail. Production mutants are separately compiled and rejected. Restore exact source bytes before positive rechecks and syntax generation. Sensitivity applies only to selected controls.

## finite_schedule

Enumerate sum(n=0..7)3^n=3280 prefixes, compare minimum feasible Gaussian order and direct compatibility. Also test lag orientation, a generated recurrence continuation, all-zero and delayed impulse, full-width normalization and cap boundary.

## evidence

#### Boundary review

The four existing focused tests pass with offline locked Cargo. A separate Rust probe calls the actual production API and directly checks recurrence equations on four boundary prefixes, including the omitted-order-zero false candidate and two compatible vectors with distinct next predictions. Its two outputs agree. This is a finite witness plus source-derived reasoning, not a proof of Gaussian elimination or all possible inputs. Allocation failure remains untested. External cycle 26 retains the probe, hypothesis and logs.

### Evidence

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch, 2026-10-30` stays in descriptant as `### Modeling`.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Four tests per module pass twice debug and twice release with equal normalized identities. Independent production and oracle mutations compile and fail named assertions; exact source bytes are restored and positives rerun. Strict algorithm all-target Clippy and narrow formatting are checked. Raw logs: $MOVA_DATA/runs/octopus-wave11-numeric-2026-10-30/.

- berlekamp_massey-false-1.log: `3d0c05e1f337d6eec3620a0dcd4d81014c4099882f5b71529b0bf80ad2ba737f`

- berlekamp_massey-true-2.log: `128b20a6296b8e867006955772095ce97146c8c5feee18e9cbca549573a2f8cb`

- control-berlekamp_massey-tests.rs.log: `d338355b83e199d736e3129ac54bca0ce4a0ef4eac437e9ce05ab66347632278`

### 2026-11-01 · descriptant

On 2026-11-01 the order-search boundary was reviewed: zero order is essential for zero prefixes, and order equal to prefix length is feasible by an empty equation set. This explains why the finite search has an endpoint without identifying a unique continuation. The former cost text also mentioned the unrelated FWHT test budget; the cost description now concerns this file only.

### 2026-11-01 · document

#### Boundary review, 2026-11-01

For a normalized prefix of length n and 0 <= k <= n, the oracle forms rows `(s[t-1], ..., s[t-k] | s[t])` for k <= t < n. Over the declared prime field, consistency is equivalent to a compatible order-k vector, assuming correct elimination and inversion. At k=0, consistency means every residue is zero; at k=n there are no rows, so feasibility is vacuous. The inclusive search therefore has a feasible endpoint. Omitting k=0 is falsified by `[0,0,0]`: the incorrect minimum is 1 instead of 0. A passing minimum-order assertion does not identify the coefficients or continuation: `[1,2,3]` admits both `[1,1]` and `[0,3]`, predicting 5 and 6 respectively. The converse of a passing test remains a fiber of implementations, and the complement above concerns fixture reports, not all incorrect recurrences. This reasoning does not prove the elimination implementation for all inputs or exercise allocation failure.

### 2026-11-01 · evidence

#### Boundary review, 2026-11-01

The four existing focused tests pass with offline locked Cargo. A separate Rust probe calls the actual production API and directly checks recurrence equations on four boundary prefixes, including the omitted-order-zero false candidate and two compatible vectors with distinct next predictions. Its two outputs agree. This is a finite witness plus source-derived reasoning, not a proof of Gaussian elimination or all possible inputs. Allocation failure remains untested. External cycle 26 retains the probe, hypothesis and logs.

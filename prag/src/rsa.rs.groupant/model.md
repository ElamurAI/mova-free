# rsa-literal-listener

## descriptant

# RSA literal-listener executable approximant

## Forward

Domain: 1..=64 worlds, 0..=64 utterances, rectangular truth matrix, integer priors 1..=1,000,000. Integer sums <=64,000,000 are exactly represented by binary64; reference L0 cannot overflow here. Other Game methods and fields are outside scope. Reject other inputs explicitly.

Metric: maximum elementwise relative error |approx-exact|/exact on positive cells; exact zero required on false cells and all-zero rows. Binary64 round-to-nearest gives reciprocal relative error <=u and multiplication relative error <=u, hence <=2u+u², u=2^-53. Use conservative 1e-12 fractional cap (1e-10 percent), assuming ordinary IEEE operations without fast-math. Reference fraction conversion incurs its own <=u rounding, so measured comparison allows 1e-12. The active budgets are 1%, 2%, 5% and 10%; all use this same tighter model. Classification: metric_kind=probability_relative, method_kind=floating_normalization, assurance_kind=analytical. This scoped classification describes the bounded L0 relation, not overall Mova accuracy; finite comparisons remain finite_observation evidence. Loosening accuracy adds no justified cost benefit.

## Invariant

Output shape matches truth; false cells and empty-support rows are exactly zero. True cells are positive and <=1. Supported rows sum to one within the same conservative numerical cap. Ordering within a row follows integer priors; exact ties stay ties. No claim about S1/L1, gradients, learned parameters or NLP accuracy.

## Behavior complement

Accepted-domain outputs cannot have negative, nonfinite or >1 cells, nonzero false cells, a different matrix shape, or positive mass on an empty support. Inputs outside the declared domain do not produce an approximation; they return a typed error.

## Reverse reconstruction

For a supported exact probability row p and known truth, compatible integer priors satisfy prior[i]/prior[j]=p[i]/p[j] for every true pair, within bounds. Common scaling is unidentifiable; false-world priors remain free. Approximate output admits all priors whose normalized true-world masses lie in the stated relative intervals, so no unique inverse is promised. Shared priors across rows intersect these constraints. Zero rows identify empty support under positive priors.

## Cost

Reference and approximant both have O(UW) output/time; reference performs many integer GCD/reduction operations. Candidate includes domain validation, exact u64 sums, one reciprocal per supported row, and at most one multiplication per cell. Allocation shape remains comparable. Expected constant-factor improvement is a prospective prediction, to be measured including validation/allocation, excluding compilation.

## Composition

The L0 component above uses the shared mova-approx ErrorBudget enumeration. Its own sidecar covers the whole L0 approximant file. Actual original rsa.rs is compiled directly into the external probe, so the comparison does not reimplement the reference formula.

## Epoch memory

## Epoch notes

## l0-bound-evidence

### RSA literal-listener executable approximant

#### Forward

Domain: 1..=64 worlds, 0..=64 utterances, rectangular truth matrix, integer priors 1..=1,000,000. Integer sums <=64,000,000 are exactly represented by binary64; reference L0 cannot overflow here. Other Game methods and fields are outside scope. Reject other inputs explicitly.

Metric: maximum elementwise relative error |approx-exact|/exact on positive cells; exact zero required on false cells and all-zero rows. Binary64 round-to-nearest gives reciprocal relative error <=u and multiplication relative error <=u, hence <=2u+u², u=2^-53. Use conservative 1e-12 fractional cap (1e-10 percent), assuming ordinary IEEE operations without fast-math. Reference fraction conversion incurs its own <=u rounding, so measured comparison allows 1e-12. The active budgets are 1%, 2%, 5% and 10%; all use this same tighter model. Classification: metric_kind=probability_relative, method_kind=floating_normalization, assurance_kind=analytical. This scoped classification describes the bounded L0 relation, not overall Mova accuracy; finite comparisons remain finite_observation evidence. Loosening accuracy adds no justified cost benefit.

#### Invariant

Output shape matches truth; false cells and empty-support rows are exactly zero. True cells are positive and <=1. Supported rows sum to one within the same conservative numerical cap. Ordering within a row follows integer priors; exact ties stay ties. No claim about S1/L1, gradients, learned parameters or NLP accuracy.

#### Behavior complement

Accepted-domain outputs cannot have negative, nonfinite or >1 cells, nonzero false cells, a different matrix shape, or positive mass on an empty support. Inputs outside the declared domain do not produce an approximation; they return a typed error.

#### Reverse reconstruction

For a supported exact probability row p and known truth, compatible integer priors satisfy prior[i]/prior[j]=p[i]/p[j] for every true pair, within bounds. Common scaling is unidentifiable; false-world priors remain free. Approximate output admits all priors whose normalized true-world masses lie in the stated relative intervals, so no unique inverse is promised. Shared priors across rows intersect these constraints. Zero rows identify empty support under positive priors.

#### Cost

Reference and approximant both have O(UW) output/time; reference performs many integer GCD/reduction operations. Candidate includes domain validation, exact u64 sums, one reciprocal per supported row, and at most one multiplication per cell. Allocation shape remains comparable. Expected constant-factor improvement is a prospective prediction, to be measured including validation/allocation, excluding compilation.

#### Composition

The L0 component above uses the shared mova-approx ErrorBudget enumeration. Its own sidecar covers the whole L0 approximant file. Actual original rsa.rs is compiled directly into the external probe, so the comparison does not reimplement the reference formula.

## measurements

### Bounded measurements

#### Current-policy checks

The shared runner's 31 example tests and strict all-targets clippy pass with active caps 1%, 2%, 5%, 10%. Included RSA checks compare actual production source, domain boundaries and 64 mixtures per cap; corruptions multiply a positive cell by 1.5 or inject nonzero mass into an empty row and are rejected for each active cap. Runner controls additionally halve actual outputs and insert NaN. These are correctness checks, not fresh timing measurements.

## rd-utility

# Entropy, utility and the converse of a speaker update

Zaslavsky, Hu and Levy, *A Rate–Distortion view of human pragmatic reasoning* (SCiL 2021, pp.347–348, https://aclanthology.org/2021.scil-1.32/) distinguishes an entropy-regularized objective from expected utility alone. The complete two-page extracted open text was consulted; extraction preserves neither mathematical layout nor figures reliably. The paper's analytic claims remain attributed to the paper, not independently formalized here. The conversation-update paper separately cited by the implementation is another source rather than a substitute for this result.

## Utility behavior complement

Domain: one world, two true utterances, prior 1, costs `[0,1]`, rationality 1; compare any admitted positive initial speaker to the source speaker update with fixed literal listener `[1,1]`. Observation: exact speaker probabilities, expected utility in bits, conditional entropy and their sum. This comparison does not assume that the arbitrary initial speaker was produced by a preceding Mova layer.

The source computes `S1=[2/3,1/3]`. With `V=[0,-1]` and an initial speaker `[99/100,1/100]`, expected utility decreases from `-1/100` to `-1/3`. Therefore the proposed behavior “a speaker update always increases raw expected utility” is excluded even on this tiny admitted domain. Its replacement objective is `G(S)=H2(S)+alpha*E_S[V]`. A finite Rust check that calls the actual module gives:

| Observation | Initial speaker | Source update |
| --- | ---: | ---: |
| Expected utility, bits | -0.010000000 | -0.333333333 |
| G, bits | 0.070793136 | 0.584962501 |

For fixed finite utilities and positive normalized probabilities, set `S*(u)=2^(alpha*V(u))/Z`. Algebra gives `G(S*)-G(S)=D_KL,2(S||S*)`; the general nonnegativity claim additionally uses Gibbs' inequality. This is a mathematical argument with stated assumptions, not a Lean proof or an execution-wide guarantee. Zero support, impossible utterances, overflow and changing listeners need their own domains. Current Mova exposes L0/S1/L1, not an arbitrary-depth recursion loop; this check makes no claim about the utility trajectory of such a loop.

## Utility-difference converse

Domain: alpha>0, fixed listener with positive support, two positive speaker probabilities, exact source arithmetic without overflow. Observation: the normalized speaker distribution. A compatible antecedent must satisfy `V(u)-V(v)=log2(S(u)/S(v))/alpha`. This reconstructs utility differences, not absolute costs or a unique listener.

Counterexample to unique reconstruction: the same actual source run with costs `[0,1]` and `[1,2]` produces `[2/3,1/3]` in both cases. A common cost offset cancels in normalization while the arithmetic remains defined. The converse is thus a preimage family; it is distinct from the utility behavior complement above. At alpha 0 this difference equation is unavailable, and zero probabilities require separate support reasoning.

## Evidence and limits

## History

### 2026-10-29 · Forward

Prospective model, 2026-10-29: Game::l0 forms p[u,w] = truth[u,w] * prior[w] / S[u], with S the sum of true-world priors. If S=0 the row is zero. Replace repeated reduced-u128 Ratio addition/division/GCD with integer row sums, one f64 reciprocal and multiplication. The source remains authoritative and its production callers are unchanged.

### 2026-10-29 · Epoch memory

2026-10-29: bounded L0 chosen because exact production reference is standalone Rust and preserves a meaningful numerical contract. Full RSA uses u128 fractions and can overflow outside bounded cases; no global replacement is made. Runtime measurements are recorded separately in model.md#measurements.

### 2026-10-29 · l0-bound-evidence

Prospective model, 2026-10-29: Game::l0 forms p[u,w] = truth[u,w] * prior[w] / S[u], with S the sum of true-world priors. If S=0 the row is zero. Replace repeated reduced-u128 Ratio addition/division/GCD with integer row sums, one f64 reciprocal and multiplication. The source remains authoritative and its production callers are unchanged.

#### Epoch memory

2026-10-29: bounded L0 chosen because exact production reference is standalone Rust and preserves a meaningful numerical contract. Full RSA uses u128 fractions and can overflow outside bounded cases; no global replacement is made. Runtime measurements are recorded separately in model.md#measurements.

### 2026-10-29 · measurements

#### Retired-policy measurements, 2026-10-29

The historical table below used the retired 1%, 5%, 15%, 25% policy. Active caps are 1%, 2%, 5%, 10%; the table is preserved as dated evidence and is not a measurement of the new profiles. The numerical L0 implementation remains identical across profiles.

2026-10-29, rustc 1.99.0, optimized standalone Rust on CPU. The probe includes the actual unmodified prag/src/rsa.rs source and the executable approximant; no reconstructed reference. Compilation is excluded from timing. Both implementations allocate their complete result; the candidate validates its domain on every call.

Two executions each compared 194 games per budget (world sizes 1,2,3,8,32,64; utterance counts 0,1,7,64; eight deterministic seeds, plus production scalar/reference examples). Maximum relative error was identically 2.21125027271913505e-16 for each budget and both runs. This finite measurement is distinct from the analytical conditional 1e-12 bound. Zero cells match exactly; supported row normalization and range were checked.

Benchmark: 64 worlds x 64 utterances, deterministic mixed truth and priors, 10 warmups, seven repetitions of 300 calls, alternating execution order, black_box inputs/outputs. Median nanoseconds per call:

| budget | reference run 1 | candidate run 1 | ratio 1 | reference run 2 | candidate run 2 | ratio 2 |
|---|---:|---:|---:|---:|---:|---:|
| 1% | 314999.3 | 3915.1 | 80.46 | 314593.4 | 3937.5 | 79.90 |
| 5% | 315241.0 | 3918.4 | 80.45 | 314566.6 | 3925.4 | 80.14 |
| 15% | 314933.3 | 3921.0 | 80.32 | 314584.8 | 3934.5 | 79.96 |
| 25% | 314888.5 | 3923.5 | 80.26 | 314695.7 | 3934.0 | 79.99 |

These are local kernel speedups, not whole-parser or training speedups. All budget profiles execute the same arithmetic; timing differences are noise. The external exploratory probe is not an installed API; the common approximation runner supplies the maintained comparison path.

The final sidecar test binary passed 10 tests, including 7 original RSA tests, all-profile domain/extrema tests, and 64 actual-reference mixtures per profile. Negative controls deliberately multiply a positive output by 1.5 and inject mass into an all-zero row: the same comparison gate rejects both for all four budgets. Test module imports the actual production source by relative path. Run the shared runner tests to repeat after integration; standalone compilation can also link the mova-approx rlib to this sidecar with rustc --test.

Untested: all possible domain points, platform timing portability, full RSA S1/L1 composition, neural gradients and end-task scores. Source and test hashes are bound by the manifests; measurements do not establish these missing properties.

### 2026-10-30 · Composition

2026-10-30: this source groupant additionally attaches a separate bounded Game::s1 companion, s1_approximant.rs. Its adjacent descriptant supplies its own narrower domain, exact-reference overflow proof, forward error propagation through exponentiation and normalization, typed behavior complement, reverse reconstruction and cost model. The original four L0 profiles remain intact beside four S1 profiles. The S1 implementation computes its own L0 values rather than composing the earlier L0 API. This establishes no L1 or downstream decision bound. Semantic AST and the remaining RSA behavior stay explicit gaps.

### 2026-10-30 · l0-bound-evidence

2026-10-30: this source groupant additionally attaches a separate bounded Game::s1 companion, s1_approximant.rs. Its adjacent descriptant supplies its own narrower domain, exact-reference overflow proof, forward error propagation through exponentiation and normalization, typed behavior complement, reverse reconstruction and cost model. The original four L0 profiles remain intact beside four S1 profiles. The S1 implementation computes its own L0 values rather than composing the earlier L0 API. This establishes no L1 or downstream decision bound. Semantic AST and the remaining RSA behavior stay explicit gaps.

### 2026-10-30 · measurements

Heading `#### Current-policy checks, 2026-10-30` stays in measurements as `#### Current-policy checks`.

### 2026-11-01 · document

#### Entropy and utility boundary, 2026-11-01

[The source-bound RD/RSA note](model.md#rd-utility) adds a specialized behavior complement for raw-utility monotonicity and a separate utility-difference converse. An actual-module finite counterexample lowers expected utility while increasing the entropy-regularized objective; a cost-shift control shows that normalized speaker outputs do not reconstruct unique costs. This is not an arbitrary-depth recursion test or a formal proof.

### 2026-11-01 · Evidence and limits

On 2026-11-01 a standalone optimized Rust harness imported the actual module by path. Exact `Ratio` assertions checked `[2/3,1/3]` and cost-shift ambiguity. Binary64 entropy checks observed the values above and the strict directions of both changes; two executions produced identical output. External harness SHA256: `f40351d5517762a9feea4f396b6024e3d19d5f165e043689a623b8ad2efbd989`. Source and executable receipts stay in external run storage. The harness is a finite counterexample, not an exhaustive experiment, throughput measurement, human-behavior validation or formal proof. No percentage error claim is admitted.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

### 2026-11-04 · document

Source: `prag/src/rsa.rs`, SHA256 `8d36c019806248ad15a2f90f7b11c09cb6881bae588818d7d0ef1d6d8a9a9ab1`. This note concerns the exact small closed-set `Game::s1` implementation, with costs in bits. It does not implement recursive RD-RSA or change production behavior.

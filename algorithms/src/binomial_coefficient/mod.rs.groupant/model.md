# algorithm-binomial-coefficient

## descriptant

### binomial_coefficient mod.rs groupant

Raw Rust is authoritative outside this sidecar. Original implementation and independent tests; mathematical citations live in the production source and forward model. Source syntax projection is partial: no macro expansion, semantic resolution or complete dependency graph.

Composition: the forward recurrence/sweep, invariants, behavior complement and distinct reverse preimages constrain the same call. The test groupant observes production through independent specification and metamorphic projections; passing leaves alone do not prove parent wiring. No useful approximate semantics are selected; no executable surrogate or substitution claim.

## approximant

### Forward, invariant and cost

For u64 n,k with k<=n define r=min(k,n-k). Start c=1, update c_i=c_(i-1)*(n-r+i)/i for i=1..r. Let g=gcd(n-r+i,i); divide numerator and denominator by g. Reduced denominator divides c_(i-1) because the binomial recurrence is integral and remaining numerator is coprime. Divide c before checked multiplication; that product is exactly c_i. These coefficients increase along this path, so an overflow proves the final coefficient exceeds u128::MAX. The implementation cannot reject a representable result due to a larger transient product. k>n returns InvalidK before subtraction. O(r log(n+1)) word operations, O(1) auxiliary space; overflowing calls stop early (c_i>=2^i for i<=r, so at most 128 iterations reach failure). Fixed-width arithmetic costs are treated as bounded machine operations. No allocation. Canonical mathematical identity: NIST DLMF section 26.3, https://dlmf.nist.gov/26.3 . Original implementation, no copied code. Exact integer output currently has no approximate companion.

## models

# Typed relations for binomial_coefficient mod.rs

Subject/revision: adjacent source SHA-256 in manifest.json. Scope: one pure function call (tests: finite harness run). Status: source-derived partial relational specification, with finite Rust evidence. Mathematical relations assume successful allocation unless modeling errors. No semantic proof or automatically executable inverse is claimed. Each section defines its operator and domain; checks and remaining gaps appear in model.md#tests.

## behavior_complement

Universe: u64 n,k and Result<u128,Error>. k>n excludes every Ok and Overflow. Otherwise C(n,k)<=u128::MAX excludes both errors and every wrong Ok value; larger C excludes every Ok and InvalidK. Inner exclusion specification derived from integral cancellation, checked independently on finite Pascal rows.

## reverse_reconstruction

For observed Ok(c), preimage is {(n,k):0<=k<=n, C(n,k)=c}, constrained to u64 n and representable c. Symmetric pairs (n,k),(n,n-k) share output. c=1 contains all edge pairs and is nonunique. Overflow has the C(n,k)>u128::MAX fiber, InvalidK the k>n fiber. No unique inverse is claimed.

## symmetry

Holding n fixed, k and n-k have identical exact outcomes for k<=n. Loop reduction realizes the same path, including true-overflow behavior. Invalid k is outside the transformation domain.

## pascal_relation

For 0<k<n, C(n,k)=C(n-1,k-1)+C(n-1,k). Checked addition of representable children predicts the parent; if either child overflows, positivity proves parent overflow. Independent additive oracle does not use multiplicative cancellation.

## row_mass

For n<=127, all coefficients and their sum fit u128, and summing k=0..n yields 2^n. This scalar projection detects mass errors but is not sufficient to validate individual coefficients. At n=128 the mathematical sum is unrepresentable although individual terms can fit.

## cancellation_integrality

At each successful iteration, reduced denominator divides c exactly. This is a divisibility invariant, not a rounding approximation. Removing division or multiplying the unreduced factors can violate exactness or create false overflow; the 131 choose 65 boundary is a falsifiable case.

## overflow_monotonicity

Along r=min(k,n-k), ratio (n-r+i)/i>=2 for i<=r; c_i therefore grows, so once checked reduced product overflows no later coefficient could return into range. No saturation or wrapping is allowed.

## edge_fibers

For n>=1, k=1 or k=n-1 yields n exactly; k=0 or k=n yields 1 for every n including zero. u64::MAX choose 2 is representable in u128, while choose 3 overflows. These fibers test full input width independently of small exhaustive rows.

## tests

Five tests in algorithms/src/binomial_coefficient/tests.rs exercise independent finite oracles, boundaries and metamorphic relations. External evidence directory: (see History). See model.md#validation for measured outcomes. Allocation refusal is not injected; successful reservations are a premise. No teacher, API, GPU or whole-system accuracy claim.

### Checks and evidence

## validation

### Validation

All 266 algorithm-library tests passed twice each in debug and release after the final edit; this includes five new tests per algorithm, 15 total. Sorted test-name/outcome records were identical across all four runs. Strict algorithm-library Clippy with all targets and -D warnings passed. Focused rustfmt check passed. The six new source syntax projections were generated twice and compared byte-for-byte. Shared library exports used for execution were restored before commit. These are finite CPU correctness observations, not speed measurements or a full-system proof.

Independent oracle counts: subset_sum checks 114,681 vector/target cases against subset masks and validates every returned witness; binomial_coefficient compares 45,451 coefficients against checked additive Pascal rows through n=300; interval_union checks 54,241 interval lists against unit-cell coverage. Boundaries separately exercise target cap, u64::MAX coefficients, cancellation at 131 choose 65, invalid k, first reversed interval and the full signed endpoint span.

Three production-source mutants compiled, then failed real assertions: reachable subsets rejected, binomial accumulator initialized to 2, touching intervals left separate. The interval mutant initially survived the focused nested/overlapping fixture because overlaps bridged the touching boundary; a direct [0,1),[1,2) case was added, and the same mutant now fails. The initial survival log is retained externally. The exhaustive interval oracle already covered touching-only cases; the correction makes the focused control diagnostic.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30, prospective prediction: cancellation prevents representable binomial intermediate overflow; descending subset updates forbid index reuse; signed widening and touching merge preserve interval coverage. Expected complexity is stated in model.md#approximant before final validation. Finite checks can refute these expectations; they do not establish a universal theorem. Preserve these premises and review model.md#validation when updating the source.

### 2026-10-30 · tests

Five tests in algorithms/src/binomial_coefficient/tests.rs exercise independent finite oracles, boundaries and metamorphic relations. External evidence directory: octopus-wave7-discrete-2026-10-30. See model.md#validation for measured outcomes. Allocation refusal is not injected; successful reservations are a premise. No teacher, API, GPU or whole-system accuracy claim.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Source SHA-256: `4638904c39ca883eaa2dc45dd9fa888df2947351feb3599f81dbf5f7527f7811`. Evidence directory basename: `octopus-wave7-discrete-2026-10-30` (external run storage).

Normalized debug record SHA-256: `3fbabf34d336c176185133e60994376de2245a94ce5c0642bd7eefd44fa8fd1c`. Mutation failure log SHA-256 for binomial_coefficient: `0881eca797c2c55e08ee7ca747a62000fc911373c71b302db3740250d3b1f41a`. Syntax SHA-256: `cce94885a8c7437cfd5316ad2867785f429e6080ef11f06be60745823f5d8471`. Allocation refusal and machine exhaustion were not injected. Models remain partial, with no automatic inverse or approximate production substitution.

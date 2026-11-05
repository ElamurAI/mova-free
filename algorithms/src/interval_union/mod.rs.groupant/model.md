# algorithm-interval-union

## descriptant

### interval_union mod.rs groupant

Raw Rust is authoritative outside this sidecar. Original implementation and independent tests; mathematical citations live in the production source and forward model. Source syntax projection is partial: no macro expansion, semantic resolution or complete dependency graph.

Composition: the forward recurrence/sweep, invariants, behavior complement and distinct reverse preimages constrain the same call. The test groupant observes production through independent specification and metamorphic projections; passing leaves alone do not prove parent wiring. No useful approximate semantics are selected; no executable surrogate or substitution claim.

## approximant

### Forward, invariant and cost

Validate all original input pairs in order; the first start>end yields ReversedInterval{index} before allocation. Reserve n pairs fallibly, copy nonempty pairs and sort lexicographically. Sweep in place: merge current (s,e) into last (a,b) iff s<=b, replacing b by max(b,e); otherwise start a new component. Processed-prefix invariant: written components are the exact union of read pairs, each has start<end, and consecutive components have strict gaps. Write index never exceeds read index, so unread values survive. Length is sum of (i128(end)-i128(start)) cast to u128; each difference is nonnegative and total cannot exceed u64::MAX because components are disjoint within the signed i64 span. O(n log(n+1)) time, O(n) reserved pairs, O(1) extra sweep state. Sorting allocates no extra buffer. Allocation failure is explicit but not injected. Original sweep implementation; general background de Berg et al., Computational Geometry, third edition (2008), chapter 2. No approximate companion: topology and exact endpoints have no chosen error metric.

## models

# Typed relations for interval_union mod.rs

Subject/revision: adjacent source SHA-256 in manifest.json. Scope: one pure function call (tests: finite harness run). Status: source-derived partial relational specification, with finite Rust evidence. Mathematical relations assume successful allocation unless modeling errors. No semantic proof or automatically executable inverse is claimed. Each section defines its operator and domain; checks and remaining gaps appear in model.md#tests.

## behavior_complement

Universe: input pairs and Result<Union,Error>. A reversed pair excludes Ok; first reversal fixes the sole domain-error index. With valid input and allocation success, outputs with empty/reversed components, touching consecutive components, wrong coverage or wrong length are excluded. This inner exclusion specification uses set equality and canonical topology, not sampling absence.

## reverse_reconstruction

For observed canonical intervals U, compatible valid inputs are all finite interval lists whose point-set union equals U; arbitrary permutation, repetitions, contained intervals and empty intervals preserve the observation. Output is not an inverse of the original list. Length alone has a larger fiber: translations with representable endpoints preserve it but change U.

## permutation

For valid inputs with successful storage, every permutation yields identical Union. For invalid inputs, the first reversed index may change, so the relation deliberately excludes errors. Lexicographic sorting removes input-order dependence.

## idempotence

Feeding canonical result intervals back into the function yields exactly the same intervals and length conditional on allocation. Strict gaps and positive component lengths make each component a fixed point of the sweep.

## empty_and_duplicate

Appending valid empty intervals or repeated existing intervals preserves canonical union and length. Validation still precedes filtering: reversed intervals are never equivalent to empty intervals.

## translation

For an integer offset d such that every endpoint+d remains in i64, translating inputs translates each output endpoint and preserves covered_length. Overflowing translations are outside the domain, not wrapped tests.

## split_join

Replacing [a,c) by [a,b),[b,c) for a<=b<=c preserves exact output, including equality boundaries. This relation requires touching intervals to merge, whereas a strict gap must remain. A mutation replacing <= by < must fail.

## length_bound

For all valid input with successful allocation, 0<=covered_length<=2^64-1, and equality is achieved by [i64::MIN,i64::MAX). Widen before subtracting; signed i64 subtraction fails on that legitimate span. Length zero iff the canonical interval list is empty.

## tests

Five tests in algorithms/src/interval_union/tests.rs exercise independent finite oracles, boundaries and metamorphic relations. External evidence directory: (see History). See model.md#validation for measured outcomes. Allocation refusal is not injected; successful reservations are a premise. No teacher, API, GPU or whole-system accuracy claim.

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

Five tests in algorithms/src/interval_union/tests.rs exercise independent finite oracles, boundaries and metamorphic relations. External evidence directory: octopus-wave7-discrete-2026-10-30. See model.md#validation for measured outcomes. Allocation refusal is not injected; successful reservations are a premise. No teacher, API, GPU or whole-system accuracy claim.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Source SHA-256: `6108b36ccd9c0afe4f4ce1311dadd0dceb6826d4ffb06e5388c6cfef79c2ae0d`. Evidence directory basename: `octopus-wave7-discrete-2026-10-30` (external run storage).

Normalized debug record SHA-256: `3fbabf34d336c176185133e60994376de2245a94ce5c0642bd7eefd44fa8fd1c`. Mutation failure log SHA-256 for interval_union: `37568f298a8128b828ac92828f80163a3915a7bb5be4b642c65edf976bc32fd9`. Syntax SHA-256: `a621422a740542b7211a9fe8223d2fbee6a7d4710b0613664dd80468b63906e7`. Allocation refusal and machine exhaustion were not injected. Models remain partial, with no automatic inverse or approximate production substitution.

# algorithm-binomial-coefficient-tests

## descriptant

### binomial_coefficient tests.rs groupant

Raw Rust is authoritative outside this sidecar. Original implementation and independent tests; mathematical citations live in the production source and forward model. Source syntax projection is partial: no macro expansion, semantic resolution or complete dependency graph.

Composition: the forward recurrence/sweep, invariants, behavior complement and distinct reverse preimages constrain the same call. The test groupant observes production through independent specification and metamorphic projections; passing leaves alone do not prove parent wiring. No useful approximate semantics are selected; no executable surrogate or substitution claim.

## approximant

### Forward, invariant and cost

The deterministic test harness enumerates fixed inputs, computes independent expected outcomes, calls the production function, and asserts equality or witness validity. Pass means every executed assertion holds; panic identifies a violated predicate. Fixtures and input order are invariant. Oracle domains: subset vectors length 0..6 over 0..3, targets 0..20 (114,681 cases); Pascal rows n=0..300 (45,451 coefficient comparisons); interval lists length 0..4 over the 15 valid endpoint pairs in -2..2 (54,241 lists). Only the domain relevant to this source is executed. Cost is fixture count times production plus oracle work; subset oracle scans up to 2^n masks, Pascal uses O(n) row storage, cell oracle checks four unit cells against each list. Additional boundary and metamorphic checks are finite. No timing or speedup claim.

## models

# Typed relations for binomial_coefficient tests.rs

Subject/revision: adjacent source SHA-256 in manifest.json. Scope: one pure function call (tests: finite harness run). Status: source-derived partial relational specification, with finite Rust evidence. Mathematical relations assume successful allocation unless modeling errors. No semantic proof or automatically executable inverse is claimed. Each section defines its operator and domain; checks and remaining gaps appear in model.md#tests.

## behavior_complement

Universe: harness runs on the finite fixtures declared in tests.rs. Passing is excluded if any asserted oracle, boundary, witness or metamorphic equation disagrees. Missing cases outside those finite sets remain unknown, and a pass is not proof of all input behavior.

## reverse_reconstruction

Observed failure identifies the set of candidate production/harness faults capable of falsifying that assertion, not a unique defect. Observed pass admits all implementations agreeing on observed projections of the fixture set; behavior outside that set is unconstrained. Assertion context and fixture source restrict compatible antecedents.

## oracle_relation

The exhaustive check compares production against a separately constructed specification: subset masks, additive checked Pascal rows, or unit-cell coverage, as appropriate to this source. Fixed finite domains are explicit in the Rust loops; checked Pascal None propagates mathematical overflow. Agreement supplies finite correspondence only.

## metamorphic_relation

Paired fixtures constrain symmetry/scaling/permutation or translation/idempotence while preserving their stated representability and allocation assumptions. These observations supplement the independent oracle; they cannot establish general correctness alone.

## mutation_separation

A compiled production mutation must make a real assertion fail: reject positive reachable subsets, change binomial initial value, or split touching intervals. The accepted original and rejected mutant separate those faults on this harness domain, but do not prove mutation completeness.

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

Source SHA-256: `d4a312a0204704f381fd2f32b74f6fc9017b45d3a6d7b99de901415bb244b5f8`. Evidence directory basename: `octopus-wave7-discrete-2026-10-30` (external run storage).

Normalized debug record SHA-256: `3fbabf34d336c176185133e60994376de2245a94ce5c0642bd7eefd44fa8fd1c`. Mutation failure log SHA-256 for binomial_coefficient: `0881eca797c2c55e08ee7ca747a62000fc911373c71b302db3740250d3b1f41a`. Syntax SHA-256: `7caf02a05cc28b2e19a24c7b8deacefa6c673ccad9eeff8c9eb1d304dfd4aa26`. Allocation refusal and machine exhaustion were not injected. Models remain partial, with no automatic inverse or approximate production substitution.

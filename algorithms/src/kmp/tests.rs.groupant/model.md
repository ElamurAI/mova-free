# algorithm-kmp-tests

## descriptant

### kmp test groupant

Combiner: source identity plus oracle construction plus assertion wiring form
a conditional finite-check model. Typed models answer separate questions; no
additive inverse identity is assumed. Syntax is partial, macros opaque.
A passing test does not establish parent wiring or full-domain correctness.

## approximant

### Forward, invariant and cost

Generate deterministic fixtures, compute the independently structured oracle,
compare production output, and fail on disagreement. Property loops hold their
listed assumptions fixed while changing order, padding, or a third sequence.
Invariant: each checked fixture has its reference computed without consulting
the output being tested; seeds advance by explicit wrapping arithmetic.
The model.md#models bounded_coverage_cost section gives algorithm-specific costs.
Assertion execution supplies bounded evidence; it does not prove the production
recurrence, all inputs, or source correctness from sidecar validity.

## models

# KMP test models

Subject: tests.rs at manifest SHA; observation: one test invocation. Domain is
the deterministic fixture family in this source and normal allocation.
All models are partial source-derived relations with executed bounded evidence.

## behavior_complement

Universe U=(fixture, candidate result, test verdict). Given naive window result
W, verdict Pass with candidate != W is absent from the oracle-assertion graph.
This is an exact complement slice for this assertion, not a proof that all
wrong algorithms fail the whole finite suite. Overlap, order, boundary and
byte equality errors are observable. Lost-overlap fallback mutant is rejected
by the new wide-byte test; no universal implementation rejection is claimed.

## reverse_reconstruction

Given a failing equality (actual A, expected W), compatible implementations
are every function f with f(text,pattern)=A and A!=W at that fixture.
All other fixture outputs remain unconstrained unless observed separately.
A passing run's preimage includes every implementation agreeing on the tested
finite domain, including overfit implementations. No unique source recovery.
Evidence: compiled mutant provides one member of a failure fiber.

## independent_oracle

The naive relation scans each valid start using slice equality; it does not
reuse prefix tables or fallback states. Empty patterns enumerate boundaries.
Agreement is checked on 3,937 old binary pairs and 2,564 new cases: 256 texts
times ten sampled patterns, three periodic cases and one long fallback case.
Matching implementations can still share specification misunderstandings.

## property_assumptions

For nonempty binary p and t, padding t by k sentinel bytes 254 on the left
and two on the right gives W(padded,p)={i+k|i in W(t,p)}. Sentinel exclusion
and nonempty p prevent extra boundary matches; removing either guard invalidates
the rule. The test checks 127 texts x14 patterns x3 paddings=5,334 relations.
The expected offsets come from the naive oracle, not the production algorithm.

## bounded_coverage_cost

The source fixes all seed arithmetic with wrapping u64 operations. New generated
texts have 64 bytes over {0,1,127,128,255}, sampled patterns lengths 0..9;
periodic and fallback fixtures add adversarial structure. Naive time per pair
is O(n*m), KMP O(n+m+z), memory O(n+m+z). Finite bounded tests cannot establish
all byte strings, all lengths, allocation behavior or amortized complexity.

## checks

### Evidence

Cargo crate library suite passed twice, single-threaded: 36 tests at this branch.
This file adds two tests to its preserved old suite. Strict Clippy on all crate
targets and cargo fmt --check passed. External scratch compiled a changed
production source with this exact test file; a filtered new test failed with
exit 101. KMP reset overlap fallback, Levenshtein doubled substitution cost,
and knapsack used ascending capacities. Only the respective mutant applies
to each file. No mutated code is committed. See model.md#models for exact coverage.

Repeated output comparison excludes Cargo build lines and elapsed-time footer.
Wall time is not an algorithm benchmark. New checks remain deterministic and
bounded, and no full input-space proof or allocation-failure coverage is claimed.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29: prospective expectations were saved outside the repository
before extraction. Existing inline tests were moved; two new tests extend
coverage. Production source prefix remained byte-identical. The new fixtures
passed twice and a specifically filtered NEW test rejected a compiled semantic
source mutant. Exact domains, assumptions and limitations are in model.md#models.

### 2026-10-29 · checks

Heading `### Evidence, 2026-10-29` stays in checks as `### Evidence`.

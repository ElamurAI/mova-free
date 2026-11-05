# algorithm-levenshtein-tests

## descriptant

### levenshtein test groupant

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

# Edit-distance test models

Subject: tests.rs at manifest SHA; observation: one test call on fixed fixtures.
Assume allocation succeeds and stable equality. Partial source-derived models
have bounded executed evidence; no machine-domain proof is claimed.

## behavior_complement

Universe U=(fixture, proposed distance d, verdict). For the independent full
table T, Pass with d!=T is absent in oracle assertions. Unicode metric assertions
also exclude zero for unequal sequences, asymmetry, and a triangle violation
on the tested domain. This is an exact complement slice of the assertion graph;
it does not exclude an implementation agreeing only on finite fixtures.
The new larger-input test rejects doubled substitution costs.

## reverse_reconstruction

For observed (fixture,d), the compatible implementation fiber is all functions
mapping that fixture to d. A passing transcript restricts those functions only
on the tested inputs and checked relations. A failure gives a counterexample,
not a unique buggy operation or original source. The compiled doubled-cost
mutation demonstrates one compatible failing implementation.

## independent_oracle

The full table retains every D(i,j) with independent initialization and
insertion/deletion/substitution recurrence; it shares no production row buffers.
Check 3,969 old binary pairs and 512 new byte pairs of lengths 0..16 versus
0..18. Unicode characters are injectively encoded to three byte labels; 169
pairs compare production char distance with table byte distance. Shared unit-edit
specification assumptions remain explicit.

## property_assumptions

For ordinary unit-cost edit distance under equivalence equality,
D(a,b)<=D(a,c)+D(c,b), symmetry and D(a,b)=0 iff a=b.
All 13 words of lengths 0..2 on {é,🙂,combining acute} yield 2,197 triangles.
The char-to-byte relabeling is injective on this exact alphabet; arbitrary
encoding or Unicode normalization is not assumed. A decomposed/precomposed
witness checks the lack of automatic normalization.

## bounded_coverage_cost

Seeded 512 byte pairs use wrapping u64 arithmetic and alphabet {0,127,128,255}.
Full-table cost O((n+1)(m+1)) space/time; triangle loop calls three bounded
distances per triple. Empty and maximal zero-sized-element guards remain old
tests. Finite enumeration does not prove metric laws for arbitrary Eq types
or characterize allocation failure.

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

# algorithm-aho-corasick-tests

## descriptant

### aho_corasick tests.rs epoch memory

Four tests: 225 ordered pattern pairs, each with duplicate first pattern, across 127 binary texts (28,575 oracle comparisons); full byte range; failure-output/empty/duplicate boundaries; injective relabel and ID permutation; five compatible predecessors and prefix extension. Compiled production mutant removing inherited outputs fails boundaries_failure_outputs_and_duplicates.

The combiner relates authoritative source and its SHA-256, partial syntax, forward relation, typed obernants, invariant, cost and finite evidence. Models preserve ambiguity in compatible predecessors. Missing semantic AST analysis, universal proof, allocator exhaustion and downstream composition remain explicit. Exact production algorithms need no fabricated executable approximation. Any future approximate profile must honor supported 1%, 2%, 5%, 10% caps and bind source and evidence.

## model

# aho_corasick tests.rs model

## Forward

Run deterministic independent byte oracles, boundary cases, transformation laws and compatible-output fibers; assert exact values and explicit false-premise counterexamples. Passing finite checks does not establish universal correctness.

## Invariant

Oracle logic does not use production failure links. All fixtures are deterministic; comparisons include output order, offsets and multiplicity. Historical external mutation evidence is separate from the tests.

## Cost

Bounded exhaustive fixture enumeration; the oracle scans each pattern at each boundary and compares the corresponding byte window. Test cost is intentionally independent of production claims.

## nonbehavior

Type: behavior_complement. Domain: Finite deterministic fixtures and test executions; no arbitrary programs or hidden production state.

Exclude acceptance when an asserted production output differs from the independent oracle, a stated conditional law fails under its premises, or a compatible-output fiber differs from direct constraints. Unvisited inputs are outside this finite acceptance claim.

Source-derived partial relation with finite evidence, not universal verification.

## preimages

Type: reverse_reconstruction. Domain: Finite deterministic fixtures and test executions; no arbitrary programs or hidden production state.

A passing observation admits every implementation agreeing on the tested fixture observations. Many implementations may differ elsewhere; test success cannot uniquely reconstruct source or establish universal correctness.

Source-derived partial relation with finite evidence, not universal verification.

## oracle-agreement

Type: correspondence_relation. Domain: Finite deterministic fixtures and test executions; no arbitrary programs or hidden production state.

Direct byte-window equality supplies the independent specification; production failure-link machinery is not reused. For each boundary e and pattern ID i, the oracle emits (i,e-|P_i|,e) exactly when |P_i| <= e and the corresponding window equals P_i. Its nested loops order the vector by (e,i), retaining empty and duplicate IDs.

Source-derived partial relation with finite evidence, not universal verification.

## premise-boundary

Type: metamorphic_relation. Domain: Finite deterministic fixtures and test executions; no arbitrary programs or hidden production state.

Tests exercise injective byte relabeling, ID permutation and prefix extension. The noninjective example shows that a collision can create a match; it does not establish that every noninjective mapping does so. Byte-order conditions concern a different algorithm, not this oracle.

Source-derived partial relation with finite evidence, not universal verification.

## mutation-detection

Type: distinguishability_relation. Domain: Finite deterministic fixtures and test executions; no arbitrary programs or hidden production state.

Removing inherited failure outputs must fail the empty/suffix-output boundary test; compilation success and assertion failure, not compilation error, establish this control.

Source-derived partial relation with finite evidence, not universal verification.

## evidence

### Evidence

Four tests: 225 ordered pattern pairs, each with duplicate first pattern, across 127 binary texts (28,575 oracle comparisons); full byte range; failure-output/empty/duplicate boundaries; injective relabel and ID permutation; five compatible predecessors and prefix extension. Compiled production mutant removing inherited outputs fails boundaries_failure_outputs_and_duplicates.

Debug and release each ran twice with identical normalized per-test outcomes (durations excluded). Strict crate Clippy with all targets and denied warnings, owned rustfmt, and four twice-generated identical source-bound syntax projections passed. All 175 crate tests also passed once with temporary module declarations. Mutants restored before hashing; no shared lib or registry edits committed. No timing, allocator-exhaustion or universal proof claim.

## History

### 2026-10-30 · descriptant

2026-10-30. Original std-only code, no copied source. Prospective model preceded code in ../mod.rs.groupant/model.md#prospective. Classical citations appear in implementation documentation; variant complexity is explicit.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

### 2026-11-01 · descriptant

2026-11-01: Reviewed the test oracle's ordered observation. Removed suffix-array material accidentally included in this Aho test model. Equal match multisets do not imply equal vectors: reversing IDs at a shared boundary is rejected. The compiled inherited-output mutant is historical evidence, not an operation performed by the tests themselves.

### 2026-11-01 · evidence

2026-11-01: The four current tests passed in an offline locked debug run. An external Rust probe called the actual API on empty and duplicate patterns and checked an independently expressed ends-with oracle; all 13 ordered records agreed. Reversing IDs within each boundary preserved the multiset but failed ordered equality. This control mutates the observed vector, not the implementation; it does not repeat the historical compiled mutant. Only this fixture and the existing test domains were checked.

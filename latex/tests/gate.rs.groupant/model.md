# latex-test-gate

## descriptant

### latex-test-gate

SUITE parses each fixture, rejects unknowns, checks structural round trip and canonical fixed point. Golden LaTeX/text compare exact strings; unknown spans checked against source slices; malformed fixtures check error kind and position; macro fixtures compare expanded trees and loop rejection.

## approximant

### Forward relation

SUITE parses each fixture, rejects unknowns, checks structural round trip and canonical fixed point. Golden LaTeX/text compare exact strings; unknown spans checked against source slices; malformed fixtures check error kind and position; macro fixtures compare expanded trees and loop rejection.

No useful approximate executable metric for exact categorical syntax; future caps1/2/5/10% only.

## invariant

### Domain, invariant and cost

SUITE parses each fixture, rejects unknowns, checks structural round trip and canonical fixed point. Golden LaTeX/text compare exact strings; unknown spans checked against source slices; malformed fixtures check error kind and position; macro fixtures compare expanded trees and loop rejection.

Only selected synthetic tests executed; document sample and random soup not run. No live VMM/API, corpus, GPU or external effects.

Selected tests linear in total fixture parsing/printing cost; loop witnesses bounded by parser expansion guards. Optional soup30000 has fixed PRNG but skipped here.

## composition

### Composition

SUITE parses each fixture, rejects unknowns, checks structural round trip and canonical fixed point. Golden LaTeX/text compare exact strings; unknown spans checked against source slices; malformed fixtures check error kind and position; macro fixtures compare expanded trees and loop rejection.

Build generates table enums consumed by parser dispatch and printers. Lexer tokens feed macros and parser; parser returns AST whose equality ignores spans. Printers and gate compare that quotient, not original bytes. Shared parent syntax backs scoped stages; stages are byte-bound projections, not independent Rust modules.

## models

## behavior_complement

The harness excludes passing when a chosen fixture violates expected output, but does not exclude all unseen parser/printer faults. Corruption test requires some applicable defect to turn red and explicitly accepts an unaffected fraction fault.

## reverse_reconstruction

A successful test transcript has many compatible implementations; fixtures constrain outputs only at their cases. Failing assertion identifies a violated observation, not a unique bug or original input beyond logged fixture.

## roundtrip_relation

gate check compares parser tree modulo Span equality, not byte source identity.

## golden_outputs

Exact expected canonical strings and Unicode text constrain different projections.

## fault_applicability

At least one configured fault changes each affected fixture; SwapFrac on x+y remains green.

## coverage_boundary

Finite fixtures and explicit skips; no universal TeX correctness inferred.

## tests

### Evidence

7 integration tests via --test gate with document and soup skipped

Selected library filters ast:: lex:: macros:: parse:: print:: table:: and integration gate excluding document_extraction_with_preamble_macros and random_token_soup_never_panics_and_roundtrips. Direct rustc --test latex/build.rs exercises private helpers. All selected tests twice debug/release with identical sorted names. Four compiled mutants at build/AST/parser/printer boundaries fail assertions then exact restoration passes. These do not individually cover every source. Build-main failure effects, allocation failures and universal roundtrip are unproved. Strict Clippy reports existing lints. No corpus/live VMM/API/GPU/full-school/held-out tests.

## History

### 2026-10-31 · descriptant

Epoch2026-10-31: full source reviewed before edits; prospective models retained externally. No optimization.

### 2026-10-31 · tests

Heading `### Evidence — 2026-10-31` stays in tests as `### Evidence`.

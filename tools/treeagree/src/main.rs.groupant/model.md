# treeagree-main

## descriptant

### Descriptant

command dispatch selects agree/check/dispute paths; agree reads reference and lex then three witness files, requires sentence alignment, creates report directory and proposes only token-count-aligned sentences. apply clones candidates, tries individual edit, bundles pending head-path/root edits only after Cycle/Roots errors, then drops failed retag except PosAgree. kept edits sorted by position; output and two TSVs precede read-back diagnostics and agree.md.

## approximant

# Forward approximant

command dispatch selects agree/check/dispute paths; agree reads reference and lex then three witness files, requires sentence alignment, creates report directory and proposes only token-count-aligned sentences. apply clones candidates, tries individual edit, bundles pending head-path/root edits only after Cycle/Roots errors, then drops failed retag except PosAgree. kept edits sorted by position; output and two TSVs precede read-back diagnostics and agree.md.

## Composition

Child fragments model one narrow helper each; their correctness alone does not establish orchestration.

The containing source wires this scoped relation to its callers. Treeagree proposal→gate→commit and treebanks parse→match→collect→render are actual joins; parsecmp is an independent diagnostic path. The external group model distinguishes these tags.

## catalog

# Typed models for treeagree-main

## behavior_complement

A failed no_worse trial cannot update working state; an isolated PosAgree cannot retry without retag. Token-count mismatch skips proposals but still contributes total word denominator.

Operator: inner exclusion on admitted domain. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## reverse_reconstruction

Final heads and reports admit multiple ordered proposals and rejected histories; bundled edits lose trial history. MISC records selected old head/relation, not full previous arbitrary MISC or complete reversible state.

Operator: compatible antecedent relation, retaining ambiguity. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## bundle_trigger

Only Cycle/Roots opens bundle retry; a Head/RootRel error does not, so gate failure order changes admissible bundles.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## retag_retry

Ordinary rule with failing fix retries only current edit without fix while other pending fixes remain; PosAgree forbids this fallback.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## write_prefix

Output conllu, edits TSV, rejected TSV precede report; no transactional rollback follows later failure.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## json_subset

json_field is narrow emitted-format transport, not a JSON parser; unicode escapes are not decoded and array commas inside ids are ambiguous.

Operator: specialized observation relation under the stated fixed inputs. Evidence: source branch derivation plus model.md#tests; unexercised branches remain hypotheses. Oracle: evaluate the scoped actual helper on admissible fixtures and compare this observation. No universal proof.

## tests

### Bounded evidence

- `dispute::tests::answers_parse_and_silence_keeps_the_draft`
- `tests::rules_fire_on_evidence_and_only_then`
- `tests::gates_reject_bad_trees`
- `tests::swap_needs_both_edits`
- `tests::retag_rides_with_the_relation`

These names passed twice in debug and twice in release; the complete treebank/comparison slice has 25 target-qualified names and identical sorted outcomes in all four runs. Ten compiled behavior mutations (one per owned source) failed named runtime assertions; exact original/restored SHA-256 equality and positive reruns were checked. Zero assertion-only mutants, zero surviving exploratory mutants, zero production fixes. Fresh raw logs, numeric exits, commands and mutation hashes are external in the wave33 trees receipt set.

#### Scope and gaps

json_str fragment is source-derived only; no direct JSON roundtrip assertion. Its unicode-control mismatch with json_field is explicitly unsupported. No corpus, network, downloads, annotation model, live teacher, full release build or site execution. The treebanks join is entirely in memory and parsecmp uses a tiny uniquely named temporary directory. Treeagree Registry::load uses the embedded UD registry. Tests are finite CPU observations, not universal semantic proof, title-match quality estimates or corpus accuracy.

## invariant-and-cost

### Invariant and cost

n tokens/e edits: candidate cloning O(n) per trial, shape/punctuation O(n²), bundle lookup O(e*n); conservative O(e*(n²+e*n+registry)). Reporting adds output bytes and repeated kept scans O(n*e). CLI retains all sentences and witnesses in memory; filesystem failures can leave earlier writes.

A failed no_worse trial cannot update working state; an isolated PosAgree cannot retry without retag. Token-count mismatch skips proposals but still contributes total word denominator.

The complement and specialized catalog constrain preserved coordinates and effect boundaries. Empty/malformed domains are only guaranteed where explicitly admitted. No whole-program cost bound follows from a helper bound.

## History

### 2026-10-31 · descriptant

#### Epoch 2026-10-31

Prospective source reading identified conditional input alignment, information loss and failure order before tests. Existing production code is retained; added tests use actual helpers. Finite results update this source-bound model, not all parser/corpus semantics.

No executable percentage surrogate has a justified observation metric for these categorical/effectful APIs. Future profiles require metric, domain and real-reference evidence at caps 1/2/5/10%; no higher cap. Syntax is partial lexical Rust projection without macro expansion or semantic resolution.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.

Source `tools/treeagree/src/main.rs` at SHA-256 `fadc15525d579f8681612a25d06356fed2156f8f7054b54e61ffc07c5a590032`. Selected actual-source tests in `treeagree`:

### Undated · document

Subject: source SHA-256 `fadc15525d579f8681612a25d06356fed2156f8f7054b54e61ffc07c5a590032`; manifest supplies exact source/byte scope. Domain: admitted states in forward model, nonoverflow arithmetic and successful allocations. Observation: one terminating invocation, except explicitly described file trace. Source-derived partial relations; named finite tests constrain selected cases only. Each entry holds arguments and environment fixed except its stated intervention.

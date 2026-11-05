# prag-agree

## model

# prag-agree

## forward

Domain X is tagged Table(text), TreeForm(block), AgreeOne(id,L,W,tree,A) or Run(labels,witness,trees,A); A is whether PRAG_AGREE_ALL equals '1', fixed during the call. Table maps first-column IDs to header/value zip maps: last duplicate header and last duplicate ID win, empty lines omitted but an empty ID on a nonempty row is retained. Run forms L and W maps and a last-block-wins exact sent_id->tree_form map after CRLF normalization. It emits one row per key of L in lexical order. TreeForm applies question before imperative before exclamation before declarative. AgreeOne applies P0 only to recognized original forms with non-character voice, then P1 else P2 else optional P3, then optional P4 and P5; rule predicates and numeric thresholds are those in labels/tree-form children. Missing label fields default empty for changed fields. Report n=|keys(L)|, rule counts sum per-row rule indicators, and move counts sum only form/act changes.

## invariant

Exact ID joins do not borrow witnesses/trees from another key. Witness-only/tree-only keys add no output rows. n counts retained label-map entries, including an empty ID if present, not raw lines. P0 cannot override character voice or an unrecognized original form; P1 consumes the form after P0 and prevents P2/P3. With A=false P3/P4/P5 cannot apply, so polarity/hedge stay supplied defaults. No move entry records polarity/hedge alone; rule totals may exceed n. Question marker takes tree-form precedence over imperative and exclamation.

## cost

For B bytes, N rows/blocks and H columns: table parsing/copying O(B+N log N+NH log H), tree-form bounded row scans O(B+N), CRLF normalization O(B) extra allocation. Run performs fixed rule probes per retained label, map lookups and O(M log M) sorting of M distinct move descriptions, plus output bytes. External file reads/witness inference are excluded.

## composition

Public APIs table, tree_form, agree_one and run compose supplied strings by exact sent_id. Native run->tree_form->agree_one->table joins execute. prag CLI reads three files before run and writes two outputs; CLI filesystem behavior is inspected only.

## epoch-memory

Initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

Documentation audit: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

## checks

cargo test -p prag --lib agree:: -- --test-threads=1; the wave12_models submodule contains new finite checks. File model.md#evidence binds old witnesses, repeated profiles and compiled production mutations. Syntax projections are partial and macro bodies opaque.

## behavior-complement

For the tagged input domain X above and matching output schema Y, let J(x,y) be the concrete forward/invariant constraints above. C_inner={(x,y) in X×Y | not J(x,y)} is a source-derived inner model of absent behavior. In particular exclude candidate outputs that violate: Exact ID joins do not borrow witnesses/trees from another key. Witness-only/tree-only keys add no output rows. n counts retained label-map entries, including an empty ID if present, not raw lines. P0 cannot override character voice or an unrecognized original form; P1 consumes the form after P0 and prevents P2/P3. With A=false P3/P4/P5 cannot apply, so polarity/hedge stay supplied defaults. No move entry records polarity/hedge alone; rule totals may exceed n. Question marker takes tree-form precedence over imperative and exclamation. The native finite checks support selected members/nonmembers; no general Rust correspondence proof or claim about malformed state outside X is made.

## reverse-reconstruction

B(tag,y) retains all tagged raw tables/blocks, parsed maps L,W,T, environment branch A and per-ID rule traces compatible with the child predicates and output y. Run constrains exact shared keys and last-wins maps; Table and AgreeOne observations are separate tags, never silently equated. Duplicate overwritten rows/headers, ignored witness/tree keys, missing-vs-explicit-default fields and unreported witness columns are lost. Report-only projection loses changed IDs; full TSV still cannot recover overwritten inputs or witness probabilities uniquely. A may be ambiguous when optional rules do not fire.

## intervention

Hold the tagged labels, witnesses, trees and ID maps fixed; compare A=false with A=true. P0-P2 retain identical predicates and precedence, while additional P3/P4/P5 branches may change act/polarity/hedge and rule traces. This is a counterfactual policy relation, not a reverse map. Optional branches are source-inspected only; finite executed evidence covers default-mode priority, threshold and voice fixtures.

## evidence

### Native evidence

Six scoped tests (four new wave12_models, two existing) pass twice in debug and twice in release. The old production source with the new tests compiled and failed crlf_sentence_identity_and_report_denominator and incomplete_labels_are_total_and_last_duplicate_wins; five other wave12 tests passed. The repair normalizes CRLF blocks and uses empty missing-field defaults in report accounting. The compiled mutant raising P2 probability from 0.2 to 0.3 fails finite_agreement_thresholds_and_voice_precedence.

Each profile has equal normalized named test identities/results. Both mutants were rejected by a named runtime assertion, not compilation failure. Original bytes were restored exactly and all seven wave12 tests then passed. No live parser, witness model, teacher, DB, corpus or course was run. Existing reader tests use supplied annotation and local compiled lexical dependencies. PRAG_AGREE_ALL was removed from test process environment; optional P3-P5 branches are inspected only.

Syntax generation repeated byte-identically. Full-base external registry scaffold structural validation repeated identically with no diagnostics; the external scaffold parent intentionally has no sidecar. Both changed sources are registered in that scaffold. Root owns final registry/ancestor integration. Structural checks establish bindings and references, not semantic verification.

Only newly added test modules were rustfmt-formatted. Legacy production style remains. Focused 15-test total is not the full prag suite.

## History

### 2026-10-30 · epoch-memory

2026-10-30 initial source-bound epoch. Old-code witnesses exposed CRLF sentence-block merging and missing act/form key panics in report accounting. Normalization repairs block identity and map defaults now agree with agree_one. Last-row-wins duplicate semantics are preserved, not silently relabeled as one observation per raw line. Costs are prospective source models; no optimization, timing gain or percentage-error guarantee is claimed.

Documentation audit, 2026-10-30: root relations now retain tagged API inputs, concrete observation constraints and compatible intermediates. Runtime source, syntax and existing test evidence are unchanged.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Source SHA-256: b50e6b91851fb5a19e9d540b2668f6025524b634522a028614be6676e7b9be46. Syntax SHA-256: 5900da74ba939d1ae6c40aa91e2aac262dcd454b90f06e2590f1948687040da3. Raw logs: $MOVA_DATA/runs/octopus-wave12-invariants-2026-10-30/.

#### Evidence hashes

- old-witness.log: 0f2a352305c080d627b7856fd2fc4c5e4e2b07de14098d660b9952aa7eb2be9f
- checks.log: 1e5808b504258641a740e7469f1be74613e690e71748a2a31f3217850091426e
- restored.log: 55bccc707d523e3f4ab9f4afde315b97c0e04d650269acbba8c3f4fe54792c26
- groupants1.json: 35c49caab79383b7ba65e0f2bdfff85db6dcff8eaa6dc3406b41798a455389c5
- groupants2.json: 35c49caab79383b7ba65e0f2bdfff85db6dcff8eaa6dc3406b41798a455389c5
- changed.json: 46789fd6edb9c2b279d19a59b1f012a0fb5cc5d6422e84bc333669b0a07d9a76
- debug-agree-1.log: e6c4501adb9a41601101395b97b21495357f39e4ac74ae1b18ed8b3e35312ef2
- debug-agree-2.log: 9a36e14d1c8af3f51a9f5336e2325ba9b03f011bfb92ab72805e9065c7a4320a
- release-agree-1.log: 0b3b74423bbdacf5d004a799ccfb4d80ee2d9384d6c7c05a4696282f4c234ba6
- release-agree-2.log: 2bd5c5fe262fc27f552d326a705485b00e8235a10dd777077eb9be2ab6be687c
- mutant-agree.log: 89c16abbac4d60211c606f970eeae824f60b2e9d47a48b53f0423962a09d2de7

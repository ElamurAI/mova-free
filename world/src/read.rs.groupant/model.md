# world-read

## model

# Reading segmentation, supplied-tree projection and orchestration

Scope: partial source-bound relations over the observations named below. Sufficient memory and terminating dependencies assumed. This is not a universal proof or an executable percentage-error approximant.

## forward

split_book selects marker-bounded text, recognizes headings, normalizes paragraph whitespace, admits stories at pre-filter byte total >=200, then removes service/short paragraphs and excluded titles. skeleton projects supplied Tree plus position-aligned original forms into ordered clauses. summarize obtains annotator, strips double-quoted spans, parses and deduplicates strings. read_book serializes paragraphs; check rates saved summaries separately. Detailed retrieval/training semantics in the remainder of this large source remain unmodeled.

## invariant

Stable story/paragraph order; retained paragraphs have >=4 words and do not start '['. Admission precedes filtering, so retained total can be <200. Clause indices are sorted; syntax does not prove semantic faithfulness.

## cost

L text bytes: O(L) segmentation storage/scans. n tree nodes, v selected clauses: O(n*v) membership checks plus phrase child sorting/output bytes. Summary dedup can be quadratic in distinct strings. Parser/teacher latency unknown; no throughput measurement.

## composition

split_book -> ordered paragraphs -> summarize -> JSONL. Original forms and Tree indices must share tokenization. CLI routes read_book/check separately; check consumes text/summary/title/par. Native checks exercise only pure supplied data, not parser or teacher.

## behavior-complement

Type: behavior_complement. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Universe: pure supplied text/tree/output triples with valid indices and aligned originals. Inner absent relation excludes service paragraphs in split_book output and nonempty skeleton of an empty tree. SAY rendering excludes its direct object, but independently selected clauses may survive. Unmodeled paths are unknown, not absent.

## reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Retain every input whose scoped projection equals the observation. Whitespace/header variants share book fibers; varying speech objects shares a skeleton fiber. Neither original text nor tree is uniquely recoverable. Finite fixtures enumerate whitespace layouts and three speech objects.

## admission-order

Type: constraint_relation. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Admission observes normalized byte totals before filtering. A long illustration can admit a four-word retained story. Moving the threshold defines a different system. Native oracle covers n=180..220.

## speech-loss

Type: information_loss. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Fix speech root, subject and original verb; vary Obj among secret/story/answer. All render Alice said. This bounded fiber witnesses lost object information.

## subject-carry

Type: state_transition. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. last_subj starts empty per skeleton call. A selected nonempty subject updates it; missing subjects inherit it in token-index order. No cross-call coreference state.

## question-priority

Type: priority_relation. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. qtype lowercases raw text; substring feel overrides first-word classes; what/happen precedes what/do. Punctuation stays on first token: who? maps Other. Native table checks these boundaries.

## check-boundary

Type: resource_boundary. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. For batch>0, invalid JSON rows are discarded before take(limit), each chunk asks once, call trace appends before processing scores, and final check.jsonl writes after all chunks. Duplicated indices count repeatedly; f64 scores have no 0..2 clamp. batch=0 panics. Source-inspected only.

## summary-coupling

Type: composition_relation. Subject: world-read. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. read_book creates directory before input read; limit counts stories. Successful rows preserve book/story/title/par/text paired with the summary of that paragraph. A pre-write failure may leave directory/external work; no atomic transaction is promised.

## checks

Native synthetic tests: cargo test -p world --lib read::wave9_models -- --test-threads=1. Test bodies define finite domains and fixtures. File-level model.md#evidence records repeated debug/release checks and a compiled negative control. No parser acquisition, teacher request, dictionary write or external assessment executed. Orchestration boundaries are source inspection only.

## epoch-memory

First scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

## wave34-review

## evidence

### Native evidence

Segmentation covers 41 byte lengths, four whitespace/line-ending variants, service/heading fixtures; skeleton covers eight boolean role combinations, three speech-object preimages, one coordinated-subject fixture, empty tree, and nine question cases. Other retrieval/extraction/training paths are explicitly unmodeled. No live parser, check teacher or I/O orchestration run.

Pure helper extraction: operation_record isolates existing op_in cue projection without changing its logic. read.rs production behavior unchanged. No runtime bug repair or measured performance gain claimed.

## wave34-evidence

### Wave34 helper evidence

Fifteen exact world library helper tests passed twice debug and twice release; sorted target-qualified numeric-status lists agree. Eight belong to read and seven to ans. New fixtures cover centered heading predicates, direct modifiers/original-form holes, modal/carry reset, participant rescue at exactly four, and ASCII-space numeric fibers. Existing independent 961-pair subsequence oracle remains selected. Three compiled production mutants were rejected by named runtime assertions: heading indentation, phrase determiner selection and event rescue total. Each restored backup hash matches exactly and its named test passed after restoration. No compilation rejection is counted as behavior evidence.

Syntax was generated twice after formatting additions and compared byte-for-byte; descendants share this one projection. Five new child groupants here retain actual partial AST/range bindings, six typed views each and explicit costs. Full source and historical claims reviewed before edits. No runtime bug fix, semantic task gain, performance gain or percentage approximation is claimed. Parser acquisition, model loading, teacher/corpus/frozen assessment, training and full answer pipeline remain unexecuted.

## History

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Universe: pure supplied text/tree/output triples with valid indices and aligned originals. Inner absent relation excludes service paragraphs in split_book output and nonempty skeleton of an empty tree. SAY rendering excludes its direct object, but independently selected clauses may survive. Unmodeled paths are unknown, not absent.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Retain every input whose scoped projection equals the observation. Whitespace/header variants share book fibers; varying speech objects shares a skeleton fiber. Neither original text nor tree is uniquely recoverable. Finite fixtures enumerate whitespace layouts and three speech objects.

### 2026-10-30 · admission-order

Type: constraint_relation. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Admission observes normalized byte totals before filtering. A long illustration can admit a four-word retained story. Moving the threshold defines a different system. Native oracle covers n=180..220.

### 2026-10-30 · speech-loss

Type: information_loss. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. Fix speech root, subject and original verb; vary Obj among secret/story/answer. All render Alice said. This bounded fiber witnesses lost object information.

### 2026-10-30 · subject-carry

Type: state_transition. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. last_subj starts empty per skeleton call. A selected nonempty subject updates it; missing subjects inherit it in token-index order. No cross-call coreference state.

### 2026-10-30 · question-priority

Type: priority_relation. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. qtype lowercases raw text; substring feel overrides first-word classes; what/happen precedes what/do. Punctuation stays on first token: who? maps Other. Native table checks these boundaries.

### 2026-10-30 · check-boundary

Type: resource_boundary. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. For batch>0, invalid JSON rows are discarded before take(limit), each chunk asks once, call trace appends before processing scores, and final check.jsonl writes after all chunks. Duplicated indices count repeatedly; f64 scores have no 0..2 clamp. batch=0 panics. Source-inspected only.

### 2026-10-30 · summary-coupling

Type: composition_relation. Subject: world-read, 2026-10-30. Observation: one completed transition in the stated domain; effects included only where named. Status: partial source model. read_book creates directory before input read; limit counts stories. Successful rows preserve book/story/title/par/text paired with the summary of that paragraph. A pre-write failure may leave directory/external work; no atomic transaction is promised.

### 2026-10-30 · epoch-memory

2026-10-30: first scoped model. Retain ambiguous fibers, source-order rules and distinction between lexical evidence and truth. Tests observe actual helpers; they do not substitute surrogates into production. Macro token bodies in partial syntax remain opaque. Future updates must rebind exact source bytes and retain these limits. No performance or task-quality improvement inferred.

### 2026-10-30 · evidence

Heading `### Native evidence, 2026-10-30` stays in evidence as `### Native evidence`.

Final source SHA-256: 10e0011630db42b9b07f1ce73b584784470e98782e18d097b688aa841c12466f.

Five tests in read::wave9_models; ten tests combined under cargo test -p world --lib wave9_models -- --test-threads=1. Twice debug and twice release: ten passed each. Restored debug pass: ten passed. All five normalized test-name/result lists identical (SHA-256 25f5e8fd801f3ec612171457cc6fdc27d27133def2a77c9c562aeee13ec5f8da). Existing compiler warnings retained; no full suite claim.

Compiled production control: changed admission >=200 to >200; boundary n=200 rejected. Named assertion failure: read::wave9_models::segmentation_threshold_and_service_filter. This was a runtime assertion failure after compilation, not compiler rejection. Source restored byte-for-byte and all ten positives passed afterward. Mutant source SHA-256 74264c452e9abf47ac29e2dc327e555b130b3c90e32f3a4483b4699f83993253; raw mutant log SHA-256 3db4df583ea7f94f1bfdc7e4f4b0e62b7759131ef42768650e032fc9159f65bf.

Syntax generated twice identically; SHA-256 0050507384216c70f8a58da4db9f171bebb0bb96bdc74c3cb83c20045ee421c3. Isolated copies of this exact source/sidecar pass groupants with no diagnostics. Full registry/ancestor integration is performed by the coordinating agent; a minimal registry against the full checkout deliberately leaves unrelated ownership diagnostics and is not counted as a passing full inventory.

Raw evidence lives outside repository under $MOVA_DATA/runs/octopus-wave9-reading-2026-10-30/.

- debug1.log: SHA-256 88068111cf5c531d673ba210a7521a7ebc4ff293962d0db82eb07204a60446ec

- debug2.log: SHA-256 16bfc7d8b0e1387047b6b752b15c726997e79883ae321a9e75ca5738810d74ed

- release1.log: SHA-256 f65b35a2385c348394e235f21c3e5f78485e4edbf1a395a3e106462b5ea476d2

- release2.log: SHA-256 c0bfe116e049f834ee7cb89286d9e8fa8e1d9c38f183198dc1ccfbd2958e358e

- restored.log: SHA-256 c154edb51986d3281a2282ed9989b42c9884f513d2015fec2cbd49c692c3d46d

### 2026-10-31 · wave34-review

2026-10-31: full source read; production bytes retained and cfg(test) coverage appended. Earlier evidence applies to its recorded hash only. Current source, syntax and selected helper receipts are in model.md#wave34-evidence. No training, annotation model, teacher, live corpus or full intent pipeline was executed.

### 2026-10-31 · wave34-evidence

Heading `### Wave34 helper evidence, 2026-10-31` stays in wave34-evidence as `### Wave34 helper evidence`.

Source SHA-256: 80ef1f69d776dcb5cd03bf925cc5c01e827d7919495ccda17d785eb3de459857. Production and pre-existing tests are byte-identical to base; only an appended cfg(test) module changes source. Earlier model.md#evidence remains a historical wave8/wave9 receipt, not a current-source claim.

Raw logs, command definitions, numeric statuses and source/test/mutation maps live under external run label octopus-wave34-reasoning-2026-10-31. Native log hashes are cataloged in evidence-summary.json. Current syntax SHA-256: 92034819457fbf6020e0f2f0c9f0abaaad635717bf1d71c1a27205bf2c3c304e.

# Scenario-tail partition and finite recognizer

Subject: `world/src/knowledge.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

Trim end; remove final Unicode scalar from the search window; find the last ASCII sentence separator in that window; split inclusive scenario and following tail. Reject scenario with <3 whitespace words. Lowercase tail, strip terminal ?!. and trim, then match the explicit Cause/Next/Claim phrases; otherwise None.

## invariant

Successful scenario is a borrowed trimmed prefix ending at the selected separator. Tail vocabulary is finite; recognized class does not prove natural-language meaning.

## cost

O(B) trimming, reverse delimiter search, word count and tail lowercase allocation; O(tail bytes) auxiliary storage.

## behavior-complement

Type: behavior_complement. Over UTF-8 question/Option pairs, exclude Some for a selected scenario under three whitespace tokens or normalized tail outside the explicit phrase sets. Exclude changing scenario text or choosing a separator earlier than the last in the search window.

## reverse-reconstruction

Type: reverse_reconstruction. For Some(s,k), compatible questions preserve the trimmed selected prefix s and append whitespace plus any case/terminal-punctuation variant of a phrase in k, provided the last-break algorithm still selects that boundary. Extra punctuation can change the selected boundary; not every typographic variant is compatible.

## unicode-boundary

Type: slice_safety. Final scalar removal uses char_indices, and ASCII delimiters produce valid UTF-8 boundaries. Empty strings have empty windows and no match. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## word-threshold

Type: admission_boundary. The scenario threshold counts split_whitespace tokens, not lexical words; exactly three suffice. No semantic length estimator is involved. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## punctuation-repartition

Type: nonmonotone_normalization. Appending '? !' style repeated sentence punctuation may place a later separator inside the tail; the algorithm can reject a previously recognized question before phrase matching. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## answer-coupling

Type: dispatch_contract. answer uses recognition only when student-tail is enabled, reads the scenario as nonquestion, and maps Cause/Next to the last root, Claim to Ask::Frame with no gap. Recognition alone invokes no parser. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

Runtime answer calls this before parser dispatch. A recognized tail selects scenario reading only if its feature flag is enabled; otherwise ordinary question reading applies. Recognition tests do not establish answer behavior or parser correctness.

## checks

`knowledge::wave34_tests::tail_recognition_keeps_scenario_and_rejects_short_or_unknown` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

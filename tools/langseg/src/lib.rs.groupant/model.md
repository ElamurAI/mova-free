# langseg-engine

## descriptant

### Descriptant

#### Observed epoch

Bounded real helper checks in model.md#tests constrain these models; syntax is a partial lexical projection with no macro expansion, name resolution or proof. Production expressions unchanged; five inline test modules added. Compiled mutants are temporary and exactly restored. Existing malformed-input and nonfinite domains remain explicit, not silently repaired. Parent composes this scope with siblings without adding overlapping costs twice.

No exhaustive nonfinite/overflow audit, corpus accuracy, surrogate percentage or throughput claim.

## approximant

### Forward relation

tokenize produces UTF-8 script/italic spans; runs coalesces scripts and translit_mask propagates strong signs over same italic id. Features PCAMW feed per-group smoothed counts. classify sums Hebrew scores. segment Viterbi uses quarter switch across sentence punctuation, coalesces path columns and aggregates explanation log odds.

Domain: HEB fixes8 columns; argmax lowest-index tie, Viterbi stay wins equality. Empty classification Hbo, neutral-only segmentation empty. Columns<K, representable u32 counts, positive finite alpha, finite nonnegative cost define mathematical scoring domain. Save rounds to4 decimals; load is permissive.

## invariant

### Invariant and cost

#### Invariant

HEB fixes8 columns; argmax lowest-index tie, Viterbi stay wins equality. Empty classification Hbo, neutral-only segmentation empty. Columns<K, representable u32 counts, positive finite alpha, finite nonnegative cost define mathematical scoring domain. Save rounds to4 decimals; load is permissive.

#### Cost

B bytes,n tokens,F features,V vocabulary,K=8: tokenizer O(B²) worst-case on repeated unmatched '<' because suffix.find scans, ordinary bounded markup O(B). Feature lookup O(F log V), model O(VK), Viterbi O(nK) time/space, explanation aggregation/sort O(F log F). No timing claim.

#### Limits

No exhaustive nonfinite/overflow audit, corpus accuracy, surrogate percentage or throughput claim. No percentage surrogate is implemented: categorical parsing/effect observations lack an admitted numerical error metric. Any future profile must define its metric/domain and use only1/2/5/10 percent caps.

## composition

### Local composition

tokenize produces UTF-8 script/italic spans; runs coalesces scripts and translit_mask propagates strong signs over same italic id. Features PCAMW feed per-group smoothed counts. classify sums Hebrew scores. segment Viterbi uses quarter switch across sentence punctuation, coalesces path columns and aggregates explanation log odds.

Children select narrow source functions with their exact byte intervals. Remaining branches stay in the file model; fragments are not exhaustive partitions. Calls and state joins follow the forward relation, not folder order. Shared scans/helpers are counted once in whole-file costs. Complement of composition is not union of complements; reverse composition retains compatible intermediates and caller premises.

## models

# Typed source-bound relations

Subject: exact source SHA-256 in manifest. Domain and resource premises are in model.md#invariant. Each model is partial, source-derived, over a single helper invocation or the described finite trace. Complements are proposed inner exclusions under those premises; reverse relations retain all compatible antecedents, with no unique inverse claim. The oracle is the named checks in model.md#tests; unexecuted cases remain hypotheses. No numerical approximation certificate.

## behavior_complement

No neutral-only token or out-of-range path column appears for admitted finite input. Missing model features contribute0; tied argmax cannot choose higher equal index. Supplied-model APIs consult no teacher/corpus.

## reverse_reconstruction

Labels admit score vectors in argmax cells and feature multisets yielding them. Segments admit all score/path states with same optimum/ties; dropped neutral ends and stripped points unrecoverable. TSV recovers rounded values, not original floats.

## byte_coordinates

Token/Segment offsets are UTF-8 byte boundaries; segment spans first token start through last token end, includes internal gaps but may omit leading/trailing neutral text.

## group_smoothing

logp_fc=ln((n_fc+alpha)/(N_gc+alpha*V_g)) cast f32. W/M threshold uses total counts, others escape pruning. Arithmetic overflow/nonpositive alpha excluded.

## switch_objective

For cost>=0, recurrence max(stay,best-cost) optimizes summed scores minus switches. Empty returns[]; cost length>=n required. Negative penalty not covered by shortcut objective.

## rounded_model_fiber

save4 decimals; load8 parseable scores, ignores extra fields, last duplicate feature wins, header marker unvalidated. Fixture roundtrip measures only bounded score drift.

## tests

### Checks and evidence

Named source/helper checks: wave31::empty_ties_and_switch_objective, script_runs_and_transliteration, classifier_learns_and_shuffled_labels_fail, viterbi_cuts_a_planted_sentence_and_only_with_a_finite_switch, deterministic_and_model_round_trip.

No exhaustive nonfinite/overflow audit, corpus accuracy, surrogate percentage or throughput claim.

## History

### 2026-10-31 · descriptant

#### Prospective epoch

2026-10-31 source review predicted: tokenize produces UTF-8 script/italic spans; runs coalesces scripts and translit_mask propagates strong signs over same italic id. Features PCAMW feed per-group smoothed counts. classify sums Hebrew scores. segment Viterbi uses quarter switch across sentence punctuation, coalesces path columns and aggregates explanation log odds. Resource model: B bytes,n tokens,F features,V vocabulary,K=8: tokenizer O(B²) worst-case on repeated unmatched '<' because suffix.find scans, ordinary bounded markup O(B). Feature lookup O(F log V), model O(VK), Viterbi O(nK) time/space, explanation aggregation/sort O(F log F). No timing claim. External prospective.md predates test additions and preserves APIs, effect order, hazards and interaction hypotheses.

### 2026-10-31 · tests

Wave31 on2026-10-31:30 distinct exact tests passed twice debug and twice release, with equal sorted named outcomes. This node claims only its listed subset; empty subset means source review only. Six actual source-behavior mutants compiled, failed assertions, restored exact bytes/hash, and passed focused green reruns. No assertion-only mutants; existing swapped-input controls are separate. Receipt bundle: $MOVA_DATA/runs/octopus-wave31-corpus-2026-10-31 (native-commands.json, source-test-map.json, mutation-results.json, *.status/*.log). No live corpus, teacher, frozen assessment, download or production data used.

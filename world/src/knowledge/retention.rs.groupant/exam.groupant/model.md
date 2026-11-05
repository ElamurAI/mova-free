# Question-only retained examination

## forward

Exam gates course equality, empty explain/facts/exercises, and at least one test/exam question. Then fingerprint-before, preparse question/option texts, traverse tests then exams, shuffle options, answer and score, verify fingerprint-after, and return rows/counts/identities/time. This is a conditional model assuming parser and answer dependencies return valid results.

## behavior-complement

Type: behavior_complement. Domain and observation are the forward relation above unless narrowed here. U is retained-state/lesson/outcome transitions for one exam call. R is the ordered gate/evaluation relation. An inner bound U minus R contains successful evaluation with supplied teaching, mismatched course, no questions, or changed persisted fingerprint. This excludes those successes by source gates; it does not establish broader absence of transient cache mutation.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation are the forward relation above unless narrowed here. For report r, compatible antecedents satisfy its course/question sequence, shuffle-dependent rows, answer results and state/assessment digests. Hashes and aggregate right counts do not uniquely recover graph, learned weights or questions. Error-before-parse fibers include teaching-bearing lessons and do not witness a successful cold exam.

## presentation

Type: ordering_relation. Domain and observation are the forward relation above unless narrowed here. Rows concatenate tests then exams in vector order. Per-item seed is ((l.n as u64)<<20 | (i as u64)<<1 | 1) xor extra, with extra=0 for tests and 0xE8A3 for exams. Index i restarts per kind. Gold is included among options and shuffle inputs; labels are used for scoring after answer, so this is not an invariance claim under changing gold/options.

## diagnosis

Type: decision_partition. Domain and observation are the forward relation above unless narrowed here. Diagnosis precedence is correct; else count present; else no best support for gold option; else not understood; else supported selection error. These are ordered predicates: count presence masks missing support. Counts summarize answer evidence and do not prove the causal source of a mistake. A truth-table oracle remains proposed.

## state-boundary

Type: observation_noninterference. Domain and observation are the forward relation above unless narrowed here. The before/after equality checks only persisted payload fingerprints. preparse may fill transient caches. new/load/train neutralize focus, but student is public and exam does not force it false; returned lesson_focus=false is a literal report field, not a measurement of arbitrary externally mutated student focus. Successful neutral-focus claims require that entry precondition.

## invariant

The gate prefix is cheap; successful cost adds parser work and answer/why for each item plus two payload hashes. No runtime benchmark, live teacher, GPU or full-school claim follows. Seconds is observational timing and prevents unconditional byte-identical report claims.

## checks

exam_rejects_teaching_and_does_not_change_state checks early rejection and unchanged fingerprint only. assessment identity tests cover selected scored-field mutations. A successful exam, fresh-process CLI retention, answer quality and public-focus mutation have not been executed in this epoch.

Shared evidence is in ../model.md#tests. Referenced older mutation controls are historical evidence, not rerun here. General semantic correspondence and unsampled behavior remain unknown.

## epoch-memory

Decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

## History

### 2026-10-29 · epoch-memory

2026-10-29: decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

### Undated · document

Subject: retention.rs, SHA-256 89715719216c1a94f9e701e24d4efea0b962bf04f5178d4916d19dc54536fb34. The manifest records the exact source interval. These source-derived relations are partial specifications; their evidence is bounded.

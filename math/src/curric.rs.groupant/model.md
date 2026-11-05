# math-curriculum

## descriptant

### Source-bound description

sample stable-sorts splitmix(SEED xor fnv(salt:id)) then takes k. prep assigns floor(n/2) dev before deduplicating ids (reported counts may exceed rows). grade recognized option overrides answer; else letter or unique numeric match. Int rational equality exact; otherwise abs error<=1%*max(abs(gold),1e-9) OR<1e-9.

## approximant

### Forward relation

sample stable-sorts splitmix(SEED xor fnv(salt:id)) then takes k. prep assigns floor(n/2) dev before deduplicating ids (reported counts may exceed rows). grade recognized option overrides answer; else letter or unique numeric match. Int rational equality exact; otherwise abs error<=1%*max(abs(gold),1e-9) OR<1e-9.

Domain: Synthetic samples/Problems only. MMLU indexing assumes<=5 choices and valid index; malformed input can panic. No external prep.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

Recognized wrong option excludes correct grade even with right numeric answer; unknown option permits fallback. k=0 returns empty. NaN does not pass tolerance.

Ordering: Recognized option→short letter→numeric unique match; norm_num tries rational then first number then expression.

Cost: O(n log n)+clone/hash bytes sampling, complete JSONL pools for prep. Grading parsing/evaluation and big arithmetic delegated.

## composition

### Composition and epoch memory

Problem kind/source→solve Expect; option_value and norm_num differ, so not identical normalization.

No corpus prep/load or held grading; arbitrary malformed numeric normalization not certified.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

Recognized wrong option excludes correct grade even with right numeric answer; unknown option permits fallback. k=0 returns empty. NaN does not pass tolerance.

## reverse_reconstruction

Sample admits all pools with same keyed prefix and unseen tail; collisions retain input order. Grade true admits numeric tolerance neighborhood or alternate spellings, not unique answer.

## failure_precedence

Recognized option→short letter→numeric unique match; norm_num tries rational then first number then expression.

## representation_transport

Problem kind/source→solve Expect; option_value and norm_num differ, so not identical normalization.

## resource_boundary

O(n log n)+clone/hash bytes sampling, complete JSONL pools for prep. Grading parsing/evaluation and big arithmetic delegated.

## domain_boundary

Synthetic samples/Problems only. MMLU indexing assumes<=5 choices and valid index; malformed input can panic. No external prep.

## tests

### Evidence

Selected actual-source checks: `cargo test -p math --lib curric::`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

No corpus prep/load or held grading; arbitrary malformed numeric normalization not certified.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. No corpus prep/load or held grading; arbitrary malformed numeric normalization not certified.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.

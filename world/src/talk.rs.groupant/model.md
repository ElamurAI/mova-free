# world-talk

## descriptant

### world-talk

act_of assigns trimmed text, affection and polarity with negative precedence; act priority greet/SOM, thank, apology, terminal ?, affection, self-correction, evaluation, request, assertion. indirect_request strips a modal prefix, normalizes action wrappers/roll back, then capability prefix and blocked moving-action checks. Memory records JSONL; recall score=importance/10+0.995^(age_seconds/3600)+substring overlap fraction.

## approximant

act_of assigns trimmed text, affection and polarity with negative precedence; act priority greet/SOM, thank, apology, terminal ?, affection, self-correction, evaluation, request, assertion. indirect_request strips a modal prefix, normalizes action wrappers/roll back, then capability prefix and blocked moving-action checks. Memory records JSONL; recall score=importance/10+0.995^(age_seconds/3600)+substring overlap fraction.

Exact categorical/effect semantics: no useful executable percent surrogate supplied. Future admissible caps 1/2/5/10% require defined metric/domain; no error bound claimed.

## invariant

Empty act is Assert. Greeting beats ?. Zero importance is retained and upper-clamped to 10; write failures are ignored. Recall deduplicates only adjacent formatted equal strings. Civil time uses Euclidean day/remainder and Monday weekday zero.

Cost: Cue scans O(B*C), sentence splitting O(B), memory load O(M bytes); recall O(M log M+M*Q*B), reflection O(M). Time arithmetic assumed representable; no universal overflow proof.

## composition

Greeting and apology can win over question punctuation; capability matches prefixes, not exact token commands.

lib exposes modules; main activates state and dispatches. ideas uses nerves::norm and talk::now, store::write_atomic; talk uses babi splitting/prag Act. VMM uses lang::parse_all and world executor summaries before teacher continuation. These adapters preserve different coordinates (line number versus sentence anchor); splice acceptance does not establish execution validity. Pure act/capability/calendar/politeness/cue tests; memory/clock-dependent effects and social writes unexecuted.

## models

## behavior_complement

Greeting/SOM, thanking and apology can exclude Ask despite terminal ?. Negative polarity excludes positive polarity even when praise is present. indirect_request rejects unsupported capability prefixes and blocked moving actions, but does not enforce a general exact-token command grammar. remember may silently fail; returning a social acknowledgement is not proof of durable persistence.

## reverse_reconstruction

Act/polarity admits all strings matching the same first successful cue branch; it cannot recover original utterance. Recall output admits unseen lower-ranked records and alternative timestamps with same formatted age; stored memories do not imply successful fsync.

## dialogue_priority

Greeting and apology can win over question punctuation; capability matches prefixes, not exact token commands.

## failure_precedence

Empty act is Assert. Greeting beats ?. Zero importance is retained and upper-clamped to 10; write failures are ignored. Recall deduplicates only adjacent formatted equal strings. Civil time uses Euclidean day/remainder and Monday weekday zero.

## resource_domain

Cue scans O(B*C), sentence splitting O(B), memory load O(M bytes); recall O(M log M+M*Q*B), reflection O(M). Time arithmetic assumed representable; no universal overflow proof.

## effect_boundary

Pure act/capability/calendar/politeness/cue tests; memory/clock-dependent effects and social writes unexecuted.


## tests

### Bounded evidence

Pure dialogue/calendar; memory writes excluded

Names: ["talk::wave28_dialogue::act_precedence_and_indirect_gate","talk::wave28_dialogue::civil_negative_seconds_and_politeness"]

Command: CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR=ISOLATED_TARGET MOVA_HOT_ROOT=FRESH_TEMP_DIRECTORY cargo test -p world --lib --bin world [--release] wave28_dialogue -- --test-threads=1

The fresh temporary directory basename must start with wave28-dialogue-. Never point this fixture at a real hot root. The binary test calls private helpers; it never invokes main or state activation.

Nine selected cases passed twice debug and twice release with identical sorted named outcomes. Only the named source behaviors are exercised.

Control: mutant-talk: question priority. All four source mutants compiled and failed with exit 101; each was restored byte-for-byte and nine positive cases passed again. lib and main have no runtime mutant claim.

Six parent syntax projections were independently generated twice and compared. Child fragments reference parent syntax with exact byte ranges and hashes. Strict whole-world Clippy remains subject to existing warnings in untouched code. No live teacher, corpus, DB, model, GPU or school execution. No semantic proof or executable percent surrogate follows.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-derived partial model; prospective hypotheses retained in external receipt. Pure act/capability/calendar/politeness/cue tests; memory/clock-dependent effects and social writes unexecuted.

### 2026-10-31 · tests

Heading `### Bounded evidence — 2026-10-31` stays in tests as `### Bounded evidence`.

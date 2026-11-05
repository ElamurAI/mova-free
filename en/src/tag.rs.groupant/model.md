# en-tag

## descriptant

### en-tag
TnT trigram tables, lexical emissions, suffix model, Viterbi

## approximant

### Forward relation
Tagged sentences only; double BOS then final BOS. Interpolation ties trigram then bigram; rare<=10 suffix max10. Exact then lowercase known lexicon bypasses hints. Unknown suffix stops at first absent; dictionary soft/hard then seen positive emission candidates.

No executable Rust surrogate attached: no demonstrated metric for preserving these categorical, stateful or effect observations. Supported future caps are 1/2/5/10%, not a claim of current error.

## invariant

### Domain, invariant and cost
Tagged sentences only; double BOS then final BOS. Interpolation ties trigram then bigram; rare<=10 suffix max10. Exact then lowercase known lexicon bypasses hints. Unknown suffix stops at first absent; dictionary soft/hard then seen positive emission candidates.
Empty tagging empty; empty training has NaNs/no support and nonempty inference may panic. Deserialize and Soft multiplier have no validation.
Train O(T+NT^3+suffix work); decode O(n*K^3) time/O(n*K^2), NT fixed; unchecked counters.
Inputs must satisfy the indexing/generated-data assumptions stated by the typed catalog; allocation failure and overflow beyond bounded fixtures remain untested.

## composition

### Mathematical combiner
Source bytes and twice-generated syntax constrain this model. Forward relation: Tagged sentences only; double BOS then final BOS. Interpolation ties trigram then bigram; rare<=10 suffix max10. Exact then lowercase known lexicon bypasses hints. Unknown suffix stops at first absent; dictionary soft/hard then seen positive emission candidates.
Reverse relation: A sequence has all words/parameters for which its score maximizes under strict-greater first-candidate ties; aggregated training counts lose original sentence ordering.
The ordered typed family records independent exclusions, compatible antecedents and source-specific boundaries. Fragment ranges compose through actual control flow; they do not cover all helpers.
Epoch memory: empty learned support, lossy projections and non-atomic effects cannot inherit stronger downstream guarantees.

## models

## behavior_complement
Empty tagging empty; empty training has NaNs/no support and nonempty inference may panic. Deserialize and Soft multiplier have no validation.

## reverse_reconstruction
A sequence has all words/parameters for which its score maximizes under strict-greater first-candidate ties; aggregated training counts lose original sentence ordering.

## interpolation_boundary
Deleted interpolation chooses q3 on ties then q2; no counts makes lambda division0/0 and NaN transition cells.

## suffix_backoff
Suffix smoothing (q+theta*p)/(1+theta) stops at first missing suffix, cap flag distinguishes uppercase first scalar.

## emission_precedence
Known exact/lowercase lexical entries return before dictionary; hints only modify unknown probabilities; hard cannot add unseen classes.

## viterbi_ties
Viterbi sums transition+emission and final BOS; strict > retains first finite maximizing predecessor; candidate emptiness is not validated.


## tests

### Actual source checks
Named check: tag::wave27_tests::tiny_known_emissions_and_empty_boundary. Command: cargo test -p en --lib wave27_tests -- --test-threads=1 (also --release). Four runs: each 10 passed, 0 failed, numeric exit0; sorted named outcomes equal. This file contributes one named check per run.
Control present for this source: false. Hash zeroed byte contribution, text changed empty chrF to1, lexgen removed three-cell PastPart copy: each compiled and failed one assertion (exit101), then exact source restoration and ten-test positive rerun (exit0). Other sources have no compiled mutation claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: read complete source before prospective model and bounded tests. Production code retained byte-for-byte; only inline synthetic checks added. Coverage is partial, not a proof.

### 2026-10-31 · tests

See evidence.json and external octopus-wave27-lexical-2026-10-31 receipts. Static built dictionary read is part of the linked library; no corpus or external services used. Unknown support, malformed model/blob, resource failures, cycles and exhaustive Unicode behavior are unexecuted gaps.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): the unknown-word dictionary hint allows the derivation tags (derive::hint) of a lower-case form the dictionary does not know. Source e2175f4ca53b; earlier receipts and evidence refer to the previous source hash and are kept as history.

2026-11-04 epoch (lexicon growth): the unknown-word dictionary hint takes lexext::hint(word, capitalised) first, then the derivation's tags for a lower-case word.

### 2026-11-04 · approximant

2026-11-04: unknown(w): allowed tags = dictionary analyses ∪ (no dictionary form ∧ not capitalised ? derive::hint : ∅).

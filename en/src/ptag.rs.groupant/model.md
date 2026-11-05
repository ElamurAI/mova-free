# en-ptag

## descriptant

### en-ptag
Averaged perceptron with frequent dictionary, TnT prior and beam

## approximant

### Forward relation
Context length asserted before filtering; known freq>=20/share>=.97. Update clock excludes known; wrong saturates u8; robust env changes updates. Forced before known; beam zero clamps1; stable score sorting then last-pair dedup; jackknife k0 clones.

No executable Rust surrogate attached: no demonstrated metric for preserving these categorical, stateful or effect observations. Supported future caps are 1/2/5/10%, not a claim of current error.

## invariant

### Domain, invariant and cost
Context length asserted before filtering; known freq>=20/share>=.97. Update clock excludes known; wrong saturates u8; robust env changes updates. Forced before known; beam zero clamps1; stable score sorting then last-pair dedup; jackknife k0 clones.
Finite beam not global optimum; empty words empty; no supported candidates can panic at history lookup; epochs0 may mark every token suspect.
Inference O(n*b*(F+NT log NT)+n*b^2), histories O(n*b^2); training epochs*tokens*features; u32 clock unchecked.
Inputs must satisfy the indexing/generated-data assumptions stated by the typed catalog; allocation failure and overflow beyond bounded fixtures remain untested.

## composition

### Mathematical combiner
Source bytes and twice-generated syntax constrain this model. Forward relation: Context length asserted before filtering; known freq>=20/share>=.97. Update clock excludes known; wrong saturates u8; robust env changes updates. Forced before known; beam zero clamps1; stable score sorting then last-pair dedup; jackknife k0 clones.
Reverse relation: Many weights/training sets/hash collisions share outputs; forced tags constrain outputs without identifying weights; averaging discards update history.
The ordered typed family records independent exclusions, compatible antecedents and source-specific boundaries. Fragment ranges compose through actual control flow; they do not cover all helpers.
Epoch memory: empty learned support, lossy projections and non-atomic effects cannot inherit stronger downstream guarantees.

## models

## behavior_complement
Finite beam not global optimum; empty words empty; no supported candidates can panic at history lookup; epochs0 may mark every token suspect.

## reverse_reconstruction
Many weights/training sets/hash collisions share outputs; forced tags constrain outputs without identifying weights; averaging discards update history.

## forced_priority
forced.get(i) tag precedes frequent known and may be unseen; missing forced entries fall back normally.

## averaging_clock
now increments only non-known token decisions; averaged cell=(total+(now-stamp)*w)/max(now,1).

## beam_merge
Stable descending scores choose earlier tag on ties; dedup retains first last-two-tag state; beam.max(1) excludes width0 emptiness.

## robust_environment
EN_ROBUST_FROM defaults3; context seeded RNG environment delegated to ctx; wrong u8 saturates and can no longer equal epoch+1 past255.


## tests

### Actual source checks
Named check: ptag::wave27_tests::forced_tags_override_seen_and_zero_beam. Command: cargo test -p en --lib wave27_tests -- --test-threads=1 (also --release). Four runs: each 10 passed, 0 failed, numeric exit0; sorted named outcomes equal. This file contributes one named check per run.
Control present for this source: false. Hash zeroed byte contribution, text changed empty chrF to1, lexgen removed three-cell PastPart copy: each compiled and failed one assertion (exit101), then exact source restoration and ten-test positive rerun (exit0). Other sources have no compiled mutation claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: read complete source before prospective model and bounded tests. Production code retained byte-for-byte; only inline synthetic checks added. Coverage is partial, not a proof.

### 2026-10-31 · tests

See evidence.json and external octopus-wave27-lexical-2026-10-31 receipts. Static built dictionary read is part of the linked library; no corpus or external services used. Unknown support, malformed model/blob, resource failures, cycles and exhaustive Unicode behavior are unexecuted gaps.

### 2026-11-04 · descriptant

2026-11-04 epoch (en-morph): dict_mask takes the derivation tags (derive::hint) for a form the dictionary does not know, unless the word is capitalised inside the sentence. Source e7ca7a999b6b; earlier receipts and evidence refer to the previous source hash and are kept as history.

2026-11-04 epoch (lexicon growth): dict_mask for a form the dictionary lacks takes lexext::hint(word, capitalised inside the sentence) first, then the derivation's tags; the case gate keeps acronym rows to words in capitals and name-list rows to capitalised words. The mask is still a tag set the model has weights for; en.bin unchanged (EWT dev XPOS 94.40 → 94.41, GUM 95.74 → 95.77).

### 2026-11-04 · approximant

2026-11-04: dict_mask(w, derive) = dictionary tag mask, else (derive ∧ not capitalised inside the sentence) the mask of derive::hint(lower w).

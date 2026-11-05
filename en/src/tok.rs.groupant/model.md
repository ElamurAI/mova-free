# en-tok

## descriptant

### en-tok
Eight-state thirteen-class FSA with lexical split/merge policies

## approximant

### Forward relation
Back pops joiner and reprocesses character; sentinel NUL flushes. Period, leading apostrophe, number suffix, cannot, clitic, hyphen rules ordered. Whole hyphen precedes split; prefix split>keep else default; adjacent punctuation merges last.

No executable Rust surrogate attached: no demonstrated metric for preserving these categorical, stateful or effect observations. Supported future caps are 1/2/5/10%, not a claim of current error.

## invariant

### Domain, invariant and cost
Back pops joiner and reprocesses character; sentinel NUL flushes. Period, leading apostrophe, number suffix, cannot, clitic, hyphen rules ordered. Whole hyphen precedes split; prefix split>keep else default; adjacent punctuation merges last.
trace omits sentinel and Back reprocessing; embedded NUL flushes and disappears; no original whitespace preservation.
FSA O(chars), recursive splits and repeated merge concatenation potentially quadratic; usize/u32 learning counts unchecked.
Inputs must satisfy the indexing/generated-data assumptions stated by the typed catalog; allocation failure and overflow beyond bounded fixtures remain untested.

## composition

### Mathematical combiner
Source bytes and twice-generated syntax constrain this model. Forward relation: Back pops joiner and reprocesses character; sentinel NUL flushes. Period, leading apostrophe, number suffix, cannot, clitic, hyphen rules ordered. Whole hyphen precedes split; prefix split>keep else default; adjacent punctuation merges last.
Reverse relation: Rendered token text admits many original whitespace runs and lexicons; whitespace exact recovery impossible.
The ordered typed family records independent exclusions, compatible antecedents and source-specific boundaries. Fragment ranges compose through actual control flow; they do not cover all helpers.
Epoch memory: empty learned support, lossy projections and non-atomic effects cannot inherit stronger downstream guarantees.

## models

## behavior_complement
trace omits sentinel and Back reprocessing; embedded NUL flushes and disappears; no original whitespace preservation.

## reverse_reconstruction
Rendered token text admits many original whitespace runs and lexicons; whitespace exact recovery impossible.

## fsa_transition
Table class order and state order are numeric explanation interfaces; Back reprocess differs from trace's simplified state reset.

## split_precedence
Hyphen whole beats split even if both sets contain word; prefix ties keep, learned global empty tie keeps unlike default false.

## spacing_relation
Spacing flag marks presence rather than amount; splits put original flag only on last part, merges inherit final flag.

## lexicon_ties
kept>=split selects global keep; prefix sp>k selects split; unordered set membership does not impose traversal tie dependence.

## tests

### Actual source checks
Named check: tok::wave27_tests::spaces_hyphen_precedence_and_nul. Command: cargo test -p en --lib wave27_tests -- --test-threads=1 (also --release). Four runs: each 10 passed, 0 failed, numeric exit0; sorted named outcomes equal. This file contributes one named check per run.
Control present for this source: false. Hash zeroed byte contribution, text changed empty chrF to1, lexgen removed three-cell PastPart copy: each compiled and failed one assertion (exit101), then exact source restoration and ten-test positive rerun (exit0). Other sources have no compiled mutation claim.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: read complete source before prospective model and bounded tests. Production code retained byte-for-byte; only inline synthetic checks added. Coverage is partial, not a proof.

### 2026-10-31 · tests

See evidence.json and external octopus-wave27-lexical-2026-10-31 receipts. Static built dictionary read is part of the linked library; no corpus or external services used. Unknown support, malformed model/blob, resource failures, cycles and exhaustive Unicode behavior are unexecuted gaps.

### 2026-11-05 · lexicon_ties

Serialization (2026-11-05, retrain-clean): the six HashSet fields and hyphen_prefix are written sorted (sorted_set, sorted_map: the same sequence/map encoding), so the same training files give the same model bytes; before, two identical en trainings differed in 21 122 bytes (process-random HashSet order) while annotating identically. Deserialization is unchanged: models written before load as before.

### 2026-11-05 · tests

tok::tests::lexicon_bytes_do_not_depend_on_hash_order (2026-11-05): two lexicons filled in opposite orders encode to the same bytes and read back equal; control: two plain HashSets of the same words encode differently; removing sorted_set from abbrev fails it. Two full en trainings on the same files: identical model SHA-256 (bff784ac…).

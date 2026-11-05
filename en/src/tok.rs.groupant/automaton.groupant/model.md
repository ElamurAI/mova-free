# en-tok-automaton

## descriptant

### en-tok-automaton
The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.

## approximant

### Scoped forward relation
The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
No executable percentage approximant attached.

## invariant

### Scoped assumptions and costs
trace omits sentinel and Back reprocessing; embedded NUL flushes and disappears; no original whitespace preservation.
FSA O(chars), recursive splits and repeated merge concatenation potentially quadratic; usize/u32 learning counts unchecked.

## composition

### Parent composition
The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
Parent combines this fragment with the other named source fragments and helpers outside their ranges. No coverage claim for unscoped code.

## models

## behavior_complement
Within this fragment's byte scope: The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
Parent relation constraining compatible states: trace omits sentinel and Back reprocessing; embedded NUL flushes and disappears; no original whitespace preservation.

## reverse_reconstruction
Within this fragment's byte scope: The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
Parent relation constraining compatible states: Rendered token text admits many original whitespace runs and lexicons; whitespace exact recovery impossible.

## fsa_transition
Within this fragment's byte scope: The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
Parent relation constraining compatible states: Table class order and state order are numeric explanation interfaces; Back reprocess differs from trace's simplified state reset.

## split_precedence
Within this fragment's byte scope: The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
Parent relation constraining compatible states: Hyphen whole beats split even if both sets contain word; prefix ties keep, learned global empty tie keeps unlike default false.

## spacing_relation
Within this fragment's byte scope: The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
Parent relation constraining compatible states: Spacing flag marks presence rather than amount; splits put original flag only on last part, merges inherit final flag.

## lexicon_ties
Within this fragment's byte scope: The state/class table drives append/new/one/sep/end/back. Back removes one pending joiner and replays the current character; Sep sets the last token flag. The terminal NUL flushes. Trace is a separate simplified explanation, not this operational recurrence.
Parent relation constraining compatible states: kept>=split selects global keep; prefix sp>k selects split; unordered set membership does not impose traversal tie dependence.


## tests

### Bounded parent evidence
See ../model.md#tests. Actual check: tok::wave27_tests::spaces_hyphen_precedence_and_nul; shared source evidence, not independent exhaustive fragment testing.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-scoped partial model; parent tests provide finite shared evidence.

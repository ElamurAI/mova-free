# world-ftqa-reconstruction

Normalized section reconstruction and question indexing.

## forward

csv folds character/quote state into ordered fields; read_rows checks the exact nine-column header and locality tokens. reconstruct groups stories by first occurrence, normalizes local sections, greedily splits summaries by longest prefix and inserts unknown chunks of at least five words. Qa numbers restart per story. text_of splits punctuation/quotes, then story_tsv/qa_tsv serialize. read_qa delegates unchanged decoding to a pure private helper.

## invariant

For nonempty known matches each decompose iteration consumes positive bytes. Empty normalized local sections cannot match; an empty local question still refuses. Accepted read_rows has exactly nine fields and locality in {local,summary}. In the practical domain of at most 65535 sections, generated section IDs are one-based; public decoder permits zero and duplicates.

## cost

For L input bytes CSV is O(L) output storage. Reconstruction scans rows for each distinct story and uses repeated linear section membership/search. With K known sections and L summary bytes, a conservative decomposition bound is O(K*L^2) plus copying; orphan insertion reruns decomposition and shifts vectors. Sentence splitting is linear except bounded lookahead and repeated current-quote word counts. No measured timing bound.

## composition

The section vector mutates during orphan discovery; final question mapping recomputes decompose after all shifts.

## epoch-memory

First source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

## checks

Run cargo test -p world --lib ftqa::wave9_models -- --test-threads=1. See the root sidecar ../model.md#evidence for repeated debug/release checks and compiled controls. Partial syntax excludes macro expansion and type/name resolution. No executable percentage approximant is claimed.

## behavior-complement

Subject: current bound source and this node's scope. Domain and observation: On finite nonempty known candidates, no iteration preserves remainder byte length; returned local Qa cannot have zero or multiple sections. Empty local inputs refuse rather than matching every text.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## reverse-reconstruction

Subject: current bound source and this node's scope. Domain and observation: Fix final sections and ordered Qas. Keep all row interleavings across distinct stories that preserve each story's local order and first-story order. Normalized section whitespace remains ambiguous.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## threshold

Subject: current bound source and this node's scope. Domain and observation: Four-token unknown summary has no recovered section; five-token unknown becomes an orphan. Existing known chunks match regardless of length. This distinguishes unknown threshold from known section acceptance.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## index-shifts

Subject: current bound source and this node's scope. Domain and observation: Insert orphan after previous match, then rerun matching. Three-section fixture maps the summary to [1,2,3] and the later local section to its shifted number, preserving restart of Qa n in the second story.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

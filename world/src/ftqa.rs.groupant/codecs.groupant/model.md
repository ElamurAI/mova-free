# world-ftqa-codecs

Sentence segmentation, TSV codecs and import side effects.

## forward

csv folds character/quote state into ordered fields; read_rows checks the exact nine-column header and locality tokens. reconstruct groups stories by first occurrence, normalizes local sections, greedily splits summaries by longest prefix and inserts unknown chunks of at least five words. Qa numbers restart per story. text_of splits punctuation/quotes, then story_tsv/qa_tsv serialize. read_qa delegates unchanged decoding to a pure private helper.

## invariant

For nonempty known matches each decompose iteration consumes positive bytes. Empty normalized local sections cannot match; an empty local question still refuses. Accepted read_rows has exactly nine fields and locality in {local,summary}. In the practical domain of at most 65535 sections, generated section IDs are one-based; public decoder permits zero and duplicates.

## cost

For L input bytes CSV is O(L) output storage. Reconstruction scans rows for each distinct story and uses repeated linear section membership/search. With K known sections and L summary bytes, a conservative decomposition bound is O(K*L^2) plus copying; orphan insertion reruns decomposition and shifts vectors. Sentence splitting is linear except bounded lookahead and repeated current-quote word counts. No measured timing bound.

## composition

text_of chooses the more frequent standalone quote, ties double quote, then emits one-based paragraph IDs. Import writes story and QA files independently, not atomically.

## epoch-memory

First source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

## checks

Run cargo test -p world --lib ftqa::wave9_models -- --test-threads=1. See the root sidecar ../model.md#evidence for repeated debug/release checks and compiled controls. Partial syntax excludes macro expansion and type/name resolution. No executable percentage approximant is claimed.

## behavior-complement

Subject: current bound source and this node's scope. Domain and observation: Pure QA decoding cannot succeed with fewer/more than nine fields or u16 overflow. It can accept a wrong header beginning with # because all such lines are comments.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## reverse-reconstruction

Subject: current bound source and this node's scope. Domain and observation: TSV observation omits doc IDs and original story identity. Sentence whitespace and quote choice counts can be lost. Decode/encode identity holds only for canonical delimiter-free QA rows, as tested.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## quote-attribution

Subject: current bound source and this node's scope. Domain and observation: A closing quote followed by a recognized speech verb among the next four allowed tokens wins over punctuation boundary. Fixture preserves asked she attribution before splitting next punctuation.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## serialization

Subject: current bound source and this node's scope. Domain and observation: story_tsv enumerates global sentence numbers independently of section IDs; qa_tsv emits comma-separated secs without sorting. The native join compares exact serialized bytes and confirms section duplicates survive.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

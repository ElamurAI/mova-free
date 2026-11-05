# world-ftqa

## model

# world-ftqa

CSV, tale reconstruction, sentence boundaries and QA interchange.

## forward

csv folds character/quote state into ordered fields; read_rows checks the exact nine-column header and locality tokens. reconstruct groups stories by first occurrence, normalizes local sections, greedily splits summaries by longest prefix and inserts unknown chunks of at least five words. Qa numbers restart per story. text_of splits punctuation/quotes, then story_tsv/qa_tsv serialize. read_qa delegates unchanged decoding to a pure private helper.

## invariant

For nonempty known matches each decompose iteration consumes positive bytes. Empty normalized local sections cannot match; an empty local question still refuses. Accepted read_rows has exactly nine fields and locality in {local,summary}. In the practical domain of at most 65535 sections, generated section IDs are one-based; public decoder permits zero and duplicates.

## cost

For L input bytes CSV is O(L) output storage. Reconstruction scans rows for each distinct story and uses repeated linear section membership/search. With K known sections and L summary bytes, a conservative decomposition bound is O(K*L^2) plus copying; orphan insertion reruns decomposition and shifts vectors. Sentence splitting is linear except bounded lookahead and repeated current-quote word counts. No measured timing bound.

## composition

main import calls read_rows -> reconstruct -> text_of -> TSV writers, preserving source-order story identity and shifted section indices. main measurement uses read_qa and read_story before report::eval_tale. The pure decode helper preserves path-derived story names and original read_qa errors. Tests exercise CSV/reconstruction/text/QA serialization joins without filesystem import or answering.

## epoch-memory

First source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

## checks

Run cargo test -p world --lib ftqa::wave9_models -- --test-threads=1. See the root sidecar model.md#evidence for repeated debug/release checks and compiled controls. Partial syntax excludes macro expansion and type/name resolution. No executable percentage approximant is claimed.

## behavior-complement

Subject: current bound source and this node's scope. Domain and observation: Universe: text/rows/output triples with finite strings. Inner absent relation excludes success for an unclosed CSV quote, a wrong exact header, invalid CSV locality, wrong QA column count or unparsable u16 ID. It does not exclude all malformed RFC CSV: quotes inside nonempty unquoted fields and post-quote text are tolerated. Named header/decode tests witness boundaries.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## reverse-reconstruction

Subject: current bound source and this node's scope. Domain and observation: For observed decoded rows retain every CSV text yielding those rows, including quoted/unquoted spellings and CR outside quotes. For reconstructed sections retain all whitespace spellings and duplicate local rows compatible with the observed question list. Output-only sections lose question/answer metadata. Native three-whitespace reconstruction fiber is complete only over those spellings.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## csv-empty

Subject: current bound source and this node's scope. Domain and observation: Empty input maps to no rows, newline maps to one empty field, and two quote characters at EOF map to one empty field. Quoted-field presence persists to EOF and resets on a record newline. Independent canonical encoder checks 196 single/two-field calls, with and without CRLF, over seven field atoms.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## header-projection

Subject: current bound source and this node's scope. Domain and observation: read_rows preserves story bytes but trims section/question/answers/attribute/ex fields. Header order and locality are exact, so whitespace variants of locality refuse while whitespace variants of other fields share decoded fibers.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## decomposition-progress

Subject: current bound source and this node's scope. Domain and observation: At each nonempty remainder r, successful known match has positive byte length or unknown cut is positive. Therefore |r| strictly decreases. Empty known sections are excluded from both prefix and nearest-match scans. The old binary timed out after one second on the named empty-candidate test; fixed test terminates.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## orphan-threshold

Subject: current bound source and this node's scope. Domain and observation: Unknown chunks with fewer than five whitespace tokens are ignored, at least five are inserted after the previous matched known section or at zero. A summary consisting solely of four words refuses for missing section; a five-word summary yields one orphan. Insertion is heuristic, not recovery of authorial order.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## greedy-fiber

Subject: current bound source and this node's scope. Domain and observation: For a fixed known list, B(parts)={summary | greedy longest-prefix splitting yields parts}. Substring matches need no word boundary. Equal-length identical known entries choose the last maximum. Normalization erases whitespace; chosen parts do not prove unique original segmentation.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## quote-boundary

Subject: current bound source and this node's scope. Domain and observation: Standalone quote tokens toggle quote state. Punctuation consumes following punctuation and a closing quote; a speech verb among the next four alphanumeric tokens suppresses a boundary. Within a still-open quote fewer than twelve words suppress it. Token sequence is retained modulo whitespace normalization, not semantic sentence truth.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## qa-path-fiber

Subject: current bound source and this node's scope. Domain and observation: qa_tsv omits story; read_qa recovers it from file stem. Fixing TSV bytes leaves all original story names compatible. QA secs retain order, duplicates and zero; unknown locality strings all decode false. Roundtrip identity is restricted to delimiter-free fields and local/summary flags; it does not recover original path.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## format-domain

Subject: current bound source and this node's scope. Domain and observation: TSV writers do not escape tabs/newlines in fields. Story decoder splitn(3) retains tabs in text while QA requires exactly nine columns. Section casts use u16 and can overflow/truncate beyond representable IDs; huge corpora, path traversal and partial filesystem writes are outside the bounded evidence.

Status: source-derived partial relation, supported only where named finite fixtures apply. Assume successful allocations, fixed dependencies and bounded native inputs; external filesystem/answer-generation effects remain unknown. The relation describes one terminating call, not all process traces.

## evidence

### Bounded native evidence

CSV canonical encoder exercises 196 calls; reconstruction checks three whitespace fibers and four/five-word thresholds. Aggregation checks 216 source/score cases plus nine pair fibers. Sampling checks two salts and five budgets with exclusion/permutation fixtures; collisions remain outside that invariance claim.

Old CSV code fails csv_finite_encoder_oracle on a quoted empty field at EOF. Old empty-known candidate binary is stopped by an owned one-second timeout (exit 124); the fixed direct test terminates. Compiled mutants change orphan threshold five to six and answered denominator to all rows, respectively; each produces a named assertion failure, not a compile error.

Structural syntax projections and registry checks bind artifacts, not semantic truth. File import/read transactions and eval_tale answering remain source-inspected only. Public structs, unrestricted metadata, section counts above u16, accepted nonfinite/manual scores and unknown types remain explicit domain limits.

Structural validation: two independently generated syntax JSON files per source compare byte-identically. An external registry retains existing registered nodes under an isolated temporary root plus these two leaves; groupants reports valid=true with no diagnostics, and changed reports both owned sources with no missing registrations. The temporary root intentionally has no repository sidecar; integration ancestors belong to the root agent.

## History

### 2026-10-30 · epoch-memory

2026-10-30: first source-bound modeling epoch. Synthetic native fixtures only; no held-out tales, live parsers or annotators. CSV lost a quoted empty final record; a zero-length known section prevented decomposition progress. The repaired scanner retains quoted-field presence and decomposition ignores empty candidates. Report observations preserve existing permissive score/label semantics rather than silently enforcing a new measurement policy. Native finite evidence does not establish unrestricted correctness.

### 2026-10-30 · evidence

2026-10-30. Twelve new tests (six per file) pass twice in debug and twice in release with one test thread; normalized test identities/results match across all four runs. The restored production bytes pass twelve tests again after controls. No held-out data, live parser, annotator, teacher or GPU was invoked. Existing unrelated compiler warnings remain.

Raw logs: $MOVA_DATA/runs/octopus-wave9-input-2026-10-30/.

- old-csv.log: SHA-256 2f4192a8d7e601430b29c46e4599ec12de09a59650a5d0d4f334b173aa94d358

- old-progress.log: SHA-256 11d953211240bedb2e9e1a08d22a8648e9150db53ed1af2dda0f7a632f19e98d

- debug-1.log: SHA-256 dd36598815808526e70d8f81e2e975848c8a27530c6db317d8190aa41358f41a

- debug-2.log: SHA-256 9ea954b576e5750e2261d40269bdd4114ca2629b29ae1f3c6ac2586abb18fd5e

- release-1.log: SHA-256 775721320b289426e2c1988d5f250b548a19a275a0bcbfe60b5f22b253baa98c

- release-2.log: SHA-256 50412e040cfa318f3351da7c87503902f61acb2d499e1c3c803673ccdcc74c24

- mutant-ftqa.log: SHA-256 2e78f80ab321ac151c458fd5ea9d5f58ba5ecbfb8afd275ea71955e838e21e22

- mutant-report.log: SHA-256 d052828184aff07844b48d407e8f40ff0bcc216b5427dc5af79a0d7b17a4df03

- restored.log: SHA-256 06ec03b5b2ed109aea50cd5d592b59c9c7d3e98d6a3ad82859a68a4eaeab056b

Final source SHA-256: df08ab40304e843e455d7a778207cde257e87ea224c59e922693774c359364aa.

- groupants-full.json SHA-256: 004960d36ae5898dbb9da1f797cccd1195faeab73f9e0f676e4141045e36573c

- changed.json SHA-256: 27c2b43111467637ca924746f390c47d76c4142bd22a59b34d237e54de03e19f

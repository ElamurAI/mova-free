# prag-feats

## model

# prag-feats

Build 32 pragmatic feature slots from quote-selected utterances, normalized UD observations, cues and paragraph metadata; source-qualified previous-act indices and training dictionaries.

## forward

For item i, extract resets quote state unless adjacent input has equal source/doc/paragraph and ordinal +1, excluding Tatoeba. quote_spans feeds row; row invokes the real annotator through row_with, producing 32 slots in SLOTS order with prev=start. prev_of independently builds source/doc/ordinal predecessors. Dict builds per-slot first-observation IDs with previous-act vocabulary seeded before rows.

## behavior-complement

Universe: valid item sequences, supplied annotation observations, valid cue tables and dictionaries paired with candidate rows/indices/IDs. Absent behavior includes a row with a different slot count, prev initialized to an act, a Tatoeba predecessor, a predecessor from another source, or an encoded missing value other than UNSEEN. These are source-derived constraints (inner bound), not an exhaustive complement of live annotation.

## reverse-reconstruction

Compatible antecedents retain every item text, quote state, annotation and cue configuration satisfying all observed coordinates and IDs. Normalization, buckets, omitted punctuation and discarded dictionary index prevent unique recovery. Equal rows may arise from distinct casing, text and annotations; IDs additionally require the exact dictionary.

## invariant

32 slots; quote intervals are UTF-8 bounds when produced by quote_spans; dictionary order is training traversal order; source isolation is distinct from licensing. This feature module does not enforce train/evaluation separation.

## cost

With B text bytes, W words, P cues, maximum phrase L, row costs O(B+W*P*L) plus annotation; annotation cost is external. Quote state scans allocate O(B); predecessor map expected O(N) time/space; dictionaries expected O(total string bytes) hashing and storage. No measured speedup or executable approximation error bound is claimed.

## coupling

Quote carry resets at paragraph boundary while prev_of deliberately crosses paragraph boundaries. Rows leave prev=start; eval train_rows/predict_seq inject silver/predicted previous acts. eval run builds Dict only from training rows; model.rs consumes Dict, SLOTS and UNSEEN. Native seam covers downstream projection, not actual annotation.

## observation-fiber

The observation (row, encoded IDs) cannot recover the full UD tree or original text; normalization WO and 'll both reach will. Predecessor indices also discard duplicate-key earlier records through last-wins insertion.

## composition

Quote carry resets at paragraph boundary while prev_of deliberately crosses paragraph boundaries. Rows leave prev=start; eval train_rows/predict_seq inject silver/predicted previous acts. eval run builds Dict only from training rows; model.rs consumes Dict, SLOTS and UNSEEN. Native seam covers downstream projection, not actual annotation. Immediate child scopes share the state described above; overlapping scopes do not add costs independently.

## checks

All six feats::wave13_models tests. Native public joins: quote_spans to row_with, row_with to Dict::build/encode/show and serialization/reindex; prev_of on supplied Items. Caller wiring inspected in eval.rs and model.rs; no live eval/parser. See the file-level model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## admissible-bounds

Extraction assumes ordinal increment fits i64 and paragraph position increment fits usize; extreme inconsistent metadata is not hardened here. Dictionary slot/value accesses assume valid indices and fewer than u32::MAX distinct values per slot, with UNSEEN reserved. Private span slicing assumes ordered, nonoverlapping UTF-8 intervals produced by quote_spans. Hand-built UD fixtures need not be trees, but these checks do not validate malformed head graphs. These limits predate this change; the witnessed fix is limited to source identity.

## evidence

### Evidence

Six native tests in feats::wave13_models passed twice debug and twice release with equal normalized names/results. The compiled old-code source-collision witness failed its source-qualified predecessor assertion; five other tests passed. A compiled production mutant replaced utterance question detection by false; bucket_boundaries_and_terminal_priority and supplied_ud_projection_and_quote_selection rejected it by assertions. Exact original bytes were restored and all six tests passed again.

No live annotation, external model, corpus, teacher, GPU or held-out runs. Two syntax generations are byte-identical; macro bodies remain opaque.

Full-base external registry scaffold adds this leaf alongside the unchanged existing root. Structural groupants reports repeat byte-identically with no diagnostics; one intentionally absent scaffold parent sidecar is reported. Changed check reports only prag/src/feats.rs and no missing registration. No central registry or parent files were edited by this worker.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

External run artifact hashes (run label octopus-wave13-feats-2026-10-30):

- prospective.md: bbc1081a20822e8128cc28d34b37ac2b702954f50ff50b29ad1445882e52ce5d

- old-code-witness.log: e0b15a11918442a7afed4eefa7b3ac33dee8e19a6debc9f3123c25018bf3c838

- debug-1.log: e4652a60aea90f6018c9ca97c1dd1b2af13e90c5689b68cc4a90b0707ff2c553

- debug-2.log: 796399b63172762ff678556ec92fe519a44ea62be024545044fe3d526073383f

- release-1.log: f7bae9e8446d5d70e3226df0379155bd36f0ba5720af0f71423f2332d4b4bd13

- release-2.log: 0a0111ad863ec6059793d9d3552766d9a687ecd46720b771f47048a44645f12b

- debug-1-named.txt: 81dfadad59e8a1ad9663cadcb4eed5b4056918c9b11c600e38476ca95af6ea3c

- release-2-named.txt: 81dfadad59e8a1ad9663cadcb4eed5b4056918c9b11c600e38476ca95af6ea3c

- mutant-question.log: b319e74b598a949521c23ba735311c016bfebd6efaacff2a9ad35ad1f6a844a5

- restored.log: 3dd36f72a82409c124fcb8919c3695a29bd9718fe7911f6641e9462dfa1f92fc

- syntax-1.json: fabe1bc9d1e057d661dcc4b208a39e19722219774525af31e3430504dd8c7724

- syntax-2.json: fabe1bc9d1e057d661dcc4b208a39e19722219774525af31e3430504dd8c7724

- full-groupants-1.json: acd13f6fbb31604dcd6e09998e198226ef462d7b3dab98d001d6b97bc888158e

- full-groupants-2.json: acd13f6fbb31604dcd6e09998e198226ef462d7b3dab98d001d6b97bc888158e

- changed.json: 831bf0cab9068708b73d198a500f6d95a874df01c95000c78f42af9c0cd335d0

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.

# prag-feats-dictionary

Training-order per-slot dictionaries with complete previous-act vocabulary and explicit unseen sentinel.

## forward

build seeds prev with Act::ALL names then start, inserts each row value once in traversal order. id looks up slot map or UNSEEN; encode projects every supplied coordinate. Serialization skips index; reindex recreates per-slot maps from values. show uses '?' for UNSEEN and SLOTS names.

## behavior-complement

Universe: valid-width rows and built/reindexed dictionary plus candidate IDs. Excludes ID reuse for distinct values within a slot, unseen training values after build, missing Act IDs in prev, and encoding an absent value as an ordinary ID. No guarantee for deliberately malformed serialized duplicate values.

## reverse-reconstruction

Given values retain every training row history with identical per-slot first-appearance order and arbitrary repeats. Given UNSEEN retain every absent string, not one unknown spelling. Numeric IDs require dictionary and slot to interpret; they alone do not reconstruct rows.

## invariant

Slot index must be in dictionary range; non-UNSEEN show ID must be in values range. Rows longer than SLOTS panic; shorter rows encode shorter vectors. Reindex is required after deserialize; no automatic repair.

## cost

Expected total input string hashing/copy time and distinct-value storage. reindex copies every retained value; encode expected O(row string bytes); show allocates explanation string. No measured speedup or executable approximation error bound is claimed.

## coupling

Dict receives training rows after prev injection; it pre-seeds all acts so predicted but unseen training acts still encode. Persisted model users must reindex before id/encode. Model and eval callers maintain that lifecycle.

## observation-fiber

Training counts and repeated-row order are forgotten except first appearances. Serialized values retain decoding data but discard the hash index. UNSEEN collapses all unobserved values and show prints one '?'.

## composition

Dict receives training rows after prev injection; it pre-seeds all acts so predicted but unseen training acts still encode. Persisted model users must reindex before id/encode. Model and eval callers maintain that lifecycle. No child fragments; this is one scoped relation.

## checks

empty_row_literal_and_dictionary_roundtrip covers every empty feature coordinate, Act coverage, missing value, show, serde omission, reindex, equal encoding and serialized bytes. See the file-level ../model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.

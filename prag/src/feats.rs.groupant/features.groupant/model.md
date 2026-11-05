# prag-feats-features

Compose quote selection, annotation and feature projection on the production path.

## forward

row delegates to row_with with a closure that tokenizes once and annotates nonempty forms. row_with selects utterance/frame, invokes that closure once, then projects normalized words/tree/cues and metadata into SLOTS.

## behavior-complement

Universe: valid item/span/carry and supplied Word observations to candidate rows. Excludes duplicated annotation calls, feature rows with quote text selected but narrator tokens used as utterance, and rows whose prev slot is not start. Source-derived partial exclusion, with annotation internals outside the model.

## reverse-reconstruction

For a row retain all selection decisions, frames and Word observations whose projection matches. Unobserved tag/head/feature differences remain candidates if they do not affect selected coordinates. A row is not an inverse parse.

## invariant

Closure is FnOnce and remains on the real row path. Frame and utterance derive from the same retained spans. Hand-built words are test observations, not asserted parser outputs.

## cost

O(B+W*P*L) excluding tokenizer/annotation, plus cloned strings and cue vectors. No measured speedup or executable approximation error bound is claimed.

## coupling

Selection child determines utt/frame/quote; projection child consumes the same values and caller Item. Composition requires shared retained spans rather than independent possible utterances and frames.

## observation-fiber

Slots retain selected first/last words, root and cue categories, not the complete utterance or parse. Row length stays 32 even for empty annotation.

## composition

Selection child determines utt/frame/quote; projection child consumes the same values and caller Item. Composition requires shared retained spans rather than independent possible utterances and frames. Immediate child scopes share the state described above; overlapping scopes do not add costs independently.

## checks

supplied_ud_projection_and_quote_selection and empty_row_literal_and_dictionary_roundtrip connect literal supplied UD observations through the same production feature function. See the file-level ../model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.

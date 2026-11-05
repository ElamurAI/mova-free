# prag-feats-quotes

One-delimiter quote automaton emits byte spans and persistent open state.

## forward

Scan char_indices; double open marks always start when closed; single open marks require nonword predecessor and alphanumeric successor. Matched straight/curly closers emit [start,byte); single apostrophes between alphanumerics do not close. If still open append [start,text.len). Filter empty and nonalphanumeric spans without changing final delimiter state.

## behavior-complement

Universe: valid UTF-8 text and states None or one supported opening delimiter paired with intervals/state. Excluded intervals split Unicode codepoints, include their opening delimiter, have empty content, or consist solely of punctuation. A closing apostrophe inside don't cannot terminate a single-quoted span.

## reverse-reconstruction

An observed empty span list permits unquoted text, punctuation-only quoted text or empty carried text. Retain all texts and opening states whose automaton ends in observed state and emits exact intervals; absent text content and delimiter style cannot be uniquely recovered from interval lengths.

## invariant

Bounds increase and are valid UTF-8 boundaries; emitted content contains an alphanumeric. Open state may survive even when no span survives filtering. Nested quotation is not represented as a stack.

## cost

O(B) time and O(B) temporary char-index storage plus spans for B bytes. No measured speedup or executable approximation error bound is claimed.

## coupling

extract supplies carry only at adjacent same paragraph/source/doc ordinal. row applies a second direct-speech filter: quote_spans detects delimiters, not narrative intent.

## observation-fiber

Straight and curly double closers may complete the same state; punctuation-only quotes disappear from span output but can affect carry. Filtering spans therefore cannot reconstruct raw delimiter events.

## composition

extract supplies carry only at adjacent same paragraph/source/doc ordinal. row applies a second direct-speech filter: quote_spans detects delimiters, not narrative intent. No child fragments; this is one scoped relation.

## checks

quote_unicode_state_and_content_bounds: six literal texts, mixed Unicode, apostrophe interior, adjacent spans, carried Unicode and empty carry. See the file-level ../model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.

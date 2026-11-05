# prag-feats-features-selection

Filter quotation as speech, concatenate utterance and retain narrator frame.

## forward

Initial frame speech cue admits all candidate spans. Otherwise admit carried span at byte zero, at least four whitespace words, no preceding alphanumeric, or !/? ending. Join trimmed retained contents with spaces; if none retain entire text and empty frame. quote is cont if carried, else none/whole/framed according to retained spans and alphabetic frame.

## behavior-complement

Universe: well-formed ordered byte spans of item text and carry flag to (utterance,frame,quote). Excludes selecting an interior one-word title without speech cue, and quote=whole when retained spans leave alphabetic narrator text. quote=cont has priority even with no retained span.

## reverse-reconstruction

For selected utterance/frame retain compatible delimiter spellings and whitespace variants inside span edges; joining multiple spans loses individual boundaries. quote=none admits unquoted texts and rejected short quotations.

## invariant

frame_of requires ordered nonoverlapping valid UTF-8 bounds; public quote_spans provides them. This private function does not validate arbitrary supplied intervals.

## cost

Text/frame scans and string copies O(B) plus narrator cue matching; candidate whitespace scans total O(B) on nonoverlapping spans. No measured speedup or executable approximation error bound is claimed.

## coupling

Speech detection uses initial candidate frame; final sayv later uses frame after filtering. These frames may differ; retain both in the state model.

## observation-fiber

Trim and join discard boundary whitespace and number of quotation spans. The speech heuristic does not distinguish every title from direct speech.

## composition

Speech detection uses initial candidate frame; final sayv later uses frame after filtering. These frames may differ; retain both in the state model. No child fragments; this is one scoped relation.

## checks

supplied_ud_projection_and_quote_selection supplies framed speech, rejected Book title, whole Hello and carried utterance with exact closure input assertions. See the file-level ../../model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.

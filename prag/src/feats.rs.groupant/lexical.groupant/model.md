# prag-feats-lexical

Normalize words and greedily consume cues in stable longest-phrase order.

## forward

N lowercases Unicode, replaces curly apostrophe and maps n't,'ll,'m,'re,'ve,'d,ca,wo,sha to not,will,am,are,have,would,can,will,shall. load parses embedded TSV, normalizes phrase tokens and stable-sorts descending length. find scans left-to-right and takes the first matching phrase, advancing by its length; otherwise advances one word.

## behavior-complement

Universe: nonempty phrase tables, token vectors, candidate ordered (index,class) lists. Excluded outputs include overlapping consumed hits, decreasing hit positions, hits beyond input bounds and a short match selected before an earlier-listed matching long phrase. This inner bound assumes no empty phrase, as guaranteed by the embedded source.

## reverse-reconstruction

For normalized word y retain all Unicode/case/apostrophe variants and contraction antecedents mapped to y. For hit lists retain all token sequences agreeing at consumed phrases plus arbitrary unmatched tokens, subject to no higher-priority match at scanned positions. will admits WO, 'll and will.

## invariant

A successful match strictly advances for nonempty phrases. find does not normalize its input; callers supply normalized tokens. Tie priority is embedded file order, not alphabetical class order.

## cost

Normalization O(input bytes) allocations; load O(P log P) phrase sort after parsing; find worst O(W*P*L), output at most W hits. No measured speedup or executable approximation error bound is claimed.

## coupling

frame_words normalizes narrator cues; utterance cues use annotation word forms and rewrite let 's to let us. Input tokenization can alter whether a multiword cue is visible.

## observation-fiber

Changing consumed phrase interior may remove a hit; changing an unconsumed token can leave the same output. Greedy consumption suppresses alternate overlapping interpretations, so inverse candidates must honor priority.

## composition

frame_words normalizes narrator cues; utterance cues use annotation word forms and rewrite let 's to let us. Input tokenization can alter whether a multiword cue is visible. No child fragments; this is one scoped relation.

## checks

normalization_and_greedy_cue_loss: ten literal normalizations, idempotence at those outputs, five-token greedy overlap example, embedded thank-you cue and ambiguous will antecedents. See the file-level ../model.md#evidence for exact log hashes and repeated run outcomes.

## epoch-memory

Wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave13: initial source-bound model. Prospective model preceded edits. Existing normalization and quote rules were retained; tests expose their information loss. Source identity witness justified qualifying predecessor keys by Src. Annotation is isolated behind a private closure used by the production row wrapper; fixtures do not call or certify tokenizer/parser. AST is a partial syntax projection, macro bodies opaque; semantic AST, live model/corpus and general proof remain absent.

### Undated · document

Source snapshot: 2a1a0bd6e43dd16f7284e993f30d0f4971c447c9e2080616033d19ac8693611f. All relations concern this snapshot on the stated admissible domain. Models are source-derived partial relations; finite fixtures are evidence, not general proofs. Complement is a conservative inner constraint model. Reverse reconstruction preserves all compatible antecedents conceptually; no enumerating solver is implemented.

# qagen-prompt-render

## forward
F(paras,rule)=rules(rule) + separator + ordered paragraph blocks. Paragraph i receives P(i+1); local sentence k receives [k+1]. Context is emitted iff nonempty and there is no immediate same-document predecessor whose checked successor equals current para. Domain: finite supplied Para references, including all i64 paragraph values; observation: exact rendered UTF-8 bytes.

## invariant
Original paragraph number is displayed but never replaces batch-local P index. Sentence IDs never replace local sentence coordinates. A predecessor in another document cannot suppress context. i64::MAX has no successor.

## behavior-complement
Output with wrapped successor context suppression, cross-document context suppression or global sent_id as local numeric labels is excluded. No semantic validation or sanitization of supplied textual fields is performed. Universe: supplied finite UTF-8 strings/Para sequences and candidate observable results. These are scoped source-derived exclusions, not merely negative test outcomes.

## reverse-reconstruction
Fibers contain Para sequences yielding equal rendered bytes. Book, block, words, dialogue, batch and sentence IDs can vary without effect. Hidden doc identifiers need only preserve adjacent equality decisions; suppressed context may vary while remaining nonempty. Delimiter-free trusted fields permit recovery of rendered title/author/paragraph/text, not omitted metadata. Operator is the set-valued fiber F^-1(y) = {x in domain | F(x)=y}; no implemented complete inverse solver or unique recovery claim.

## specialization
Context predicate is independently modeled by i128(prev)+1=current and document equality, avoiding overflow in the finite reference. Rules are distinct prefixes; changing only the rule preserves the paragraph body.

## cost
O(B+N+S) time and output space; no database or teacher invocation.

## composition
Schema version selects a fixed rules prefix; renderer appends batch-local paragraph and sentence coordinates. Context uses original document identity and paragraph number only for immediate-predecessor adjacency. Child scopes share source identity; inverses must share the same Rule and Para sequence rather than combining unrelated marginal fibers.

## checks
Six new wave17_models tests (two schema, four prompt), including 256 rule bytes and 98 successor cases. See root ../model.md#evidence for compiled controls and repeated build-profile outcomes.

## epoch-memory

Prospective review: unchecked predecessor addition could overflow; cross-document comparison already correct. A synthetic MAX to MIN witness panicked before repair. checked_add preserves ordinary adjacency and keeps context at the overflow boundary. Schema literal mapping had no witnessed defect. No API changes.

## limitations
Partial source models, with literal and finite observations; no analytical approximation error or speed claim. Syntax projection is partial, macro bodies opaque. Input text is rendered verbatim, including delimiter-like content; grammar-injective recovery requires trusted delimiter-free fields. Teacher, parser, corpus, database and data acquisition were not executed.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective review: unchecked predecessor addition could overflow; cross-document comparison already correct. A synthetic MAX to MIN witness panicked before repair. checked_add preserves ordinary adjacency and keeps context at the overflow boundary. Schema literal mapping had no witnessed defect. No API changes.

# qagen-prompt-rules

## forward
Select fixed literal RULES for V1 and RULES_V2 for V2; v2 requires an explicit verbatim span while v1 permits its historical wording.

## invariant
Original paragraph number is displayed but never replaces batch-local P index. Sentence IDs never replace local sentence coordinates. A predecessor in another document cannot suppress context. i64::MAX has no successor.

## behavior-complement
Output with wrapped successor context suppression, cross-document context suppression or global sent_id as local numeric labels is excluded. No semantic validation or sanitization of supplied textual fields is performed. Universe: supplied finite UTF-8 strings/Para sequences and candidate observable results. These are scoped source-derived exclusions, not merely negative test outcomes.

## reverse-reconstruction
The two distinct rule strings recover V1 or V2 uniquely; unrelated text is outside the image. Operator is the set-valued fiber F^-1(y) = {x in domain | F(x)=y}; no implemented complete inverse solver or unique recovery claim.

## specialization
Context predicate is independently modeled by i128(prev)+1=current and document equality, avoiding overflow in the finite reference. Rules are distinct prefixes; changing only the rule preserves the paragraph body.

## cost
O(1) static-string dispatch; prefix copy costs its fixed length.

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

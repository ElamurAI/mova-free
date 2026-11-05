# qagen-schema-version

## forward
Map byte 1 to V1, 2 to V2, all remaining 254 bytes to None; num is the inverse on accepted variants.

## invariant
ALL order is stable; canonical names are unique; parse(name(t))=Some(t); Rule::of(r.num())=Some(r).

## behavior-complement
Noncanonical normalized spellings, internal whitespace changes, abbreviations and version bytes outside {1,2} cannot produce accepted values. Universe: supplied finite UTF-8 strings/Para sequences and candidate observable results. These are scoped source-derived exclusions, not merely negative test outcomes.

## reverse-reconstruction
For accepted t, all s with lowercase(trim(s))=name(t) are compatible; rejected output collects every other string. Canonical name and version number uniquely recover their enum inside these closed alphabets. Operator is the set-valued fiber F^-1(y) = {x in domain | F(x)=y}; no implemented complete inverse solver or unique recovery claim.

## specialization
Normalization quotient identifies case and surrounding whitespace but preserves internal spaces; round trips alone cannot catch a consistently renamed alphabet, hence independent literals.

## cost
O(1) time and storage.

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

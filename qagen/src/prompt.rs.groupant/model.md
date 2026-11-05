# qagen-prompt

## model

# qagen-prompt

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
O(B+N+S) time and output space for rendered text B, N paragraphs, S sentences; context join adds O(context bytes) temporary allocation.

## composition
Schema version selects a fixed rules prefix; renderer appends batch-local paragraph and sentence coordinates. Context uses original document identity and paragraph number only for immediate-predecessor adjacency. Child scopes share source identity; inverses must share the same Rule and Para sequence rather than combining unrelated marginal fibers.

## checks
Six new wave17_models tests (two schema, four prompt), including 256 rule bytes and 98 successor cases. See root model.md#evidence for compiled controls and repeated build-profile outcomes.

## epoch-memory

Prospective review: unchecked predecessor addition could overflow; cross-document comparison already correct. A synthetic MAX to MIN witness panicked before repair. checked_add preserves ordinary adjacency and keeps context at the overflow boundary. Schema literal mapping had no witnessed defect. No API changes.

## limitations
Partial source models, with literal and finite observations; no analytical approximation error or speed claim. Syntax projection is partial, macro bodies opaque. Input text is rendered verbatim, including delimiter-like content; grammar-injective recovery requires trusted delimiter-free fields. Teacher, parser, corpus, database and data acquisition were not executed.

## evidence

### Wave17 prompt evidence

Six named wave17_models tests pass identically in two debug and two release runs: two schema and four prompt. The controls helper compared all six normalized named outcomes across all four final runs. No teacher, parser, corpus, database or data acquisition was run. Existing warnings in en and prag remain outside this change.

The original prompt fails maximum_predecessor_retains_context by integer overflow. Five compiled production mutations fail their intended assertions: schema canonical character name to person; version 2 to V1; prompt document equality to true; sentence coordinate k+1 to k+2; V2 rule dispatch to V1. Exact source bytes were restored before the final passing checks.

The alphabet oracle uses independently written literal names, the rule oracle covers all 256 bytes, and the context oracle covers 98 combinations using widened i128 arithmetic. The prompt golden body tests local coordinates and an omitted-metadata fiber. Finite evidence is not an exhaustive proof over arbitrary text.

External scaffold structural inventory reports both owned leaves available_partial with no diagnostics. Its three global diagnostics concern only the unchanged root group: new scaffold child and two transitive source bindings require integration refresh. This worker did not modify the global registry or parent groups.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective review: unchecked predecessor addition could overflow; cross-document comparison already correct. A synthetic MAX to MIN witness panicked before repair. checked_add preserves ordinary adjacency and keeps context at the overflow boundary. Schema literal mapping had no witnessed defect. No API changes.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: `2e30ac525e2bf8cf8432298ff35eaf2ec08933c17aff60e480017a2b7cca89f6`.

Raw run: `$MOVA_DATA/runs/octopus-wave17-prompts-2026-10-30`. Syntax projections generated twice per source and compared byte-identically; macro bodies remain opaque.

| Artifact | SHA-256 |
| --- | --- |
| old-witness.log | 6c0b3c7bc5790f6ab5224d022891635e276e2ec0b59ea51c998bdc9fdb24b45f |
| mut-schema-alphabet.log | 407156c2a9ce15dd7cde2096d08e914e779412d1a92c05bb207877f78ce008ca |
| mut-schema-version.log | 5339dd1bc8c3112f6aef8a7cf4ebe62ca14de914b5ac76fc00e2e2dd5718565c |
| mut-prompt-document.log | e4205fdf543c492536dc1b8071857c8000addb91f74c66a07431ab98bb1ab90c |
| mut-prompt-coordinate.log | ad1a3c83ae0aaea326118a5d9642ebd5ff778a1f135084420b89a0e2a6e4751e |
| mut-prompt-rule.log | 008c306e4c5e24edf84b8fba8f2b084e71031d3c2f65926ca2b8c39fff88d39d |
| final-debug-1.log | 0caa173d8034714dcb39a432fed15564c394080b7d8e8ee164a7e729ae21f1c2 |
| final-debug-2.log | 06e4ad17b3655831a247b18af805635266bd2e70382d1ce595d13b0bf6112853 |
| final-release-1.log | bf2b1c0924cd3643f2f2fbf2fb42bd07d5d304dbe233dc007b20547510402e4e |
| final-release-2.log | a7fa15bf1f617a592056ccdca6ccd45488617d6fac0ad2a6740a7500ce58b698 |
| controls.log | 6bd372ec64e9a4c76a5b4c2ed867f58cee16f335a16725670c987ec127d206c0 |

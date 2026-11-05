# Anchor and fallback arbitration

## forward
Scope: anchor_tree and read. Parse frame, calculate baseline, route candidate, then resolve anchor using checked one-based subtraction. Missing candidate and score<6 choose baseline. For confident candidates: high effective coverage and available span choose span unless describe/number has slot; otherwise slot, available span, baseline. Effective coverage is raised for happen clauses and adjective feeling spans.

## invariant
Zero/out-of-range anchors are absent; positive valid anchors select exactly the corresponding tree. An absent anchor does not erase an existing slot. Source label and sentence metadata follow the chosen branch, not a truth certificate.

## cost
Anchor lookup is O(1); external parsing, baseline, state routing and span extraction dominate. Strings are cloned for the selected answer and diagnostic fields.

## behavior-complement
A zero anchor cannot access the first tree; low confidence cannot return a state slot. If a candidate is confident but lacks both slot and span, state origin is impossible. Full language complement remains unknown.

## reverse-reconstruction
A sentence fallback hides whether state was absent, low confidence, or lacked an answer field unless reason/conf/fact are retained. A span may be chosen through either the high-coverage branch or no-slot fallback. Keep all branches compatible with the full observation.

## boundary
Correct index extraction does not ensure own and fw match; deliberate shuffle controls rely on this separation. Anchor arithmetic repair eliminates the observed debug zero-underflow only, not all sentence arithmetic elsewhere.

## composition
This is the QA join boundary among qframe, index, own text, span and baseline. It preserves parser acquisition. Candidate and lexical child models are necessary but insufficient to prove this complete interaction.

## checks
anchor_zero_and_range_are_absent covers zero, endpoints, overlength and u16::MAX with supplied trees. It failed on the original subtract-before-filter expression. Full read invokes the parser and is not exercised by these helper tests.

## epoch-memory

Predicted zero absence, observed debug panic, repaired with checked_sub. No positive-anchor semantics change intended; repeated bounded tests recorded separately. Partial syntax only.

## History

### 2026-10-30 · epoch-memory

2026-10-30: predicted zero absence, observed debug panic, repaired with checked_sub. No positive-anchor semantics change intended; repeated bounded tests recorded separately. Partial syntax only.

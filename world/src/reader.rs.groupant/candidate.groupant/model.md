# State routing

## forward
Scope: state_answer. Empty index yields None. Resolve main subject in characters, then objects/places; resolve others excluding subject. An accepted subordinate anchor needs score>=4 and predicate or at least one word. Dispatch on QKind to branch-specific searches, field/cause fallbacks and confidence adjustments. Num selects the first matching numeric attribute with confidence eight; Happen with an unresolved supplied subordinate refuses to guess.

## invariant
Candidate facts come from ix; rendered entities/events come from fw. This shared identity assumption is required externally. Branches preserve fact iteration order where they use first/last rather than explicit ranking. Candidate slot is normalized by Tale::cand outside this range.

## cost
Entity resolution, index scoring/ranking and rendering are external; repeated scans can visit F facts several times. Conservative source model O(F K W L) plus sorting and entity work, no measured bound.

## behavior-complement
Empty index cannot produce a candidate. Happen with a subordinate and no accepted anchor cannot select a later event through this branch. These exclusions do not prevent semantically spurious causes or entity resolution.

## reverse-reconstruction
A candidate retains slot, sentence, score, span kind, predicates and a label; multiple QKinds and worlds can render identical values. Reconstruct all routes compatible with the observed tuple and fixed dependency outputs; do not infer a unique event from rendered text.

## boundary
Temporal slack and fixed scores are heuristics. Sentence arithmetic assumes ordinary bounded story lengths; overflow near u16 limits is untested. The anchor fact may be from a shuffled world by caller design.

## composition
Consumes lexical projection, qframe roles and index/world identity. Produces a Cand consumed by arbitration; locally confident selection does not establish final answer quality or a valid own-text join.

## checks
Confidence and section-envelope native tests constrain helper assumptions; complete branch execution remains a gap. Catalog statements here are source inspection, not a claim that all routing arms have test coverage.

## epoch-memory

Preserve branch-specific temporal behavior and unknown parser/entity quality. Partial syntax is available; semantic interpretation is not mechanically certified.

## History

### 2026-10-30 · epoch-memory

2026-10-30: preserve branch-specific temporal behavior and unknown parser/entity quality. Partial syntax is available; semantic interpretation is not mechanically certified.

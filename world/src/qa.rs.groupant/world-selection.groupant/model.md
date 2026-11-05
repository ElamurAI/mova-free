# world-qa-world-selection

## forward

Queries select IDs, sentence/version state and event roles from a supplied World. one chooses the first resolved ID, not all ambiguous entities. Holder requires one or more times before selection. events accepts omitted bounds as 1/MAX, rejects explicit zero/malformed/overflow/reversed bounds, then selects inclusive sentence membership and role/verb intersections. where/holder @first use first location history, unlike end-of-sentence observation.

## invariant

Empty holder cannot become a successful empty answer or block a valid fallback. Invalid explicit interval cannot silently widen selection. Valid bounds beyond the document remain accepted and simply filter. Event provenance is rendered with anchors; this does not certify truth.

## cost

E events and R requested roles with candidate IDs D: O(E R D) role checks plus rendering. Histories and snapshots cost source-dependent scans. Interval validation O(token bytes); new guards O(1). No performance gain claimed.

## composition

Public joins: World::apply on constructed Cmd/Line values -> answer -> verdict; read_questions -> answers_md on supplied frozen expectations. No neural parser, teacher, corpus, CLI persistence or courses executed.

## behavior-complement

Domain/universe: terminating calls on consistent supplied worlds and well-formed observations in this scope. Inner absent-behavior relation excludes transitions violating the invariant above. This is a source-derived partial exclusion; unknown paths and malformed world states remain outside it. It is not a complement of merely sampled tests.

## reverse-reconstruction

A selected ID constrains the first resolving candidate but leaves later candidates unknown. Empty interval result can reflect no event, no role match or time exclusion. The answer string cannot reconstruct full state, history or original question.

## projection-boundary

Six finite valid bounds against a one-event world use direct interval membership as oracle; four invalid bounds reject. apply -> answer executes actual pure APIs. Other history/relationship paths are source-inspected only; sentence count casts and malformed public state remain unverified.

All three typed entries bind the manifest revision and scope. Operators are invariant exclusion, output preimage and observation constraint respectively; status is partial source modeling with finite native evidence. Reverse relations preserve all described ambiguity; no complete solver or universal inverse is provided.

## checks

cargo test -p world --lib qa::wave12_models -- --test-threads=1. See the file ../model.md#evidence for actual repeated profiles and compiled controls. Partial syntax projection leaves macro bodies opaque; no semantic AST proof.

## epoch-memory

Prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.

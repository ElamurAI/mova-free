# world-qa3-story-order

## forward

Event descriptions resolve across worlds, filter verbs/roles/anchor ranges, and sort by (sentence,line,ID). order uses TimeGraph ordering and explanatory paths, then independently compares telling (sentence,line), with explicit equal-coordinate wording. Anachronies require real branches, shared world, strict endpoint relation and qualifying own announcement; rank by descending hit count then ID.

## invariant

Telling ties must not assert either event told earlier. Event-ID tie breaking in selection is deterministic but does not establish chronological priority. Empty sort dispatch is rejected before indexing; valid sorting still uses predecessor count and sentence tie key.

## cost

Event selection O(E log E) plus role checks. Anachrony pairs O(E²) after graph closure; closure cost belongs to story. Sort O(N² log N) comparator-key recomputation plus closure; paths and output add their own work. New tie/empty checks are constant cost.

## composition

Public joins: World::apply on constructed Cmd/Line values -> answer -> verdict; read_questions -> answers_md on supplied frozen expectations. No neural parser, teacher, corpus, CLI persistence or courses executed.

## behavior-complement

Domain/universe: terminating calls on consistent supplied worlds and well-formed observations in this scope. Inner absent-behavior relation excludes transitions violating the invariant above. This is a source-derived partial exclusion; unknown paths and malformed world states remain outside it. It is not a complement of merely sampled tests.

## reverse-reconstruction

Text order answer admits many distinct time graphs yielding the same relation/path display. Equal telling coordinates can denote distinct simultaneous descriptions or the same event; they do not imply story-time equality.

## projection-boundary

Constructed v3 World::apply event stream joins graph ordering with public answer and report. Tie fixture changes only telling coordinates, preserving story BEFORE; a named assertion rejects invented earlier narration. Empty sort is an old-code panic witness and fallback regression. Complete malformed-story topology validation is not claimed.

All three typed entries bind the manifest revision and scope. Operators are invariant exclusion, output preimage and observation constraint respectively; status is partial source modeling with finite native evidence. Reverse relations preserve all described ambiguity; no complete solver or universal inverse is provided.

## checks

cargo test -p world --lib qa3::wave12_models -- --test-threads=1. See the file ../model.md#evidence for actual repeated profiles and compiled controls. Partial syntax projection leaves macro bodies opaque; no semantic AST proof.

## epoch-memory

Prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective constant-cost guards and tie comparison predicted rejection without changing valid selection or signatures. Four old-code witnesses failed: empty holder success, malformed interval broadening, empty sort panic and invented earlier narration for a tie. Narrow repairs retain ordinary substring scoring and existing historical explanation behavior. Future epistemic explanations and metadata consistency remain explicit limitations. No optimization or general percentage guarantee is inferred.

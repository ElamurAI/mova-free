# world-schooltools-parsing-marked-answer

## forward

Collect indices >=1 with starts_with('*'). If exactly one r, output question then field[r] stripped of all initial stars and following leading whitespace, then all other options in their original order. Zero or multiple marked options return a copy unchanged.

## invariant

Length preserved for every field vector, including empty input. Question at index0 is not considered a marker. Multiple-star prefix on one option counts as one marked option.

## cost

O(F+marker-prefix-bytes), O(F) vector storage; strings remain borrowed.

## composition

Parent parser supplies trimmed slices; canon itself accepts arbitrary slices and does not trim unmarked options. tally compares canonical right against wrongs; lint counts ambiguous marker options before canon; apply_edits canonicalizes both before/after for guards.

## behavior-complement

Exclude selecting one answer when two option entries start '*'; exclude reordering distractors among themselves; exclude treating starred question as an answer marker. With exactly one marked option, the original contiguous leading star run is removed before leading whitespace is trimmed. A star after that whitespace can become the first returned character: canon(["q", "* *a", "b"]) returns ["q", "*a", "b"]. Thus this relation does not exclude a returned answer starting with a newly exposed star.

## reverse-reconstruction

Given canonical output, antecedents include all positions of its marked right answer among the options with any positive initial star multiplicity, preserving wrong order. Unmarked and multiply marked vectors form unchanged fibers; whitespace removal admits many prefixes.

## marker_cardinality

Number of marked entries, not total stars, selects the branch: **a is one entry, *a and *b are two.

## permutation_stability

Unique-star branch is a stable removal/reinsertion at position1. Every wrong option keeps relative order and multiplicity.

## normal_form_limits

Canonicalization does not repair ambiguous markers or validate field width. Its second application can change a strange raw input only if stripping reveals another marker after whitespace; parser-normalized ordinary fixtures define the tested domain.

## length_observation

Moving a marked answer changes which Unicode-scalar length tally observes as right; answer-position variants with identical distractor order share tally.

## checks

Exact world library selections: schooltools::wave9_models::parser_canonical_fibers_and_tally. Current repeated debug/release outcomes and compiled controls are in the root sidecar ../../model.md#wave34-evidence. Only these supplied-input tests are current evidence; older parent module-wide receipts are historical. Macro bodies remain opaque.

## epoch-memory

Prospective model saved before edits. This epoch deepens one existing node rather than renaming it. Production implementation unchanged; tests add helper-boundary observations where stated. No live lessons, frozen suites, model loading, teacher or network. Read-only symbolic relations are broader than the finite native domain.

## History

### 2026-10-31 · document

Wave34, 2026-10-31. Source-bound helper model; no universal proof or approximation-error certificate. Historical parent epoch remains intact.

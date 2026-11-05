# tracking

## forward

Push on trimmed trailing question mark; otherwise newest congruent wins, complete removes only it, partial removes nothing. Implicit question uses given before extending it; generic first/next fallback never pushes.

## invariant

Exactly one Step per sentence. Stack is an increasing subsequence of pushed IDs. Partial/implicit leaves stack unchanged; complete removes only selected ID.

## cost

S sentences and Q open questions require up to S Q eligibility calls plus ordered-set updates and explanation construction. Add child eligibility cost and output size; track annotation cost remains external.

## composition

This fragment shares source and state with its siblings; root composition specifies ordering and latent dependencies. It is not an independent replacement API.

## epoch-memory

Predicted latest eligible and partial persistence; 46,656 bounded histories confirm. Generic-hedge prose overstates implementation: Who+maybe is complete. No production repair or continuous approximation metric proposed.

## checks

Five native tests: 46,656 six-step histories (279,936 transitions), uncertainty interaction, empty-content slot gate, four polarity preimages, ordered givenness. Fragment scopes are defined by source byte ranges; root evidence distinguishes executed helper tests from inspection-only branches.

## behavior-complement

Within the scoped input/output universe, violating recency or givenness is excluded by the source model. This inner constraint family does not claim a complete complement.

## reverse-reconstruction

Converse retains all inputs compatible with the scoped output and fixed dependencies; projections and discarded candidates generally create multiple antecedents. No single inverse is claimed.

## recency

Newest partial blocks an older complete candidate; maximum eligible ID is independent oracle on ordered IDs.

## givenness

Filter requested lemmas against given; classify remaining against answer, preserving request order, duplicates and case.

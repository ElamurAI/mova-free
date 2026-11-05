# mining-export

## forward

Merge dictionary evidence, accumulate OBJ marginals, retain last duplicate pair count/books, gate count/books with dictionary bypass but count>=3 always, classify by matrix and PMI. Aggregate two-word events and export sorted rows then final map.

## invariant

OBJ rows require count>=3 even with dictionary evidence. Particle rows require min only. Final map is pair-unique, detailed output need not be. Successful writes do not imply atomic publication.

## cost

For B bytes, U pairs, E event rows and R retained rows: O(B+(U+E)log(U+E)+R log R), plus matrix lookup and output bytes. Sorting is separate from accumulation; no timing claim.

## composition

This fragment shares source and state with its siblings; root composition specifies ordering and latent dependencies. It is not an independent replacement API.

## epoch-memory

Predicted first minimum dictionary rank and first MWE row; finite tests confirm. Limitations: substring flags, unconditional first-line discard, duplicate last pair versus summed marginals, absent phrasal book gate and unused k%1 set. Production unchanged.

## checks

Five native tests: 64 rank triples, eight phrase/POS contexts plus header, 80 MWE flag/type/title combinations, reversed order, four pair-key preimages. Full run/exports not executed. Fragment scopes are defined by source byte ranges; root evidence distinguishes executed helper tests from inspection-only branches.

## behavior-complement

Within the scoped input/output universe, violating dependency-feedback or export-collision is excluded by the source model. This inner constraint family does not claim a complete complement.

## reverse-reconstruction

Converse retains all inputs compatible with the scoped output and fixed dependencies; projections and discarded candidates generally create multiple antecedents. No single inverse is claimed.

## dependency-feedback

Matrix score consults existing global idiom table, clamps known literal OBJ scores, and can consult known layer. Freeze prior snapshot.

## export-collision

Particle rows use min only, no min_books; insertion overwrites same OBJ key in global map although detailed output can contain both. Two-file output is not atomic.

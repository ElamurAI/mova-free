# Derived index and focus restoration

## forward

On validated g and initialized s with false focus, fold frames in order i=0..N-1. Append i to entity occurrences, eligible verb, reverse next/cause and source-line indexes; overwrite sayer[q]=i; append mcode(f) then f. Return course/ledger from payload with reconstructed student.

## behavior-complement

Type: behavior_complement. Domain and observation are the forward relation above unless narrowed here. U is validated-payload/returned-state pairs; R is this deterministic fold conditional on mcode. An inner bound U minus R includes true restored lesson_focus, a retained ghost lookup absent from frames, a skipped qualifying verb occurrence, or an earlier quote owner surviving a later overwrite.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation are the forward relation above unless narrowed here. The output graph and persisted fields determine the accepted payload modulo canonical vector order; the original live cache contents and focus cannot be reconstructed. If only indexes are observed, many frame/text/weight states remain compatible. The relation preserves this ambiguity rather than treating indexes as an invertible encoding.

## occurrence-order

Type: ordered_fold_relation. Domain and observation are the forward relation above unless narrowed here. by_ent records occurrences from ents plus ent.and, including repeated IDs, in frame traversal order. It is not a set of unique frame numbers. by_verb excludes empty and LIGHT verbs; prev/effect reverse next/cause. Proposed fixtures with repeated IDs distinguish occurrence lists from deduplicated lists.

## quote-owner

Type: last_writer_relation. Domain and observation are the forward relation above unless narrowed here. For a target q, sayer[q]=max{i | q is in frame_i.quote} when the set is nonempty. Multiple mentions within one frame leave the same owner; later frames overwrite earlier ones. Reverse observation q->i requires occurrence at i and absence at later indices, with earlier occurrences arbitrary.

## cold-state

Type: state_projection_relation. Domain and observation are the forward relation above unless narrowed here. Successful load starts from default transient fields and sets lesson_focus=false; persisted weights, aliases and kind relations are restored explicitly. Codes are recomputed by mcode, not copied from prior live codes. Full parser cache equivalence is outside the observed contract; no lesson parsing is invoked in this loop.

## invariant

Cost follows total entity occurrences, links and frames with ordered-map updates plus mcode cost; mcode internals are outside this fragment. Assuming constant mcode would be unjustified. This range is the reconstruction suffix of the checkpoint parent.

## checks

oracle_checkpoint_fiber_erases_stale_indexes_and_focus compares literal entity/verb/prev/effect maps and false focus across two live states, then reloads. Roundtrip covers selected persistent fields and empty caches. Duplicate entity incidence and duplicate quote ownership remain proposed tests.

Shared evidence is in ../../model.md#tests. Referenced older mutation controls are historical evidence, not rerun here. General semantic correspondence and unsampled behavior remain unknown.

## epoch-memory

Decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

## History

### 2026-10-29 · epoch-memory

2026-10-29: decomposition preserves the twenty file-level models and makes this observation boundary explicit. The prospective expectation is that all twelve existing focused CPU retention tests remain green because Rust is unchanged. Executed results are recorded in the root ../../model.md#tests after checking. AST is not materialized for this range; no syntax selection, expanded AST, inverse solver or automatic combiner is supplied.

### 2026-10-30 · document

#### audit-epoch-2026-10-30

The current source binding includes four appended synthetic oracles. Production
byte ranges remain unchanged. The [checkpoint audit](../../model.md#audit) refines the
behavior-complement and preimage models with resealed first-error witnesses,
provenance/sentinel cases, ordered occurrence and quote-owner observations, and
serializer-versus-validator trust boundaries. The earlier proposed quote and
provenance checks now have finite evidence; exhaustive gates, semantic code
equivalence and successful cold examination remain gaps. Current whole-file
partial syntax is materialized in the root sidecar; this fragment retains its
reviewed byte-range scope without claiming a semantic or expanded AST.

### Undated · document

Subject: retention.rs, SHA-256 89715719216c1a94f9e701e24d4efea0b962bf04f5178d4916d19dc54536fb34. The manifest records the exact source interval. These source-derived relations are partial specifications; their evidence is bounded.

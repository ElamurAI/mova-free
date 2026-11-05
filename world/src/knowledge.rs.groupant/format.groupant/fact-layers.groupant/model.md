# Ordered English and idea fact projection

Subject: `world/src/knowledge.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

Find the first exact English delimiter and first exact idea delimiter. If both exist and English offset < idea offset, return two trimmed slices; otherwise return a one-element vector from strip_citations. Layer success bypasses citation stripping.

## invariant

Result cardinality is exactly two in the ordered-marker branch and one otherwise; the two strings may be empty. Original preamble is erased only in the layered branch.

## cost

O(B) delimiter search and copied output, plus citation-scan cost only on fallback; O(B) retained strings.

## behavior-complement

Type: behavior_complement. For one-call string/vector transitions, exclude singleton output when both first delimiters appear in the required order, or stripping '(Genesis 1:2)' inside the selected English slice in that branch. Exclude a two-item result on missing/reversed first markers.

## reverse-reconstruction

Type: reverse_reconstruction. For observed pair [e,i], compatible strings have arbitrary preamble then the exact English marker then whitespace-wrapped e then the first idea marker and whitespace-wrapped i, subject to first-marker constraints. For singleton [s], candidates must fail ordered-marker selection and map to s under citation scanning.

## marker-order

Type: ordered_guard. Both first offsets matter: a prior idea delimiter blocks layered extraction even if a later idea delimiter follows English. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## cardinality

Type: output_shape. One versus two items identifies the chosen branch, not whether statements are semantically present; empty slices are legal. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## citation-bypass

Type: dependency_selection. Layered text retains citations verbatim after trimming; fallback invokes citation scanning. The two routes cannot be composed as strip-then-split. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## preamble-erasure

Type: projection_fiber. Layered outputs do not retain Hebrew or word-for-word preamble; arbitrary compatible preambles yield the same pair without changing teaching facts. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

The field fold extends facts with the returned vector before processing the next line. Ordered markers choose the two-slice route before the citation fallback. Child costs overlap format's helper cost.

## checks

`knowledge::wave34_tests::layered_fact_projection_has_ordered_marker_precedence` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

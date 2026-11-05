# world-mwe-cupt

CUpt accepted rows, annotation grouping and token-ID projection.

## forward

Retain rows with >=11 tab columns and positive ASCII-decimal usize IDs; range, decimal, zero, empty and overflowing IDs are ignored. Preserve retained row order. Map each unique numeric ID to row index+1; duplicate numeric IDs map to zero. Parse heads through that map, absent/unparsable heads to zero. Lowercase lemmas, preserve forms/POS/relations. Annotation groups use string-key BTreeMap order; later nonempty category wins.

## invariant

Output vectors have identical lengths. Each nonzero head names a retained row with an unambiguous source ID. Gold indices name retained positions; duplicate annotation labels may duplicate indices, and self-heads can remain in Sent but predict ignores them.

## cost

O(B+N log N+A log G) plus string copying, where A counts annotation parts. O(B+N+A) storage includes read_to_string and grouped output.

## composition

Filtering must precede head remapping. Otherwise a sparse raw head can index an unrelated word or panic. Annotation indexing shares the retained order, not raw numeric IDs. Missing head maps to root-like zero, deliberately losing distinction.

## epoch-memory

First lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.

## checks

Native filter mwe::wave10_models; see ../model.md#evidence for fragments or ../model.md#evidence at the root. Finite supplied fixtures and independent oracles; no live parser, corpus, teacher or global dictionary activation. Partial syntax has opaque macro bodies and no semantic AST. No executable percentage surrogate or speed claim.

## behavior-complement

Universe: CUpt bytes and parsed outputs after successful I/O. Inner absent relation excludes nonzero attachments to missing or duplicated IDs and retained range/empty-node rows. This is tolerant projection, not a complete CUpt schema validator; comments must begin with # and annotation syntax is permissive.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## reverse-reconstruction

Given Sent retain all raw files producing its retained rows, including comments, ignored range/decimal rows, capitalization erased in lemmas and renumberings preserving head identity. Zero heads admit root, missing, duplicate and unparsable antecedents; they cannot identify which cause occurred.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## head-coordinate

For retained row j and unique source ID d, map(d)=j+1. For every row's raw head d this mapping replaces the old raw ID. The native sparse fixture [10,20,30] with heads [0,10,99] must become [0,1,0]; ignored range/decimal rows cannot shift the mapping.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## gold-order

Group IDs are strings, so BTreeMap sorts lexically, not numerically (10 before 2). This gold order feeds greedy judge. Categories are metadata only for matching; repeated category assignments retain the last nonempty value. No canonical semantic grouping or category agreement is imposed.

Subject: current source hash and this node's declared scope. Relation describes one terminating call over the finite inputs stated above, with fixed dependencies and successful allocation. Status: partial source-derived model; named native fixtures support only their finite domain. Unobserved effects remain unknown.

## History

### 2026-10-30 · epoch-memory

2026-10-30 first lexical epoch: old native witnesses returned raw heads [0,10,99], accepted noun-only overlap and duplicate token IDs, and panicked on an out-of-range head. Narrow fixes translate unambiguous IDs, omit invalid/missing arcs, and enforce the documented distinct/shared-verb condition. Prediction-order greedy matching remains intentionally unchanged; historical tuning scores can differ. No corpus retuning or measured speed claim.

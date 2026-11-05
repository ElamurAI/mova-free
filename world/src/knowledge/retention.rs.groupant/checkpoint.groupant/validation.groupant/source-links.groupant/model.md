# Source uniqueness, ledger membership and global edge bounds

Subject: `world/src/knowledge/retention.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

Collect source vector into ordered texts map; reject duplicate keys by length. Require every text lesson key in ledger. Traverse frames in vector order, requiring exact source key, then every quote/cause/next target < frame count. Sentence-local Frame::link is not traversed here.

## invariant

Every accepted frame has a text-backed source and every checked global link is in bounds. All text keys have ledger entries; ledger strings need not be valid digests or match content.

## cost

For T texts, L ledger keys, F frames and E checked links: O(T log T + T log L + F log T + E), with string/key comparisons explicit and O(T) map storage; fail-fast prefixes can be cheaper.

## behavior-complement

Type: behavior_complement. For inputs that passed the envelope prefix, exclude continuation with duplicate source keys, absent ledger key, absent frame source, or any checked target >=F. Do not exclude self-edges, cycles or frame.link values; those lie outside these predicates.

## reverse-reconstruction

Type: reverse_reconstruction. A returned frame-provenance error admits all earlier-valid text/ledger maps and earlier-valid frames, but a missing exact source for the failing frame. Link errors admit source-valid failing frames and the first invalid traversed target; same message joins many frames and edge kinds. Success leaves source text meanings unconstrained.

## duplicate-before-ledger

Type: failure_order. Map length comparison precedes ledger membership, so duplicate+missing-ledger candidates report duplicate source line. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## exact-source

Type: provenance_relation. Ledger membership of lesson n is insufficient for frame (n,line); that exact pair must occur in texts. Frame sentinel has no special exemption. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## edge-scope

Type: topology_boundary. Range checking quote/cause/next does not require DAG structure or unique quote ownership. Self/cyclic edges are accepted when sources and bounds hold. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## read-only-gates

Type: effect_projection. No file write occurs on rejection; maps and decoded frames are local and no partially validated Retained is returned. Temporary allocation is not durable publication. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

Envelope gates precede source collection; source uniqueness precedes ledger membership, then each frame source precedes its quote/cause/next bounds. Only after the whole loop does kind validation initialize/validate its temporary student.

## checks

`knowledge::retention::tests::oracle_resealed_rejections_partition_validation_gates` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

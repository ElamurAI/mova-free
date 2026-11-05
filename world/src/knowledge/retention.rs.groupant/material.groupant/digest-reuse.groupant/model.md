# Least compatible teaching-digest witness

Subject: `world/src/knowledge/retention.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

Reject foreign course before scanning. Iterate retained ledger in ascending key order; serialize/hash (course,key,explain,facts,exercises) for each key and return the first equality. Exhaustion returns None. This is a read-only lookup, not learning or reservation.

## invariant

Some(n) names an existing matching ledger key with no earlier match. Placeholder lesson number and assessment fields are not digest inputs; exercise labels and option order are included.

## cost

For m examined entries and teaching serialization bytes B, O(m*B) serialization/hash and O(B) temporary bytes; early exit cuts m. No memoization or optimization claimed.

## behavior-complement

Type: behavior_complement. In state/lesson/result transitions, exclude successful cross-course lookup, Some(n) for a nonmatching or absent key, and a later key when an earlier match exists. Assessment-only edits cannot change matching membership with teaching fields fixed.

## reverse-reconstruction

Type: reverse_reconstruction. Some(n) requires matching course, digest equality at n, and inequality at every smaller key; later ledger rows are arbitrary. All placeholder n/title/file/test/exam values remain possible. None means no key matches. Hash equality does not uniquely recover teaching strings without additional assumptions.

## number-substitution

Type: identity_projection. Each stored key replaces the candidate placeholder number in the digest; otherwise identical teaching can be discovered at several retained IDs. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## least-witness

Type: ordered_selection. BTreeMap key order, not insertion order, resolves multiple matches. Deleting the least matching key reveals the next compatible antecedent. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## assessment-separation

Type: noninterference. Tests/exams, file/title/level/stars and placeholder number do not enter T_key. Exercises do, including gold labels; this is teaching/assessment separation rather than blanket label independence. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## query-only

Type: state_effect_relation. No student fields or ledger entries mutate during reuse. A returned number is evidence of hash membership, not a fresh-process retention-quality score. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

Material parent composes read-only reuse and separate frontier queries; callers choose reuse versus allocation. Course guard precedes per-key hashing, first match prevents later serialization; no train/read is called.

## checks

`knowledge::retention::tests::oracle_reuse_fiber_selects_smallest_of_multiple_antecedents` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

# Checked maximum successor without reservation

Subject: `world/src/knowledge/retention.rs` at manifest source SHA. Horizon: one terminating call or explicitly named load prefix on finite synthetic inputs, pinned dependencies and ordinary allocation/I/O. This is a source-derived partial model; finite tests do not establish universal correspondence.

## forward

Read last ledger key, default zero when empty, checked_add(1), then reject candidate equal to KINDS_SRC. Return the candidate otherwise. Overflow error precedes reserved-value error because no candidate exists after failed addition.

## invariant

Success is positive and greater than every stored key on the current usize domain; it is never KINDS_SRC. Existing holes stay unused and the ledger is unchanged.

## cost

O(log L) conservative ordered-tree maximum access plus constant checked arithmetic; O(1) auxiliary space. No scan of holes or digest strings.

## behavior-complement

Type: behavior_complement. Over ledger/result transitions, exclude a successful hole allocation <=max key, successful reserved sentinel, and successful wraparound on usize::MAX. With fixed keys, changing digest values cannot change result.

## reverse-reconstruction

Type: reverse_reconstruction. Successful k has antecedent ledgers with maximum k-1, or empty ledger when k=1, arbitrary lower keys and arbitrary values, provided k is nonreserved. Overflow requires maximum usize::MAX; reserved failure requires maximum KINDS_SRC-1. These fibers do not identify which lesson occupied any key.

## sparse-ledger

Type: frontier_relation. Keys {2,7} yield 8 despite vacant lower positions. Empty and {0} both yield 1, witnessing distinct compatible inputs. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## error-order

Type: checked_arithmetic. checked_add rejects max before the reservation predicate. Since KINDS_SRC==usize::MAX, its predecessor gives a separate reserved error. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## digest-erasure

Type: value_noninterference. Only the key set influences successor; malformed or changed digest strings do not participate in this query's guards. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## no-reservation

Type: concurrency_boundary. Repeated calls without ledger insertion return the same number. A caller must coordinate insertion; this helper offers no uniqueness across concurrent independent states. Domain and fixed-source assumptions are those above; this is a source-derived constraint, checked only by the bounded named oracle where applicable.

## parent-interaction

Material parent uses this independent read-only query when callers need a new number. Maximum access precedes checked addition and then sentinel gate; returning a number does not update ledger or reserve it.

## checks

`knowledge::retention::tests::oracle_allocation_frontier_ignores_holes_and_rejects_boundaries` directly executes this helper or its containing load/reuse path with literal expected results. Full source and selected test body reviewed; no implementation-copy oracle. Exact names, profile repetitions and mutation receipts are recorded in the wave34 evidence summary. These are finite synthetic witnesses; unobserved cases and opaque dependencies remain gaps.

## epoch-memory

Wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

## History

### 2026-10-31 · epoch-memory

2026-10-31 wave34: prospective relations saved before test additions. Production bytes preserved. This new finer scope retains its parent's earlier models and history. Shared partial syntax is generated twice and compared; no semantic AST or solver claimed. No teacher, corpus, model loading, successful examination, training or lesson replay is credited as persistent knowledge.

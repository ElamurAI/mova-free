# Sparse-table oracle checks

## forward

Every nonempty interval is compared with a direct iterator minimum. Tagged values compare only key while exposing id, supporting an independent leftmost-identity oracle. Boundary and owned-String cases test validity and clone behavior.
Harness maps the compiled implementation to Pass or an assertion failure.

## invariant

Expected results come from direct slices rather than optimized storage.
Tagged Eq and Ord both use key; observing id adds information unavailable from value equality.
Assertion context identifies interval endpoints or trace size/step.

## cost

All-interval direct minima cost O(n^3) per length and query work adds O(n^2). Tagged identity checks cost O(n^3) at n=9.
These harness bounds imply no production speed claim.

## behavior-complement

Type behavior_complement. U is pairs of compiled implementations and harness
verdicts for this fixed suite with successful execution; R assigns actual
verdicts. Target U minus R. Conditional inner bound excludes Pass when an
executed comparison is unequal and excludes Fail when every call terminates
normally and all assertions hold. Passing samples do not imply correctness.
The leftmost-to-rightmost tie mutant exhibits a forbidden Pass at tagged interval 0..3.

## reverse-reconstruction

Type reverse_reconstruction. Pre(v) contains all implementations whose finite
observations yield verdict v. Passing implementations may differ on untested
sizes, data and histories. Failure is not a unique diagnosis:
wrong id 2 instead of 1 may arise from construction or query tie selection.
The mutation experiment supplies one compatible antecedent. No complete
implementation enumeration is claimed.

## independent-oracle

Type direct_reference_relation.
Expected value is direct slice minimum; expected tagged index is the first index minimizing key. Sequential scans do not use log levels or overlapping blocks. Owned String checks add generic snapshot observations.
This relation is exact on the declared finite reference domain, with
implementation correspondence checked by assertions.

## operation-obligations

Type query_identity_frame.
Every valid nonempty range yields Some; empty, reversed and beyond-end ranges yield None. Ties preserve original identity despite block overlap. Replacing caller String data after construction must not alter the stored snapshot.
These fixed scenarios do not establish all-input completeness.

## bounded-coverage

Type bounded_coverage_cost.
All nonempty intervals are enumerated at n=0..67 on repeated signed arrays and at n=9 on tagged equal minima. Other cases include empty, singleton, full range, usize::MAX, i64 MIN/MAX and non-Copy String values.
This exhausts intervals on selected arrays, not values or all histories.
It is neither a statistical confidence estimate nor performance measurement.

## tests

Four tests pass twice within 62 library tests and one doctest.
rustfmt --check and Clippy --all-targets -- -D warnings pass.
A compiled scratch source replaces a.min(b) by a.max(b); exactly tests::equal_minima_return_leftmost_identity fails at 0..3 (id 2 != 1), exit 101.
No mutant is committed. This exercises compiled production behavior rather
than only corrupting reference state.

## descriptant

Compose direct reference and actual implementation, preserving agreements
and rejected source mutation as bounded evidence.
Value equality misses wrong equal-minimum identity; tagged observations close that gap.
Partial AST omits macro expansion and type resolution. Structural registration
does not prove mathematical correctness.

## History

### 2026-10-29 · document

Revision 1, 2026-10-29. Four deterministic tests, source-bound in manifest.

### 2026-10-29 · tests

2026-10-29: four tests pass twice within 62 library tests and one doctest.
rustfmt --check and Clippy --all-targets -- -D warnings pass.
A compiled scratch source replaces a.min(b) by a.max(b); exactly tests::equal_minima_return_leftmost_identity fails at 0..3 (id 2 != 1), exit 101.
No mutant is committed. This exercises compiled production behavior rather
than only corrupting reference state.

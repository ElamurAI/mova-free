# Fenwick oracle checks

## forward

direct folds i64 slices into i128. check_all compares every prefix and interval. A fixed generator updates a plain vector and the tree; boundary checks compare typed errors and cloned complete state.
Harness maps the compiled implementation to Pass or an assertion failure.

## invariant

Expected results come from direct slices rather than optimized storage.
Generated small updates stay in i64; full-width cases explicitly test overflow rejection and compare both vectors.
Assertion context identifies interval endpoints or trace size/step.

## cost

All-interval direct folds cost O(n^3) per length; tree queries add O(n^2 log(n+1)). The 2,000 update trace checks use O(Ln) direct folds.
These harness bounds imply no production speed claim.

## behavior-complement

Type behavior_complement. U is pairs of compiled implementations and harness
verdicts for this fixed suite with successful execution; R assigns actual
verdicts. Target U minus R. Conditional inner bound excludes Pass when an
executed comparison is unequal and excludes Fail when every call terminates
normally and all assertions hold. Passing samples do not imply correctness.
The subtraction-to-addition mutant exhibits a forbidden Pass at empty interval 1..1.

## reverse-reconstruction

Type reverse_reconstruction. Pre(v) contains all implementations whose finite
observations yield verdict v. Passing implementations may differ on untested
sizes, data and histories. Failure is not a unique diagnosis:
wrong empty-range sum -22 may arise from incorrect cancellation, bounds, or corrupted block state.
The mutation experiment supplies one compatible antecedent. No complete
implementation enumeration is claimed.

## independent-oracle

Type direct_reference_relation.
Expected interval result is sum(i128(v[j]), j=a..b). After accepted add(i,d), the plain vector changes only its ith point. No reference computation uses block layout or bit operations; snapshots cover rejected changes.
This relation is exact on the declared finite reference domain, with
implementation correspondence checked by assertions.

## operation-obligations

Type update_error_trace.
Each 500-step history at four sizes alternates updates and prefix/range observations. Invalid indices and overflowing points must return exact errors and preserve full state. Adding MAX, reversing it, then adding MIN covers valid full-width transitions.
These fixed scenarios do not establish all-input completeness.

## bounded-coverage

Type bounded_coverage_cost.
Intervals are enumerated at n=0..33 on deterministic signed arrays. Seed 0x5eed with a wrapping LCG generates 500 updates each at n=1,7,32,65, totaling 2,000. Final trace states also receive all-interval checks. Boundaries include MIN/MAX, usize::MAX, empty and reversed ranges.
This exhausts intervals on selected arrays, not values or all histories.
It is neither a statistical confidence estimate nor performance measurement.

## tests

Four tests pass twice within 62 library tests and one doctest.
rustfmt --check and Clippy --all-targets -- -D warnings pass.
A compiled scratch source replaces prefix subtraction by addition; exactly tests::every_interval_matches_direct_slice fails at 1..1 (-22 != 0), exit 101.
No mutant is committed. This exercises compiled production behavior rather
than only corrupting reference state.

## descriptant

Compose direct reference and actual implementation, preserving agreements
and rejected source mutation as bounded evidence.
Empty-range cancellation detects a prefix-combination bug missed by prefix-only checks.
Partial AST omits macro expansion and type resolution. Structural registration
does not prove mathematical correctness.

## History

### 2026-10-29 · document

Revision 1, 2026-10-29. Four deterministic tests, source-bound in manifest.

### 2026-10-29 · tests

2026-10-29: four tests pass twice within 62 library tests and one doctest.
rustfmt --check and Clippy --all-targets -- -D warnings pass.
A compiled scratch source replaces prefix subtraction by addition; exactly tests::every_interval_matches_direct_slice fails at 1..1 (-22 != 0), exit 101.
No mutant is committed. This exercises compiled production behavior rather
than only corrupting reference state.

# algorithm-wavelet-matrix-tests

## model

# Wavelet matrix finite checks

## domain
Compiled implementation; all ternary arrays of lengths 0..5 (364 arrays),
their valid index intervals and selected thresholds. Additional fixed cases
include MAX, 2^63, duplicates, snapshot mutation and invalid usize bounds.

## forward
Independently copy and sort each selected slice, compare every kth position
including the first missing position, and count predicates with iterator
filter. range_count uses direct lower<=v<upper rather than count subtraction.
Inspect prefix increment invariants. Each test returns pass or a Rust assertion
failure; the intentional k<=z production mutant is separately compiled.

## invariant
Oracle values derive from caller-owned Vec slices, never from matrix rows.
Expected interval counts remain independent of bit traversal and stable
partition details. Exhaustive enumeration covers 1+3+9+27+81+243 arrays.

## cost
Bounded 364 snapshots plus fixed cases; per snapshot O(n^2) ranges and
O(n log(n+1)) sort per range, selected constant threshold pairs and n kth
checks. Production row construction O(64n) per snapshot. This is finite
correctness evidence, not a benchmark or allocation stress test.

## behavior-complement
Type behavior_complement. In the universe of candidate implementation
behaviors and finite verdicts, exclude a passing verdict when any enumerated
observation differs from the sorted-slice/predicate oracle. Failures outside
enumerated cases are not excluded by a pass.

## reverse-reconstruction
Type reverse_reconstruction. A passing verdict admits every implementation
agreeing on the sampled observations, including incorrect implementations
elsewhere. A kth failure at [0,1], k=1 narrows candidates to those returning
the observed wrong value or branching incorrectly, but cannot uniquely
identify a source defect. Full logged assertion provides a concrete fiber.

## independent-oracle
Type direct_reference_relation. Sorting the actual selected slice and
filtering original elements uses neither zero-prefix rank nor bit routing.
Unsigned extrema distinguish numeric order from signed reinterpretation.

## boundaries
Type interval_boundary_coverage. Empty intervals count zero; reversed and
out-of-bounds intervals are absent. k at length and usize::MAX is absent.
Value-bound reversal differs from equal-bound zero count.

## discrimination
Type mutation_sensitivity. Replace k<z by k<=z in production. The named
kth_zero_branch_control at [0,1], k=1 must fail an assertion, not compilation.
Restored source bytes must equal the original before positive rerun.

## tests
Four tests pass twice in each profile; see model.md#evidence for hashes and logs.

## descriptant
Small exhaustive alphabets exercise duplicates and interior interval routing;
fixed high-bit witnesses cover behavior absent from ternary values. This
composition does not cover every large length or allocation failure. Partial
syntax is not a semantic proof. No useful numerical surrogate of a finite
test verdict is proposed; executable approximant remains an explicit gap.

## evidence

### Evidence

4 tests pass twice debug and twice release. Normalized named outcomes match within each profile. Compiled control kth_zero_branch_control: k<z changed to k<=z fails by named assertion (exit 101), then source is restored byte-for-byte and all focused tests pass. Syntax projections run twice and compare byte-for-byte.

## History

### 2026-10-30 · document

2026-10-30. Original fixtures with no external problem statements.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

External logs: $MOVA_DATA/runs/octopus-wave18-structures-2026-10-30/. Finite tests provide no universal proof or timing guarantee.

Source algorithms/src/wavelet_matrix/tests.rs: sha256 9711b619c5149f73a82d5ea6d622721ebb38e90c36775c19e7dc0d4a1055f23a

- wavelet_matrix-debug-1.log: sha256 53b85b3440b903f66f5307cfa8d7339a541ce890b8f608f9b1ed3cfb6c0ccaa9

- wavelet_matrix-debug-2.log: sha256 8291cc72e554a5aa78186d53861a09c77640cb453110e199571024e9c94cfc99

- wavelet_matrix-release-1.log: sha256 3a0556f4fbee33f3b8a0e80fd5c701f53f8c5d47ef335509feae080673095687

- wavelet_matrix-release-2.log: sha256 2af3c5a12f51752acdf9edeaa5f3d3dcce9e432556d910896397c69db2c8edce

- wavelet_matrix-mutant.log: sha256 8c4e9579811733c20ca39e237e048bf400cef4cfa495ee789fbb99a88cf9db13

- wavelet_matrix-restored.log: sha256 ef4f497ad91b5d661bd7bca2b612dde1a46e9ebb96bce66a8bac24db0cb9309f

- syntax.json: sha256 f93971385ced4f855cc7e9a1506c01426279edef5f12a0b2070ccb5ca00e07c1

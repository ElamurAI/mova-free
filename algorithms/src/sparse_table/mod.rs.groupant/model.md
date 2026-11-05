# Immutable sparse range minima

## forward

Construction clones n input values and stores rows of original indices.
Row zero is 0..n. Row k selects the smaller value from two row k-1 halves;
equal values choose the smaller original index. Row k contains n-2^k+1
entries. Empty input has no rows. query(a,b) returns None unless a<b<=n.
For a valid interval, k=floor(log2(b-a)) uses leading_zeros on a nonzero
usize. Compare row k at a and b-2^k and return a reference to the selected
owned value. len and is_empty report the original count.

## invariant

Each row entry denotes the leftmost minimum of its power-of-two interval.
All stored indices are in 0..n. For query length L, 2^k<=L<2^(k+1):
the first and last width-2^k blocks cover the interval and may overlap.
Idempotence of minimum makes overlap harmless; smallest-index tie selection
preserves identity. Subtractions follow validation; shifts have k<usize::BITS.
Construction doubles width with checked_mul, preventing overflow on huge
lengths. Extremely large zero-sized inputs can still exhaust index storage;
allocation failure/panic is not silently converted into None.

## cost

O(n log(n+1)) index storage and comparisons, n clones, O(n) stored values.
Query has O(1) index work and one comparison, with Ord cost accounted separately.
User-defined comparison or clone cost is not assumed constant in wall time.
No timing or allocation-success guarantee is claimed.

## behavior-complement

Type behavior_complement. U is lawful input snapshots, usize endpoint pairs,
and optional returned original indices/values. R is the forward selection
relation. Target U minus R. Inner excluded behavior includes Some on invalid
or empty ranges, None on valid ranges, an outside-range selected index,
nonminimal values, and selecting a later equal minimum. The source rule
specifies exact selection; bounded tests support correspondence, not proof
for arbitrary custom types or faulty Ord implementations.

## reverse-reconstruction

Type reverse_reconstruction. Given fixed valid [a,b) and observed minimum
value m, retain every snapshot with every in-range value >=m and at least
one equal to m; outside values are unconstrained. If the selected index j
is observed, additionally a<=j<b, v[j]=m, and earlier in-range values are
strictly greater than m. These are fibers over order equivalence, not unique
input recovery. None identifies the invalid-range class for known n, while
many endpoint pairs share it. Full original values are retained internally,
but query projection discards most of that information.

## overlap-cover

Type interval_cover. Holding a,b fixed, choose w as the greatest power of two
not exceeding b-a. [a,a+w) union [b-w,b) equals [a,b). The overlap has no
effect on an idempotent minimum. Looking up a second block at a+w would
violate the cover near non-power-of-two boundaries. The direct interval
oracle checks all endpoints for lengths through 67.

## tie-identity

Type stable_argmin. Selection orders candidates lexicographically by
(value under Ord, original index). For order-equivalent distinct tagged
values, id observation must equal the first minimum's index. This extra
identity condition is stronger than value equality and is checked using
a lawful Tagged order that ignores id. A max-index tie mutant is rejected.

## ownership

Type snapshot_frame. Construction clones each input value once; later caller
replacement of an input element cannot alter the stored snapshot. Query is
immutable and returns a reference tied to the table borrow. This contract
assumes Clone's usual independence for observable value data; interior
mutable shared payloads and unstable comparators require caller discipline.
The String test replaces an original element after construction and observes
the original owned minimum.

## tests

Four tests pass twice within 62 crate tests plus one doctest.
rustfmt and Clippy -D warnings pass. All intervals at n=0..67 are compared
with direct slice minima; separate tests cover invalid/extreme endpoints,
i64 extremes, tagged equal minima, and non-Copy String snapshots.
Compiled external tie mutant changes a.min(b) to a.max(b). Exactly the
filtered equal_minima_return_leftmost_identity test fails at range 0..3,
id 2 versus 1, exit 101. No mutant enters repository source.

## descriptant

Rows and query cover compose through the same stable argmin; the sibling
test leaf checks both value and identity observations independently.
Epoch observation: direct numeric minima cannot detect a wrong equal-value
representative, so tagged identity is a separate oracle. Generic Ord/Clone
assumptions and allocation limits remain explicit. AST is partial syntax;
no macro expansion or semantic verification is inferred from registration.

## History

### 2026-10-29 · document

Revision 1, 2026-10-29. Original implementation with source-bound partial AST.
Assume a lawful stable Ord implementation and Clone preserving comparison
and identity-relevant data. Allocation and user-defined Clone/Ord panics are
outside the Option query contract.

### 2026-10-29 · tests

2026-10-29: four tests pass twice within 62 crate tests plus one doctest.
rustfmt and Clippy -D warnings pass. All intervals at n=0..67 are compared
with direct slice minima; separate tests cover invalid/extreme endpoints,
i64 extremes, tagged equal minima, and non-Copy String snapshots.
Compiled external tie mutant changes a.min(b) to a.max(b). Exactly the
filtered equal_minima_return_leftmost_identity test fails at range 0..3,
id 2 versus 1, exit 101. No mutant enters repository source.

### 2026-10-30 · document

#### Executable numerical observation — 2026-10-30

The adjacent rust_approximant.rs adds a scalar value observation, classified
scalar_relative / interval_certificate / per_input_certificate at 1/2/5/10%.
Its compact state stores global minimum and length when global spread divided
by global maximum fits the cap. This bounds every query's relative minimum
loss by monotonicity; otherwise the original table supplies exact values.
Construction costs O(n) and storage O(1) when accepted, with O(1) query work.
Fallback adds an O(n) scan to this module's O(n log(n+1)) table construction.
The surrogate preserves range validity and zero minima, but does not supply
this module's original references, leftmost ties or any in-range witness.
Its models and evidence are in rust_approximant.rs.groupant; that scalar bound
does not establish an error bound for consumers that rely on witness identity.

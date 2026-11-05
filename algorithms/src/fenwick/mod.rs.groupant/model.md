# Fenwick exact dynamic sums

## forward

State (v,t) stores i64 points v and i128 blocks t. For zero-based i, block
t[i] sums v[i+1-lowbit(i+1)..i+1]. Construction initializes singleton blocks
and propagates each completed block to i|(i+1). add(i,d) first checks i<n,
then checked_add on v[i]; errors leave both vectors unchanged. Success changes
v[i] by d and adds d to every containing block reached by i|(i+1).
prefix_sum(e) accepts 0<=e<=n and sums disjoint ending blocks by clearing the
lowest set bit of e. range_sum(a,b) accepts a<=b<=n and subtracts two prefixes.
value returns a copied point or None. Error precedence is invalid index first.

## invariant

All current points remain i64; every block equals its stated point sum.
The compile-time assertion requires usize::BITS<=64. Any subset contains
at most 2^64-1 points, so its sum lies strictly above -2^127 and below 2^127.
Construction partial blocks and prefix accumulators are disjoint subset sums.
A changed block represents a valid post-update subset, so adding delta also
fits i128. Prefix subtraction equals a valid interval sum and fits i128.
For every loop index i<n, i<=usize::MAX-1, so i+1 does not overflow;
the update successor strictly increases, possibly to usize::MAX, then exits.
No unbounded history can overflow aggregates because each point remains bounded.

## cost

Construction O(n) arithmetic and two O(n) vectors. add and each query are
O(log(n+1)) machine-word operations, value O(1). No heap allocation in queries
or updates. These are symbolic bounds, not timing measurements.

## behavior-complement

Type behavior_complement. U consists of valid input states, typed API calls,
arbitrary typed returns and same-length output vectors. R is the forward
transition relation. Exact target U minus R; an inner bound excludes invalid
or overflowing additions that mutate any state, successful updates changing
another point, incorrect prefix/range sums, and an accepted reversed range.
This bound omits some wrong internal block layouts unless observed; it is
not the exact complement. Tests supply bounded correspondence evidence.

## reverse-reconstruction

Type reverse_reconstruction. Given only range sum q on fixed [a,b), the
antecedents are all i64 vectors with sum(v[a..b])=q, all outside points free,
and t determined by the block invariant. Empty range implies q=0; otherwise
the fiber can contain many vectors. Given a successful full post-state and
known update (i,d), predecessor is unique if v'[i]-d lies in i64: restore
that point and rebuild blocks. A failed update is identity on state but
different rejected deltas/indices can produce equivalent coarse observations.
These are declarative preimages; general reconstruction enumeration is absent.

## arithmetic

Type bounded_arithmetic. Hold n and vector bounds fixed. Sum range is
[n*(-2^63), n*(2^63-1)], contained in i128 for n<=2^64-1. Update eligibility
is exactly -2^63<=v[i]+d<=2^63-1, tested before any write. This policy rejects
point overflow even when a wider aggregate could represent it. The oracle
uses direct i128 slice sums; tests include two MAX and two MIN points and
both overflow signs. Universal bound is mathematical; finite tests alone
do not establish every machine behavior.

## mutation-frame

Type atomic_point_transition. For accepted (i,d), v'[j]=v[j]+d*[j=i];
t'[k]=t[k]+d exactly for blocks containing i. For a rejected add, (v',t')=(v,t).
Queries borrow immutably. Test snapshots compare all storage on invalid-index
and arithmetic errors; fixed traces independently update only a direct vector.
No rollback is needed because all fallible arithmetic checks precede mutation.

## prefix-decomposition

Type disjoint_block_partition. For e>0, repeatedly clear its lowest set bit;
the removed blocks partition [0,e) without overlap. Thus the sum equals the
direct slice fold; difference of prefixes cancels [0,a) for range [a,b).
The empty prefix/range is zero. This exact decomposition assumes the block
invariant; tests enumerate every interval on 34 deterministic arrays.

## tests

Four tests pass twice within 62 crate tests and one doctest;
rustfmt and Clippy with -D warnings pass. Sizes 0..33 cover every interval,
2,000 seeded updates cover n=1,7,32,65 with prefix/range checks, and boundary
tests cover empty, reversed, usize::MAX, i64 extremes and atomic errors.
External compiled mutant changes range prefix subtraction to addition.
Exactly the filtered every_interval_matches_direct_slice test fails at
range 1..1: -22 versus 0, exit 101. No mutant is committed.

## descriptant

The production leaf and sibling test leaf compose through the public API and
direct slice oracle. Block identities justify query wiring; checked point
updates preserve the global arithmetic bound for arbitrarily long histories.
Epoch observation: i128 storage alone would not bound unbounded updates;
retaining checked i64 points makes the arithmetic contract closed.
Partial syntax AST has opaque macros and no semantic resolution. Bounded
tests and structural model checks do not constitute a full proof.

## History

### 2026-10-29 · document

Revision 1, 2026-10-29. Original code; source hash in manifest. No unsafe code
or external exercise content. Allocation failure is outside returned errors.

### 2026-10-29 · tests

2026-10-29: four tests pass twice within 62 crate tests and one doctest;
rustfmt and Clippy with -D warnings pass. Sizes 0..33 cover every interval,
2,000 seeded updates cover n=1,7,32,65 with prefix/range checks, and boundary
tests cover empty, reversed, usize::MAX, i64 extremes and atomic errors.
External compiled mutant changes range prefix subtraction to addition.
Exactly the filtered every_interval_matches_direct_slice test fails at
range 1..1: -22 versus 0, exit 101. No mutant is committed.

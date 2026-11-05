# algorithm-coin-change

## descriptant

### coin_change implementation groupant

Original code; no external implementation was copied. The mathematical recurrence is standard dynamic programming. Raw Rust is authoritative outside this sidecar. Syntax is partial and omits expansion and semantic resolution.

Composition: source identity, forward recurrence, invariant and typed relations jointly constrain observations. behavior_complement excludes outcomes; reverse_reconstruction retains compatible antecedents. They do not supply a global inverse; leaf checks do not prove caller wiring.

## approximant

### Forward, invariant and cost

best[0]=0; best[a]=min over d<=a of 1+best[a-d], unreachable=None. Positive d makes ascending a a dependency order. Every finite previous<=a-d<a, so increment cannot overflow.

O((A+1)*(D.len()+1)) time and O(A+1) space, including validation and initialization.

## models

# Typed relations

Subject: mod.rs, source SHA-256 in manifest.json. Successful-result relations assume storage success.

## behavior_complement

Let S(D,A)={sum k_i : k_i natural, sum k_i*d_i=A}. Universe: input/result triples. For positive D and successful storage, Ok(Some(c)) is excluded when c != min S; Ok(None) is excluded when S is nonempty; all Some outcomes are excluded when S is empty. Any zero denomination excludes every Ok, even at A=0. Allocation errors are environmental, outside the numeric model.

## reverse_reconstruction

Observed Some(c) admits precisely positive D,A with a c-coin representation and no shorter representation, conditional on storage success. None admits all nonrepresentable amounts. These preimages are nonunique. The finite singleton case A=12,c=3 recovers d=4; no general inverse is claimed.

## permutation

Permuting D leaves feasible multiplicity sums and the optimum unchanged; original order cannot be recovered from count.

## duplicate_extension

Appending an existing positive denomination leaves the optimum unchanged because multiplicities were already unbounded. Splitting one coordinate among duplicate denominations preserves achievable counts.

## scale_equivariance

For positive q with representable scaled inputs, M(qD,qA)=M(D,A) on successful allocation. Feasible multiplicity vectors correspond bijectively; resource errors need not be invariant.

## denomination_extension

Appending a positive denomination enlarges the feasible set. Using infinity for None, the minimum cannot increase on successful calls; impossible sums may become possible.

## singleton_preimage

For D=[d], d>0, output is Some(A/d) iff d divides A, otherwise None. For A,c>0, singleton antecedent d=A/c exists iff c divides A. At A=c=0 every d>0 is compatible.

## resource_partition

ZeroDenomination precedes other failures. AmountOverflow follows iff A+1 or its Option<usize> byte size overflows usize or exceeds isize::MAX. Reservation refusal gives AllocationFailed. Successful counts never exceed A; allocation refusal was not injected.

## tests

### Checks and limits

Four tests passed in each of two debug and two optimized standalone rustc --edition 2024 --test runs. Single-thread outputs matched after removing timing from the footer. Strict clippy-driver --test -D warnings and rustfmt passed. Compiled copied production mutant (omit count increment) failed unchanged assertion checks. No mutation remains in repository source. Syntax extraction ran twice and matched bytes; source hashes are bound in manifest.json.

No allocator-refusal injection, benchmark, machine-domain proof or workspace integration is claimed. Parent integration owns exports and central registry wiring.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: 64 subsets of denominations 1..6 at amounts 0..30: 1,984 comparisons each against BFS and exhaustive multiplicity enumeration. Tests cover zero rejection, storage-size overflow, non-greedy optimum, unbounded reuse, permutation, duplicates, scaling, extension and singleton preimages. Four tests passed twice each in debug and optimized standalone builds, with equal normalized outcomes. Strict lint passed and compiled mutants failed real assertions. Source syntax was identical twice. Reviewer correction replaced a non-overflowing chain witness with [MAX,MAX,1,2], and expanded coin cost accounting to include initialization and validation. Initial smoke tests preceded this recorded evidence; no prospective speedup claim was made. Finite observations are not universal proof. Allocation refusal, huge actual slices and complete preimage execution remain gaps. Preserve these observations in later epochs.

### 2026-10-30 · tests

2026-10-30: 64 subsets of denominations 1..6 at amounts 0..30: 1,984 comparisons each against BFS and exhaustive multiplicity enumeration. Tests cover zero rejection, storage-size overflow, non-greedy optimum, unbounded reuse, permutation, duplicates, scaling, extension and singleton preimages.

# algorithm-sparse-table-rust-approximant

## descriptant

### Certified numerical range-minimum companion

Prospective prediction: one min/max scan and constant retained storage should
reduce construction cost for narrow inputs compared with the original sparse
table. All queries remain O(1). Broad inputs add a scan before exact table
construction, so fallback can regress. No unconditional speedup is claimed.
Build timing must include the certificate scan; query timing must explicitly
exclude or include construction for both implementations equally.

Let m and M be global extrema. Every valid range has minimum R in [m,M].
For R>0, (R-m)/R=1-m/R <= 1-m/M=(M-m)/M. Accept precisely when
100(M-m)<=pM; u128 products safely cover the entire u64 domain. If M=0,
every range is exactly zero. If m=0<M, all four profiles fall back. An empty
slice uses compact state (length=0, minimum=0), and has no valid range.
Failed certificates build the original SparseTable<u64> and return its exact
copied result. All state is owned; caller mutation cannot change observations.

The value may not occur inside the query range: [99,100] at P1 returns 99
for [1,2). Thus original reference identity, leftmost ties, selected index,
and an in-range witness are explicitly unsupported. Valid-range feasibility
and Option shape are unchanged; allocation/resource behavior is not identical
because compact construction avoids allocating the exact table.

Mathematical combiner: min/max scan establishes the global interval; the integer
predicate chooses compact scalar or exact table; range validation preserves the
same successful observation domain. Wiring tests cover both paths. No parent
composition bound, semantic AST, formal proof or production substitution follows.

Current observations: four focused tests pass twice; exact integer boundaries,
nonexact accepted results, fallback, zero and maximum values are covered.
A compiled 25%-certificate mutant is rejected. Benchmark evidence is supplied
by the shared runner; finite success is distinct from the interval derivation.

## models

# Typed models

## behavior_complement
Universe: finite u64 slices, four supported budgets, usize query endpoints and
Option<u64> results after successful construction. Exclude Some on invalid
ranges and None on valid ranges. For exact range minimum R, exclude estimates
v>R or 100(R-v)>pR; at R=0 exclude all nonzero v. Exclude fallback=true when
the global certificate passes and fallback=false when it fails. Exclude any
mutation of the input slice. This is a source-derived partial specification,
not an exhaustive runtime proof; resource failures lie outside numeric claims.

## reverse_reconstruction
For compact state (n,m), compatible inputs are length-n slices with global
minimum m and some maximum M satisfying 100(M-m)<=pM. Empty input is the
unique length-zero antecedent. For nonempty inputs many permutations and maxima
are compatible; the state does not recover input order or query witnesses.
For observed (query,v) alone, compatible compact inputs additionally require
valid endpoints and global minimum v; compatible fallback inputs fail the global
certificate and have query minimum v. Invalid-range None constrains endpoints,
not original values. Enumerating bounded finite slices gives finite preimages;
no unique reconstruction or unbounded inverse implementation is claimed.

## forward and invariant
A compact state has no exact table and retains original length and global
minimum. Fallback retains the exact table with identical length. Every valid
query returns a lower numerical estimate within the selected relative cap.
Bounds are integer checked; no floating-point rounding or u64 multiplication.
Length/is_empty are exact. No in-range/reference witness guarantee exists.

## cost
Scan O(n) time, O(1) auxiliary memory. Accepted state O(1) storage and query
O(1). Rejected state adds original O(n log(n+1)) build and storage, with O(1)
queries; fallback can regress. Cloning a compact state is O(1); cloning fallback
copies its exact table and has its original cost. Allocation failures and timing
are environmental, outside the analytical numerical guarantee.

## composition
The scan supplies [m,M], the gate certifies the whole interval, and validation
checks 0<=start<end<=n before either branch. The function 1-m/R is monotone
in R>0, so its largest loss is at M, even for ranges not containing m.
No downstream branch, witness consumer or relative-error composition is modeled.

## tests

### Checks and limits

Negative control: a separate compiled source mutation changed the certificate
percentage to 25. The accepted_nonexact_and_threshold_monotonicity test failed
at the required fallback assertion. Compilation succeeded; rejection is semantic.
The production source was not modified by the mutation.

Finite tests do not establish a universal guarantee. The interval inequality in
model.md#descriptant supplies the scoped analytical argument. The shared runner owns
build and query timing, fixture/check records and integration evidence. No
corpus, GPU or end-to-end gain is claimed by these focused tests.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30. ApproxMin::from_slice(values, budget) constructs an immutable
numerical model for every finite u64 slice. query(start,end) observes nonempty
half-open ranges; invalid, reversed and out-of-bounds ranges return None.
The supported caps are 1%, 2%, 5%, 10%; classification is scalar_relative /
interval_certificate / per_input_certificate.

### 2026-10-30 · tests

2026-10-30: four Rust tests passed twice each in debug and optimized builds using the real compiled
mova-algorithms SparseTable and mova-approx ErrorBudget. The exhaustive oracle
checks all 4,681 arrays of length 0..4 over [0,1,90,95,98,99,100,u64::MAX],
every supported cap, and all endpoints through length+1 against direct slice
minimum and original SparseTable. It checks Option shape, lower-bound loss,
exact fallback agreement, length, emptiness and usize::MAX invalid endpoints.
Additional tests cover empty/all-zero/constant-MAX/mixed-zero/extreme inputs,
exact thresholds and their next rejected integer, monotone certificate dispatch,
and genuinely nonexact compact answers whose value is outside the query range.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

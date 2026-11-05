# algorithm-knapsack-rust-approximant

## descriptant

### Certified knapsack Rust companion

Before implementation the prediction was O(n log n) time and O(n) storage on
certified inputs, versus O(nC) exact row work. Failed certificates add sorting
overhead to exact DP. This is a certified heuristic with fallback, not an FPTAS.

The lower value L is the greater of density greedy packing and the best feasible
single item, both including all zero-weight values. Cross products and tie
breaks are exact and deterministic. Only fitting items decrease remaining
capacity. Fractional prefix packing, with its fractional value rounded upward,
gives U. Thus L<=V<=U. Accept only U-L<=floor(pU/100).
This implies L>=(1-p/100)U>=(1-p/100)V. Otherwise call the exact reference.

The certificate computes floor(pU/100) by quotient/remainder decomposition,
avoiding multiplication overflow. On usize<=64 targets slice value totals and
cross products fit u128. No floating point or approximate capacity arithmetic.
The API returns a feasible objective, not selected indices. Large capacities
can succeed without exact row allocation: dimension-error equivalence is not
claimed. Ranking allocation can fail independently of exact row allocation.

Epoch observation: best-single-item fixes greedy tie behavior at usize::MAX.
A zero-weight baseline can admit nonoptimal output without artificial degradation.
Finite tests support the implementation; the inequality supplies its scoped
analytical argument. No Lean proof or semantic AST projection is present.

## models

# Mathematical models

## behavior_complement
Universe: finite item slices, capacity, supported budget and result on usize<=64.
For success exclude value greater than optimum V, upper_bound less than V,
and relative objective loss greater than budget (zero loss when V=0).
A passing certificate excludes used_exact_fallback=true; a reference result
excludes false. Allocation failure is environmental. This is a specification
complement; exhaustive runtime verification is not claimed.

## reverse_reconstruction
For observed (v,u,false), compatible inputs have greedy-or-singleton value v,
fractional rounded upper u, and a passing certificate. For (v,v,true), inputs
have optimum v and a failing certificate. Many multisets and permutations
are compatible. Enumerating finite lists and retaining matching relations
gives bounded preimages; no unbounded reconstruction implementation exists.

## invariant and composition
Each packing step retains nonnegative remaining capacity and distinct selected
indices of total weight capacity-remaining. Fractional density prefix is an LP
upper bound. The combiner joins feasible lower, fractional upper and integer
predicate, selecting certified output or exact reference. Tests exercise both
routes; constituent correctness alone would not establish the wiring.

## cost
Filter O(n), sort O(n log n), scans O(n), storage O(n). Rejected certificate
adds O(n(C+1)) time and O(C+1) memory. Small capacity and weak certificates
may regress. Typed errors have no numerical approximation guarantee.

## Current profiles and classification

Caps 1%, 2%, 5%, 10% use the same integer certificate and exact fallback.
Classification is objective_loss / relaxation_certificate /
per_input_certificate. Bound kind remains analytical: for p<=10,
L>=(1-p/100)U and OPT<=U imply L>=(1-p/100)OPT.
A per-input certificate establishes this premise; finite observed error does
not establish the inequality for unseen inputs.

## tests

### Evidence

Four optimized Rust unit tests pass twice with actual mova-algorithms and
mova-approx libraries. 1,100 deterministic instances (length 0..10, 100 each)
are compared with independently enumerated subsets at all four budgets;
each call is repeated and compared exactly. Loss uses independent integer
arithmetic, not the implementation certificate.

Checks cover empty input, zero optimum, zero-weight u64::MAX, usize::MAX
capacity, arbitrary u128 certificate boundaries, both dispatch routes, and
nonexact output 300 versus optimum 330 accepted for 15/25% only.
A compiled mutant replacing certificate loss by zero was rejected by three
tests before strengthening the independent loss oracle.

Focused timing: 256 equal items weight=value=1000, capacity=100000; rustc -O,
black_box, 3 exact and 3000 approximate calls per budget. Two runs give value
100000 and zero observed loss; checksums 300000 and 300000000 match.
Approximate ns/call by budget 1/5/15/25: 787/788/801/796 then 791/803/788/797.
Exact ns/call: 21483813/19209591/19126045/19167497 then
20012207/19343775/19323867/19230526.
This favorable workload is not representative performance; short calls and
CPU contention limit precision. Temporary standalone timing harness; integrated
harness is owned by parent. No universal runtime claim.

Run through integrated companion harness or rustc --test with extern
mova_algorithms and mova_approx libraries. Finite tests are not universal proof.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29. max_value(items, capacity, budget) returns Estimate with value,
upper_bound and used_exact_fallback. Budgets 1/2/5/10% bound relative objective
loss against the exact optimum V; define zero loss when V=0.

### 2026-10-29 · tests

Heading `### Evidence, 2026-10-29` stays in tests as `### Evidence`.

### 2026-10-30 · tests

#### Current cap epoch, 2026-10-30

The 1/5/15/25 measurements above are historical; the 15/25 profiles are retired.
Current profiles are 1%, 2%, 5%, 10%. Five optimized standalone Rust tests
pass twice against the updated shared contract and unchanged exact algorithm.
The 1,100 fixed-seed instances retain independently enumerated subset optima.
Certificate equivalence is exhaustively checked for 0<=L<=U<=1000 at every
budget, with exact acceptance/rejection boundaries also checked at u128::MAX.
Raw integer and fractional API budgets above 10% are rejected, including the
retired tiers and nonfinite fractions.

The revised nonexact fixture has free value 500, L=600, OPT=630, U=652.
Only 10% certifies it: certificate gap is 52/652 (~7.98%), true loss is
30/630 (~4.76%). Tighter profiles return exact 630. A separate L=60,U=64
fixture checks budget-dependent dispatch without requiring output error.
A compiled zero-loss certificate mutant fails all five tests.
Classification: objective_loss / relaxation_certificate / per_input_certificate.
No new timing or universal speedup is claimed.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

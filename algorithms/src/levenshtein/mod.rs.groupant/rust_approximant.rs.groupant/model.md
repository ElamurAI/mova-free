# algorithm-levenshtein-rust-approximant

## descriptant

### Executable byte Levenshtein approximant

Before measurement: a scan should beat quadratic DP for long strings whose
histogram lower bound and aligned upper bound agree closely. Equal histograms
with many shifted positions should fall back and can regress due to scan cost.
A large identical context should shrink the DP middle. This is a prediction,
not a measured throughput claim. The shared runner owns comparative timings.

Only byte unit-cost insertion/deletion/substitution is modeled. Unicode scalar
or grapheme distance, weighted edits, edit scripts and downstream task accuracy
are unsupported. Successful allocations are assumed on fallback; resource
failure is not covered by the scalar numerical guarantee. The certificate
needs two fixed 256-entry usize histograms and no heap allocation.

## approximant

### Forward approximant and mathematical combiner

Observation: scalar byte Levenshtein distance D, exact zero when D=0.
Trim common affixes, form histogram lower L and aligned-edit upper U, and
return U only after the safe integer certificate (U-L)*100<=p*L; otherwise
return production D on the trimmed middle. model.md#models derives each bound and
the trimming equality, specifies ordered behavior complement and reverse
reconstruction, and explains composition and arithmetic invariants.

Combiner premises are source-bound: lawful byte equality, p in {1,2,5,10},
finite valid slices, and successful fallback allocation. Composition preserves
L<=D<=U and output overestimate at most p percent. A successful exact fallback
collapses the interval. Parent consumers must separately analyze their own
observation metric; this scalar theorem is not a bound for branching or tasks.

## models

# Mathematical models

## behavior_complement

Universe: pairs of finite byte slices, a supported budget p and an Estimate
on successful completion. Write D for exact unit-cost edit distance. Exclude
distance<D, lower_bound>D, upper_bound<D, lower_bound>distance,
distance>upper_bound and distance-D>p*D/100; when D=0 exclude distance!=0.
Also exclude a true fallback flag on an accepted certificate, and a false flag
on a rejected certificate. For fallback exclude any nondegenerate interval.
These are model relations, not merely a list of negative tests.

## reverse_reconstruction

Given (v,l,u,false), compatible inputs are exactly pairs whose trimmed aligned
upper bound is u=v, symmetric histogram bound is l and certificate passes.
Given (v,v,v,true), compatible inputs have exact distance v and rejected
cheap bounds. Byte renaming and shared prefix/suffix insertion produce many
compatible antecedents. Enumerating bounded byte pairs and retaining these
relations gives finite preimages; no unique or unbounded reconstruction is
claimed. Successful allocation is an additional environmental premise.

## forward and trimming proof

For common initial byte c, first-edit recurrence gives
D(cx,cy)=min(D(x,y),1+D(x,cy),1+D(cx,y)).
By triangle inequality D(x,y)<=D(x,cy)+1 and <=D(cx,y)+1.
Thus matching initial bytes can be removed without changing distance.
Reverse both strings: reversing an edit sequence preserves its cost, so
D(reverse(x),reverse(y))=D(x,y), and the same argument removes a common suffix.
Repeatedly remove a maximal common prefix, then suffix of the remaining
slices. These segments never overlap, including when one slice becomes empty.

Let h_a,h_b be byte histograms of the trimmed middle; let
A=sum_c max(h_a(c)-h_b(c),0), B=sum_c max(h_b(c)-h_a(c),0).
Each unit edit can lower A by at most one, and likewise B; both must reach
zero at the target. Hence D>=max(A,B,abs(len(a)-len(b)))=L.
Using both directions makes the bound and returned interval symmetric.
The length-gap bound follows because one edit changes length by at most one.
Align the first min(n,m) positions, substitute each mismatch and insert/delete
the remaining length gap. This constructs an edit sequence of cost
U=aligned_mismatches+abs(n-m), proving D<=U.

Accept iff U-L<=floor(L*p/100). For L>0,
(U-D)/D <= (U-L)/L <= p/100. For L=0 this accepts only U=0, so
the true zero has exact output. Otherwise call the real production distance
on the trimmed middle and return its exact value as both interval endpoints.
The elementary derivation here is the mathematical source; no paper is
claimed for this particular composition.

## invariant, arithmetic and combiner

Histograms count input bytes exactly. Directional excess sums are bounded by
their own slice length. U<=max(n,m), so mismatch plus length gap cannot overflow.
The certificate evaluates floor(L*p/100) as
(L/100)*p+((L%100)*p)/100. Since p<=10, neither product nor sum overflows
usize. Subtraction is performed only after L<=U. No floats or larger caps.
The combiner joins the trimming equality, lower/upper proofs and integer
predicate, then dispatches to the certificate or exact production fallback.
Tests check actual wiring as well as individual premises.

## cost and scope

Accepted path O(n+m+256) time, O(256) stack storage. Fallback adds
O(n'*m'+1) time and O(min(n',m')+1) row storage for trimmed lengths n',m'.
Repeated/equal histograms may yield a weak bound. Small inputs can regress.
Classification: scalar_relative / interval_certificate / per_input_certificate.
Bound kind analytical with a scoped derivation; finite tests cannot establish
a universal theorem or a composition-wide or end-to-end accuracy claim.

## tests

### Evidence and reproducible checks

The independent full-matrix oracle checks all 127 binary strings of length
0 through 6 against all others: 16,129 ordered pairs, every supported budget.
It checks true-distance bounds, relative error, exact-zero behavior, repeated
determinism, symmetry, trimming equality and agreement with production.
Further cases include all 256 bytes, reversed bytes, empties, periodic shifts,
equal histograms, affixes and unequal lengths. Certificate tests cover every
lower bound 0..500 with gaps 0..60 at all caps plus usize::MAX and thresholds.

Nonexact fixture: [0;n]+[2,3,4] versus [1;n]+[3,4,2] has L=n, U=n+3, D=n+2.
At n=100 only P5/P10 certify; at n=300 all four certify a nonexact answer.
The equal-histogram ab/ba pair forces exact fallback at every cap.
Allocation failure and arbitrary resource exhaustion are not exercised.

Compiled mutation evidence and release repetition are recorded below.

Debug: 4/4 tests passed. Optimized: 4/4 passed twice with the same deterministic
assertions and named test results. Formatting check passed.
The compiled source mutation changed only the production dispatch condition
from if certified(lower, upper, budget) to if true. It compiled, then failed
3/4 tests: exhaustive relative error, adversarial relative error and budget
dispatch. The unmodified source was never overwritten. This negative control
demonstrates numerical and route checks can reject a broken certificate gate.
Actual benchmark timings are left to the shared runner; these are correctness
runs and do not support a speedup claim.

## History

### 2026-10-30 · descriptant

Modeling epoch 2026-10-30. This opt-in companion returns a scalar distance
estimate, an enclosing interval and a fallback flag. Production stays unchanged.
All four ErrorBudget caps (1%, 2%, 5%, 10%) use one deterministic certificate.
The method is an original composition and derivation of elementary edit-distance
bounds, not an attribution to a new published algorithm.

### 2026-10-30 · approximant

Retained epoch observation, 2026-10-30: histogram collisions can make L zero
even for large nonzero D. Exact fallback is therefore essential. The n=100
nonexact fixture changes dispatch between 2% and 5%, whereas n=300 certifies
at every cap. This illustrates that budget is a ceiling, not target error.
Prospective costs and unsupported observations are in model.md#descriptant.

### 2026-10-30 · tests

2026-10-30: four embedded tests passed in a standalone Rust test compilation
linking the real mova-approx and mova-algorithms libraries. The shared runner
can compile the same source as a module without copying production code.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

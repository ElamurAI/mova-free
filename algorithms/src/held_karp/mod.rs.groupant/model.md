# algorithm-held-karp

## descriptant

### held_karp: mod.rs

This is a new implementation epoch, not a measured optimization. No speedup or numerical surrogate is proposed: exact objectives remain exact. Partial source syntax has opaque macro bodies; semantic analysis and unrestricted correctness remain outside the structural model. Four compiled objective/oracle controls are retained as falsification evidence.

## approximant

### Forward, invariant and cost

Forward: subset recurrence D(S,j) minimizes a path from zero through exactly S ending at j; close with j->0. Return None if closure has no finite candidate. Invariant: every finite table entry denotes an actual simple path with that exact vertex set; parent records the attaining predecessor. Empty and singleton domains are API zero-cost conventions. Cost O(n^2*2^n) time, O(n*2^n) space, n<=18 before any exponential allocation. Arithmetic bound 18*u64::MAX fits u128; size cap bounds shifts and products. Held and Karp (1962) provide classical attribution; source and fixtures are original.

## models

# Typed models: held_karp mod.rs

## behavior_complement

Universe: admitted optional-u64 square graphs and candidate successful cycles. Excluded outputs have nonzero anchor, missing/repeated vertices, a missing selected arc for n>=2, or objective different from their edge sum. For n<=1 the specified zero-cost special case applies. These necessary exclusions form an inner absent-output model. Evidence: independent witness checks on exhaustive triangles.

## reverse_reconstruction

For returned tour (p,z), compatible graphs have every selected arc present with sum z and no cheaper Hamiltonian cycle, and select p under the procedural tie rule. Nonselected arcs and all diagonal arcs remain ambiguous; even the selected costs admit multiple decompositions. Ignoring tie constraints gives an outer fiber, not a complete reverse solver. None instead constrains every vertex permutation to contain an absent arc. Permutation enumeration checks forward membership.

## subset_recurrence

For S subset of vertices 1..n-1 and j in S, D(S,j) is minimum cost of a path 0 through exactly S ending at j, infinity when absent. D({j},j)=arc(0,j); otherwise min over k in S-j of D(S-j,k)+arc(k,j). Closure minimizes D(all,j)+arc(j,0). Ascending numeric masks always compute proper subsets first. Exhaustive directed triangles compare with factorial enumeration without sharing this recurrence.

## witness_identity

Parent[S,j] records the predecessor that attained the stored strict improvement. Reconstruction clears exactly the current bit and writes a vertex at each descending position; start/end remain zero. Every returned n>=2 witness has n+1 vertices and exactly the graph coordinates whose weights sum to its cost. The independent check sorts the first n vertices and sums windows; cap-boundary directed ring checks all 18 vertices.

## tie_selection

Ascending masks, end vertices and predecessor vertices retain the first equal-cost candidate; closure picks the smallest final vertex among equal costs. For the complete zero-cost graph on four vertices this returns [0,3,2,1,0]. This deterministic state-local rule does not imply globally lexicographically smallest forward order. Exact repeat equality and explicit tie fixture are evidence.

## diagonal_projection

Holding off-diagonal arcs fixed for n>=2, all changes to diagonal arcs leave result unchanged because the recurrence excludes repeated vertices. For n=1 both Some and None diagonals yield [0,0] with zero cost by API convention, not by following a graph loop. Empty input yields empty vertices. Both singleton diagonal states and directed small graphs are tested; a general metamorphic enumeration is not claimed.

## resource_partition

Order: n>18 before square-shape validation; admitted n<=1 uses small output allocation; larger n allocates (n-1)*2^(n-1) u128 cells and u8 parents, plus n+1 witness cells only when feasible. At n=18 this is 2228224 cells per table, about 36.1 MiB total payload. Size cap makes shifts/products representable even on 32-bit usize. Allocation failures are returned but not artificially injected. Shape/cap and n=18 ring are checked.

## wide_arithmetic

Any finite path/cycle sums at most 18 u64 edges, hence at most 18*(2^64-1), below u128::MAX. Infinity sentinel is skipped before addition and cannot collide with a finite optimum. Missing arcs differ from zero arcs. Four-vertex u64::MAX graph, zero arcs and disconnected small graphs distinguish these states. No floating-point approximation or saturated objective occurs.

## tests

Run cargo test -p mova-algorithms held_karp::tests. Three tests cover independent enumerations, objective/witness identities and shape/resource boundaries. Debug and release each repeat twice with identical sorted test outcomes. Strict all-targets Clippy and narrow rustfmt pass. Raw logs stay under external run key (see History); see model.md#validation for hashes. No cross-algorithm integration is executed in this leaf task; the root owns composition tests.

### Checks

## validation

### Validation

Six owned tests pass twice in debug and twice in release. Three per module have identical normalized test identities across all four runs. Four compiled implementation/test-oracle objective controls fail named equality assertions; exact source bytes restore and positive focused checks pass. Strict all-targets algorithm Clippy passes. Partial syntax projections repeat byte-identically. No timing or general proof claim.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original exact implementation and fixtures. Deterministic ties, witness identity, explicit admission and wide objective arithmetic define the API. The implementation exports its mathematical certificate/witness directly; finite exhaustive checks inspect those identities independently.

### 2026-10-30 · document

Subject: adjacent source SHA-256 in manifest; epoch 2026-10-30. Observation is the pure call result or finite test outcome. All entries are partial relational specifications supported by source inspection and the named finite checks; formulas are not automatically verified. Allocation behavior is conditional on successful reservation. No general inverse, semantic AST or approximation-error guarantee is claimed.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms held_karp::tests. Three tests cover independent enumerations, objective/witness identities and shape/resource boundaries. Debug and release each repeat twice with identical sorted test outcomes. Strict all-targets Clippy and narrow rustfmt pass. Raw logs stay under external run key octopus-wave11-optimization-2026-10-30; see model.md#validation for hashes. No cross-algorithm integration is executed in this leaf task; the root owns composition tests.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External run key: octopus-wave11-optimization-2026-10-30. SHA-256 evidence:

- `held_karp-debug-1.log`: `1165437eae75a3e1f2bcc0648c9fc4dab509f95532c2ae8afa0cbe97283e2dd6`

- `held_karp-debug-2.log`: `1165437eae75a3e1f2bcc0648c9fc4dab509f95532c2ae8afa0cbe97283e2dd6`

- `held_karp-release-1.log`: `de93d03627045f4af01d4e3cd7358a536d8c226460510bf96da5c03e433389e3`

- `held_karp-release-2.log`: `b997fde5f590e7f51d1c4d47967ce3d355219d2322141d39187f8f78fc06dfb0`

- `held_karp-identity.txt`: `e51a3f33e75ec80142c3bd07cf8346a07f725c2efdf37a1834c662b4048687cf`

- `held_karp-mod.rs-mutant.log`: `c42a1efbcb194963b2c981b4a50642ea0305223f962a090e9c29b43eefbc583a`

- `held_karp-mod.rs-mutant-restored.log`: `020f920240a73cf198bccaf07dda21be26470b086be80329f1400bad8a899f2a`

- `clippy.log`: `5f04f6b51d898d08f9077b5d6c8fedf70a17c77579347c18617e41b4292617fb`

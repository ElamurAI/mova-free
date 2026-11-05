# algorithm-bipartite-matching-tests

## descriptant

### Modeling

Prospective design is retained in model.md#prospective. Existing dijkstra and floyd_warshall interfaces informed adjacency and validation choices; no imported solutions. Adding duplicate occurrences preserves maximum cardinality and feasibility. Reordering adjacency preserves optimum size but can change chosen mates. A long alternating chain requires reassignment of existing mates; simple greedy free-neighbor selection does not satisfy the contract.

The mathematical combiner reads source, source-bound partial syntax, forward and typed obernants, invariants and evidence together. The complement is first and the converse is second; neither is interchangeable with negative tests. Future epochs must retain failed controls and adjust claims to measurements. Models do not prove source correctness or downstream wiring.

Implementation and tests are separate leaves; registry import and parent wiring belong to integration. Syntax omits semantic analysis and expanded macros. Exact algorithms have no approximate replacement profile.

## model

# bipartite_matching tests model

## Forward

Runs a bounded deterministic assertion suite for bipartite_matching; successful completion means precisely that all enumerated oracle and boundary assertions held for this compiled source. Six tests per algorithm. Exhaustive all simple graphs on L,R=0..3: 689 graphs, with independent enumeration of all edge subsets and pairwise distinct endpoints. Check maximum cardinality, length, membership, right uniqueness and repeat equality. Boundary fixtures include duplicate edges, empty sides, deterministic ties, invalid endpoints and a 1,024-left alternating chain. A compiled mutant disabling matched-left queue traversal must fail reassignment_duplicates_and_ties.

## Invariant

Each oracle input is fixed and bounded; expected results come from a structurally independent algorithm. Every asserted success is inspected for the relevant contract. All exhaustive counts concern the finite domain only.

## Cost

Enumerates sum(2^(L*R)) graphs for L,R=0..3; subset oracle O(2^E E^2) time and O(E+R) space per graph. Chain fixture has 1,024 left vertices.

## nonbehavior

Type: behavior_complement. Excluded test observations include a passing suite when an enumerated assertion differs, or claims of universal correctness from a finite pass. Untested inputs are outside the suite observation scope; mutation rejection establishes sensitivity only to its specific defect.

## compatible_implementations

Type: reverse_reconstruction. Given a passing test transcript, the compatible antecedents are every implementation agreeing on all enumerated assertions; behavior elsewhere remains unconstrained. A failed named assertion restricts the antecedent to implementations violating that concrete predicate, without unique bug localization.

## oracle_separation

Type: oracle_independence. The oracle enumerates edge subsets and checks pairwise endpoint uniqueness; it neither searches alternating paths nor reassigns mates. Shared endpoint representation and Rust runtime remain assumptions.

## explicit_test_universe

Universe U consists of pairs (candidate implementation, claimed Boolean completion outcome) for this fixed bounded assertion suite, under terminating ordinary execution. R contains exactly pairs whose claim equals conjunction of the suite assertions. The behavior complement is U minus R, including false pass and false failure claims. Behavior on untested inputs is unconstrained; finite acceptance is not universal correctness.

## certificate_soundness

Type: finite_duality_observation. For every one of 512 simple 3-by-3 graphs the returned matching is feasible, its residual search reaches no free right, and the derived vertex cover meets every edge with exactly the matching cardinality. These finite checks connect the duality formula to actual output, rather than assuming cardinality optimality from feasibility alone. Transpose and 4608 insertion observations constrain related calls.

## reconstruction_sensitivity

Type: compiled_fault_observation. Hold fixtures fixed and replace production free=left_mate[left] with free=None in an isolated compiled copy. residual_cover_certifies_maximal_cardinality must fail, demonstrating sensitivity to truncated reassignment. Exact-output reconstruction checks 512 graph outputs plus duplicate-edge antecedents, retains ambiguous fibers, and excludes duplicate right mates and an alternate maximum outside the deterministic fiber.

## evidence

### Evidence

Three tests per algorithm. Exhaustive all simple graphs on L,R=0..3: 689 graphs, with independent enumeration of all edge subsets and pairwise distinct endpoints. Check maximum cardinality, length, membership, right uniqueness and repeat equality. Boundary fixtures include duplicate edges, empty sides, deterministic ties, invalid endpoints and a 1,024-left alternating chain. A compiled mutant disabling matched-left queue traversal must fail reassignment_duplicates_and_ties.

Focused debug runs twice and release run require identical normalized per-test statuses; whole-crate debug tests and strict all-target Clippy are also checked. The compiled mutant is an external copy, with exactly one selected test failing (exit 101); production source remains intact. Source-bound syntax is generated twice and compared after formatting. These finite checks do not establish a universal proof, runtime improvement or application integration.

Commands: cargo test -p mova-algorithms --offline bipartite_matching::tests:: -- --test-threads=1; same with --release; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings; mova-modelcheck syntax REPO SOURCE OUTPUT. Temporary uncommitted library exports enable focused checks; integration owns exports and registry.

Observed result: all three focused tests passed in both debug runs and release, with identical normalized statuses. Both source projections were identical across generation runs. Strict Clippy passed. The named external mutant compiled and was rejected: 0 passed, 1 failed, 2 filtered out, exit 101. No universal or approximate-profile claim follows.

## prospective

### Prospective

Before implementation: validate every right endpoint, then augment once per left vertex in input order using FIFO search and adjacency order. Maintain inverse mate arrays and flip alternating paths. Predict O(E+L(L+R+E)) time and O(L+R) auxiliary space. Exhaustively compare all <=3 by <=3 simple graphs with subset enumeration of edge sets, verify edge membership and uniqueness. Duplicates, rectangular/empty sides, invalid endpoints, deterministic ties and a long alternating chain supplement. Compiled greedy-only mutant must fail.

Finite tests and syntax projections are evidence, not proofs. No approximate executable profile is proposed.

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch 2026-10-30` stays in descriptant as `### Modeling`.

#### Deep graph epoch 2026-10-30

Three additional tests connect the specialized mathematical relations to actual production results. Implementation/API/error behavior is unchanged. The new models distinguish exact deterministic outputs from coarser optimal-value observations and retain preimage ambiguity. Earlier evidence remains historical. No speed, approximation profile, general proof or production substitution is claimed.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

#### Deep graph evidence 2026-10-30

Source SHA-256: dda7039075888c3151ff369fe9260da0083d234cddc648f6c08482269f5b80db. Six focused tests pass in debug and release, each repeated twice with identical named outcomes (12 tests across the two algorithms). Strict all-target Clippy passes. Two isolated mutant files compile, and each named new test rejects its algorithm's mutant. Mutant log SHA-256 for this algorithm: 55a2659eea112be07fe262e8f7d9c9d8b5f8fccad6112679e4a8d9abff4163e5. Raw logs live under MOVA_DATA/runs/octopus-deep-graphs-2026-10-30. Syntax generation is repeated twice per source and compared byte-for-byte.

Bellman fixtures: 576 potential-transport pairs, one reachable-cycle transport pair, 84 distance-certificate memberships (one accepted), 75 compatible graph constructions and 25 negative-slack exclusions. Matching fixtures: 512 residual/cover certificates, 512 transpose comparisons, 4608 one-edge insertions, 512 exact-output reconstructions with duplicate variants; duplicate-right and alternate-maximum observations are excluded. These are finite correspondence checks, not unrestricted proofs.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

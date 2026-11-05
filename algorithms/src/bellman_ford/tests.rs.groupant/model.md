# algorithm-bellman-ford-tests

## descriptant

### Modeling

Prospective design is retained in model.md#prospective. Existing dijkstra and floyd_warshall interfaces informed adjacency and validation choices; no imported solutions. Adding duplicate identical edges or permuting adjacency leaves successful distances and reachable-cycle status unchanged; invalid-edge precedence may change under permutation. Zero-weight cycles remain valid. Unreachable negative cycles cannot affect the source result.

The mathematical combiner reads source, source-bound partial syntax, forward and typed obernants, invariants and evidence together. The complement is first and the converse is second; neither is interchangeable with negative tests. Future epochs must retain failed controls and adjust claims to measurements. Models do not prove source correctness or downstream wiring.

Implementation and tests are separate leaves; registry import and parent wiring belong to integration. Syntax omits semantic analysis and expanded macros. Exact algorithms have no approximate replacement profile.

## model

# bellman_ford tests model

## Forward

Runs a bounded deterministic assertion suite for bellman_ford; successful completion means precisely that all enumerated oracle and boundary assertions held for this compiled source. Six tests per algorithm. Signed exhaustive oracle covers n=1..3 with each ordered pair absent, -1 or +1: 19,767 graphs and 59,214 source queries. Independent Floyd intermediate-vertex closure supplies reference weights and negative-cycle reachability. Boundary tests include zero cycles, disconnected negative self-loop, parallel edges, i64 extremes and validation precedence. A compiled mutant disabling last-round cycle rejection must fail cycles_and_full_width_paths.

## Invariant

Each oracle input is fixed and bounded; expected results come from a structurally independent algorithm. Every asserted success is inspected for the relevant contract. All exhaustive counts concern the finite domain only.

## Cost

Enumerates sum(n*3^(n*n)) oracle queries for n=1..3, each O(n^3), with O(n^2) oracle space; fixed boundary fixtures.

## nonbehavior

Type: behavior_complement. Excluded test observations include a passing suite when an enumerated assertion differs, or claims of universal correctness from a finite pass. Untested inputs are outside the suite observation scope; mutation rejection establishes sensitivity only to its specific defect.

## compatible_implementations

Type: reverse_reconstruction. Given a passing test transcript, the compatible antecedents are every implementation agreeing on all enumerated assertions; behavior elsewhere remains unconstrained. A failed named assertion restricts the antecedent to implementations violating that concrete predicate, without unique bug localization.

## oracle_separation

Type: oracle_independence. Floyd closure uses allowed intermediate vertices, whereas production uses synchronous edge-count relaxation. Distinct recurrences reduce shared-defect risk; arithmetic and Rust runtime remain shared assumptions.

## explicit_test_universe

Universe U consists of pairs (candidate implementation, claimed Boolean completion outcome) for this fixed bounded assertion suite, under terminating ordinary execution. R contains exactly pairs whose claim equals conjunction of the suite assertions. The behavior complement is U minus R, including false pass and false failure claims. Behavior on untested inputs is unconstrained; finite acceptance is not universal correctness.

## certificate_partition

Type: finite_relation_partition. For the fixed four-vertex signed graph enumerate U={candidate vectors [0,a,b,z]:a=-3..2,b=-1..5,z in {None,0}} (84 observations). The independent inequality-plus-tight-reachability predicate partitions U into one accepted actual production result and 83 excluded candidates. Each candidate is compared with equality to production; the too-low vector witnesses why inequalities alone are insufficient.

## metamorphic_sensitivity

Type: compiled_fault_observation. Hold test fixtures fixed and replace production checked_add(weight) by checked_add(abs(weight)) in an isolated compiled copy. potential_reweighting_correspondence must fail; restored production passes. The observation is a Boolean distinction between these two implementations, not a claim that all negative-edge defects are detected. Integer potential and constructed-distance tests additionally cover ambiguous output fibers.

## evidence

### Evidence

Three tests per algorithm. Signed exhaustive oracle covers n=1..3 with each ordered pair absent, -1 or +1: 19,767 graphs and 59,214 source queries. Independent Floyd intermediate-vertex closure supplies reference weights and negative-cycle reachability. Boundary tests include zero cycles, disconnected negative self-loop, parallel edges, i64 extremes and validation precedence. A compiled mutant disabling last-round cycle rejection must fail cycles_and_full_width_paths.

Focused debug runs twice and release run require identical normalized per-test statuses; whole-crate debug tests and strict all-target Clippy are also checked. The compiled mutant is an external copy, with exactly one selected test failing (exit 101); production source remains intact. Source-bound syntax is generated twice and compared after formatting. These finite checks do not establish a universal proof, runtime improvement or application integration.

Commands: cargo test -p mova-algorithms --offline bellman_ford::tests:: -- --test-threads=1; same with --release; cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings; mova-modelcheck syntax REPO SOURCE OUTPUT. Temporary uncommitted library exports enable focused checks; integration owns exports and registry.

Observed result: all three focused tests passed in both debug runs and release, with identical normalized statuses. Both source projections were identical across generation runs. Strict Clippy passed. The named external mutant compiled and was rejected: 0 passed, 1 failed, 2 filtered out, exit 101. No universal or approximate-profile claim follows.

## prospective

### Prospective

Before implementation: validate source then every edge, including unreachable edges. Synchronous distances after k rounds minimize walks of at most k edges; improvement in round V means a reachable negative cycle. Signed i64 weights widen to checked i128. Predict O(V(V+E)) time and O(V) auxiliary space. Independent Floyd closure on every <=3 vertex graph with each ordered pair absent, weight -1 or +1; boundary fixtures cover zero cycles, disconnected negative cycles, parallel arcs and full-width paths. Compiled cycle-detection mutant must fail.

Finite tests and syntax projections are evidence, not proofs. No approximate executable profile is proposed.

## History

### 2026-10-30 · descriptant

Heading `### Modeling epoch 2026-10-30` stays in descriptant as `### Modeling`.

#### Deep graph epoch 2026-10-30

Three additional tests connect the specialized mathematical relations to actual production results. Implementation/API/error behavior is unchanged. The new models distinguish exact deterministic outputs from coarser optimal-value observations and retain preimage ambiguity. Earlier evidence remains historical. No speed, approximation profile, general proof or production substitution is claimed.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

#### Deep graph evidence 2026-10-30

Source SHA-256: 51f928c217bd441aa21c5d9b16e41a5be8077048cd4aed6fb2005a144c9d076d. Six focused tests pass in debug and release, each repeated twice with identical named outcomes (12 tests across the two algorithms). Strict all-target Clippy passes. Two isolated mutant files compile, and each named new test rejects its algorithm's mutant. Mutant log SHA-256 for this algorithm: 07438187ebd41ab50bb24304003cb1ee9c279387c1d9840fd63b15fe58da8776. Raw logs live under MOVA_DATA/runs/octopus-deep-graphs-2026-10-30. Syntax generation is repeated twice per source and compared byte-for-byte.

Bellman fixtures: 576 potential-transport pairs, one reachable-cycle transport pair, 84 distance-certificate memberships (one accepted), 75 compatible graph constructions and 25 negative-slack exclusions. Matching fixtures: 512 residual/cover certificates, 512 transpose comparisons, 4608 one-edge insertions, 512 exact-output reconstructions with duplicate variants; duplicate-right and alternate-maximum observations are excluded. These are finite correspondence checks, not unrestricted proofs.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.

### 2026-11-01 · compatible_implementations

On 2026-11-01 a compiled reverse-validation traversal passed all six tests. It selects the last invalid endpoint instead of the first, because the suite never presents two invalid endpoints together. On `[[(2,0),(3,0)],[]]` from source 0, the original returns endpoint 2 and that mutant returns 3. Thus this passing transcript admits both implementations, despite their different error observations. This is a concrete converse fiber witness, not an exhaustive classification of the fiber.

### 2026-11-01 · evidence

#### Validation-order transcript fiber — 2026-11-01

An isolated copy reversed both row and edge iteration only in endpoint validation. All six existing tests passed against that compiled mutant. A separate Rust probe compared the actual source with the mutant on two invalid endpoints in one row: the original returned `InvalidEdge { from: 0, to: 2, vertices: 2 }`, the mutant returned endpoint 3. The probe output was identical on two runs. The suite therefore does not determine validation order among multiple invalid edges; this observation does not claim that other changes escape detection. The production source and test source were unchanged. Probe, mutant construction and logs are retained in the cycle 24 external run.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

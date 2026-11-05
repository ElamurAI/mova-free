# algorithm-held-karp-tests

## descriptant

### held_karp: tests.rs

This is a new implementation epoch, not a measured optimization. No speedup or numerical surrogate is proposed: exact objectives remain exact. Partial source syntax has opaque macro bodies; semantic analysis and unrestricted correctness remain outside the structural model. Four compiled objective/oracle controls are retained as falsification evidence.

## approximant

### Forward, invariant and cost

Finite forward map: each deterministic fixture is sent to the exact implementation and an independently enumerated optimum; a run passes iff every asserted objective, witness, certificate and error agrees. Invariant: oracle selection state is restored after every recursive branch. Injection enumeration costs O(P(c,r)*r) including path work; tour enumeration costs O((n-1)!*n). Test generation and source syntax are finite and explicit; neither exact source parsing nor finite agreement proves unrestricted correctness.

## models

# Typed models: held_karp tests.rs

## behavior_complement

Universe: deterministic executions of this finite test source with linked implementation. A passing run excludes candidate outputs that violate the enumerated oracle equalities, witness checks or boundary assertions. This is an inner absent-behavior description over executed fixtures only, not a specification of all possible inputs. Assertion failure is observed, with compilation failure excluded from mutant evidence.

## reverse_reconstruction

Observation is pass/fail plus named assertion. The compatible antecedent set contains every implementation/oracle version producing that observation on these fixtures. Passing output loses nearly all input and execution detail; many wrong implementations may pass finite fixtures. Failure constrains the reached assertion but does not identify a unique defect. The retained compiled mutation is one witness in the failure fiber.

## independent_oracle

Hungarian tests enumerate injections recursively with a used-column array and compare optimum, then separately verify all primal/dual identities. Held-Karp tests enumerate permutations by swaps and sum actual optional arcs, independently of subset recurrence. Fixtures are original finite synthetic data. Oracle agreement is exact on these bounded domains, not a general proof.

## mutation_sensitivity

Production objective+1 and purpose-specific oracle optimum+1 changes compile and fail named equality assertions. Original bytes are restored before positive checks. The transformation is a deliberately false objective, not malformed syntax; evidence establishes sensitivity to this class only and cannot establish all semantic defects are detected.

## boundary_partition

Selected fixtures cover empty inputs, singleton conventions, malformed shapes, precedence of dimension caps, work limits, ties and full-width integer sums. Held-Karp executes the accepted 18-vertex cap on a directed ring; Hungarian rejects an ordinary 512-square fixture by work budget. No impossible allocation or allocator-exhaustion test is attempted.

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

- `held_karp-tests.rs-mutant.log`: `bf1a772db765042f4c2d08ac832c20242f825262a6322020d121d368559e48b0`

- `held_karp-tests.rs-mutant-restored.log`: `01f190eb1cbcb0e55a3b42522d82636fd23c57e3e568105197c32b90d6c92426`

- `clippy.log`: `5f04f6b51d898d08f9077b5d6c8fedf70a17c77579347c18617e41b4292617fb`

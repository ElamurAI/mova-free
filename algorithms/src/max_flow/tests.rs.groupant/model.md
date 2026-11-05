# algorithm-max-flow-tests

## descriptant

### max_flow tests.rs

Original std-only implementation and adjacent bounded checks. Classical concept: Edmonds-Karp shortest augmenting paths, Edmonds and Karp (1972). No source copied.

The prospective artifact preceded implementation. The runtime checks refine this mathematical description; no timing claim, universal proof, macro expansion or semantic AST is supplied. Executable approximation is intentionally absent: these are exact APIs, with no justified lossy metric or substitution budget.

## model

# max_flow tests.rs model

## Forward

Run named exhaustive, boundary and metamorphic tests; each successful test is conjunction of its explicit Rust assertions. The suite executes production methods directly and independent reference predicates.

## Invariant

Oracle fixtures stay finite and deterministic; successful output is checked rather than inferred from constructor success. Evidence separates production correctness from harness mutation sensitivity.

## Cost

Exhaustive domain 2^12 graphs times at most 2^4 cuts and E edges; bounded production calls and explicit numeric fixtures. No measured performance claim.

## nonbehavior

Type: behavior_complement. For fixed finite fixtures, U consists of candidate terminating implementations paired with claimed pass/fail outcomes. R pairs each with conjunction of actual assertions. U minus R contains false pass and false failure claims. Unexercised inputs are unconstrained; a passing suite is not universal correctness.

## compatible_implementations

Type: reverse_reconstruction. A passing transcript admits every implementation satisfying the finite assertions, including implementations wrong outside the observed domain. A failing assertion restricts antecedents to implementations violating that exact predicate, without uniquely locating the defect.

## oracle_independence

Type: oracle_independence. The value oracle enumerates all separating vertex cuts; it does not augment paths. Certificate checks use capacity bounds, incoming/outgoing sums and cut equality. Shared endpoint representation and Rust arithmetic remain assumptions.

## finite_domain

Type: bounded_correspondence. 4096 directed unit-capacity graphs on four vertices, 25 two-edge capacity combinations, 10 direct preimages, numeric maximum and malformed inputs constrain production outputs. They do not exhaust weighted multigraphs; the explicit cancellation and parallel fixtures cover selected additional behavior.

## fault_sensitivity

Type: compiled_fault_observation. Compile an isolated production mutation that replaces checked total accumulation with value=0. The exact residual_cancellation_and_preimage test must fail at its value-two assertion; successful compilation followed by assertion failure establishes executable sensitivity, not broad mutation coverage.

## evidence

### Evidence

Compile an isolated production mutation that replaces checked total accumulation with value=0. The exact residual_cancellation_and_preimage test must fail at its value-two assertion; successful compilation followed by assertion failure establishes executable sensitivity, not broad mutation coverage.

#### Measured outcome

All four focused tests passed in each of two debug and two release runs; named assertion transcripts were byte-identical after timing removal. Strict all-target Clippy with -D warnings and cargo fmt --check passed. Each source syntax projection was generated twice and was byte-identical. Each isolated production mutant compiled successfully and failed the named equality assertion.

Artifact SHA-256 bindings (external run label above):

## History

### 2026-10-30 · descriptant

2026-10-30 prospective epoch: O(V+E) memory. O(V+E+VE(V+E)) conservative time: Edmonds-Karp has O(VE) augmentations; full BFS and array initialization cost O(V+E) each. The additive term covers E=0. No runtime timing claim or capacity-dependent iteration count.

### 2026-10-30 · evidence

2026-10-30: planned and then measured in the dedicated graph-optimization run. See external run label octopus-wave5-graph-optimization-2026-10-30: prospective.md, debug/release twice normalized, strict-clippy.log, format.log, syntax twice and compiled production mutant logs. Four tests per algorithm. Bounded results are not universal verification.

- max_flow-debug-1.log: f142efe017fa72285eb1e5b9d7317978ca17f94fedc3173273076bba30cfe785

- max_flow-debug-2.log: 078399c2ec1e45bd6d39cb6add9e941a7347afe38fd405c53a49a2e2a999081e

- max_flow-release-1.log: b1150dabd69f362a686fa82f40c7510e321fcca1826e40647cd72b4a9ad848b3

- max_flow-release-2.log: 9c03e7af88a53b370d15febdcc3a59326f21f5f0283d4eaa59f9c5606024fbf6

- max_flow-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- max_flow-mutant-rejected.log: 0f04d8aff7a839221de2a2eee224c3e91ed4fb148836759654348be71a723338

- strict-clippy.log: fba0904fdec91250d0aafb0674967baaad449a27cd510117ea1fb9defea286f1

- format.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- prospective.md: c041f70a6de4c683ebc42555609c330a5d55d45d83086f796a86ba4562c996c0

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

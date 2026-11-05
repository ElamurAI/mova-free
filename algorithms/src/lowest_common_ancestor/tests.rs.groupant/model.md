# algorithm-lowest-common-ancestor-tests

## descriptant

### lowest_common_ancestor tests.rs

Original std-only implementation and adjacent bounded checks. Classical concept: binary lifting/doubling of parent functions. No source copied.

The prospective artifact preceded implementation. The runtime checks refine this mathematical description; no timing claim, universal proof, macro expansion or semantic AST is supplied. Executable approximation is intentionally absent: these are exact APIs, with no justified lossy metric or substitution budget.

## model

# lowest_common_ancestor tests.rs model

## Forward

Run named exhaustive, boundary and metamorphic tests; each successful test is conjunction of its explicit Rust assertions. The suite executes production methods directly and independent reference predicates.

## Invariant

Oracle fixtures stay finite and deterministic; successful output is checked rather than inferred from constructor success. Evidence separates production correctness from harness mutation sensitivity.

## Cost

Parent function enumeration sum(n=1..5)(n+1)^n; chain/intersection oracle polynomial in bounded n, plus O(100000 log100000) long-chain preprocessing. No timing claim.

## nonbehavior

Type: behavior_complement. For fixed finite fixtures, U consists of candidate terminating implementations paired with claimed pass/fail outcomes. R pairs each with conjunction of actual assertions. U minus R contains false pass and false failure claims. Unexercised inputs are unconstrained; a passing suite is not universal correctness.

## compatible_implementations

Type: reverse_reconstruction. A passing transcript admits every implementation satisfying the finite assertions, including implementations wrong outside the observed domain. A failing assertion restricts antecedents to implementations violating that exact predicate, without uniquely locating the defect.

## oracle_independence

Type: oracle_independence. The oracle follows parent chains and detects repeats with linear membership, then intersects ancestor vectors. It does not use depths, binary lifting or production state flags. All (n+1)^n parent functions for n=1..5 are inspected.

## finite_domain

Type: bounded_correspondence. All 8476 parent functions across sizes one through five, every query pair in valid forests, chains at binary boundaries through 100000 vertices, relabeling and leaf extension constrain production outputs. The finite evidence does not establish universal cycle rejection or lifting correctness.

## fault_sensitivity

Type: compiled_fault_observation. Compile an isolated production mutation that returns None instead of Some(a) for equalized endpoints. The exact long_chain_and_power_boundaries test must fail at an ancestor assertion; this is one fault observation, not complete defect detection.

## evidence

### Evidence

Compile an isolated production mutation that returns None instead of Some(a) for equalized endpoints. The exact long_chain_and_power_boundaries test must fail at an ancestor assertion; this is one fault observation, not complete defect detection.

#### Measured outcome

All four focused tests passed in each of two debug and two release runs; named assertion transcripts were byte-identical after timing removal. Strict all-target Clippy with -D warnings and cargo fmt --check passed. Each source syntax projection was generated twice and was byte-identical. Each isolated production mutant compiled successfully and failed the named equality assertion.

Artifact SHA-256 bindings (external run label above):

## History

### 2026-10-30 · descriptant

2026-10-30 prospective epoch: O(n log(n+1)) preprocessing time and space for n>0, O(1) for empty construction. Each parent enters a path once; doubling has floor(log2 n)+1 rows. Query O(log(n+1)) time and O(1) extra space. Iterative construction has no depth-dependent call stack.

### 2026-10-30 · evidence

2026-10-30: planned and then measured in the dedicated graph-optimization run. See external run label octopus-wave5-graph-optimization-2026-10-30: prospective.md, debug/release twice normalized, strict-clippy.log, format.log, syntax twice and compiled production mutant logs. Four tests per algorithm. Bounded results are not universal verification.

- lowest_common_ancestor-debug-1.log: 5c945b9297e9100c9b43377b08a16b22ceafa2e43b0dccdafab30542c2bf886c

- lowest_common_ancestor-debug-2.log: bb6effb2906730a8eff65568e1801c317771e873a28e942bf7b00b93dd78d8b9

- lowest_common_ancestor-release-1.log: 3e74e3fa97adc7a888143ed626bbf084981dc0cbbdb8ca7343268a517055b102

- lowest_common_ancestor-release-2.log: 9e4f7e8de01307112b4bfa0ef489e4712d9ecfc0492e8c542584e10e4dbbb04b

- lowest_common_ancestor-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- lowest_common_ancestor-mutant-rejected.log: e1f76b95dc1ddce023f9662e11486200f1d15864557fd1e9853fcd879b0f255d

- strict-clippy.log: fba0904fdec91250d0aafb0674967baaad449a27cd510117ea1fb9defea286f1

- format.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- prospective.md: c041f70a6de4c683ebc42555609c330a5d55d45d83086f796a86ba4562c996c0

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

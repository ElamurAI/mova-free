# algorithm-dag-shortest-paths-tests

## descriptant

### algorithm-dag-shortest-paths-tests descriptant

The combiner joins the authoritative source hash and partial syntax with forward path semantics, first behavior complement and second result-preimage relation, invariants, cost and executed finite evidence. Agreement preserves a bounded correspondence claim; it does not turn AST syntax into a proof of Rust behavior. The implementation/test boundary is explicit: witness paths certify distances and finite oracle passes constrain implementations only on observed inputs.

The mathematical mechanism is DAG shortest-path relaxation, Cormen et al., Introduction to Algorithms, 3rd edition, section 24.2. Endpoint projection deliberately composes with the existing deterministic topological_sort.

Retained observations: disconnected invalid input remains observable; distance vectors have many compatible graph antecedents; allocation failure and defensive overflow branches have no finite fixture evidence. Signed full-width weights require widened sums even though each individual edge is i64.

## model

# algorithm-dag-shortest-paths-tests

## Domain

Deterministic finite fixture executions and assertion verdict vectors for the adjacent implementation; not an all-input proof.

## Forward

Six tests: enumerate 4^6 four-vertex increasing-label DAGs with absent/-1/0/1 edges, every source and a fixed nontrivial relabeling, compare exhaustive path enumeration; then assert global ordered errors, i64 extrema widened to i128, duplicates and input preservation, potential transforms/nonunique fibers, and a 2048-vertex negative chain plus isolated vertex.

## Invariant

Oracle enumerations are independent of production scheduling and use immutable generated graphs. Assertions compare full vectors; all test loops are deterministic. Bounded costs and arithmetic follow fixture sizes; no randomness or external data.

## Cost

Finite enumeration 4^6 DAGs at 4 sources, path oracle exponential in general but n=4 here; other fixtures linear in 2048. Symbolic suite cost, no timing claim.

## nonbehavior

Type: behavior_complement.

In the universe of fixture/verdict pairs, excluded behavior contains a passing verdict when any encoded expected value differs, or a claim of universal correctness from these finite fixtures. The independent oracle enumerates paths rather than reproducing the production scheduling mechanism.

## verdict_preimage

Type: reverse_reconstruction.

Given the all-pass assertion vector, recover all candidate implementations agreeing with the fixture map on exactly the executed inputs. This set includes the reference and implementations that fail on untested inputs; it does not identify unique source code. A failing oracle assertion constrains the preimage to implementations disagreeing on that concrete graph/source/output.

## oracle_correspondence

Type: finite_relation.

Every path in an increasing-label DAG terminates. Enumeration covers 4,096 graph codes and 16,384 graph/source comparisons, plus relabeled comparisons; arbitrary labels test the production topological dependency.

## mutation_separation

Type: distinguishability_relation.

Replacing strict minimum comparison by maximum must disagree when two source paths have unequal sums; the exhaustive oracle assertion rejects this compiled mutation. Compilation success is required; compile failures do not count.

## coverage_boundary

Type: coverage_relation.

Fixtures sample graph semantics, source/endpoint priority, disconnected behavior and metamorphic identities. They do not exercise allocation failure or the defensive checked-distance overflow return. Repeatability means identical normalized named test outcomes across two debug and two release runs, not identical timings.

## Limits

Source-derived mathematical models with finite executable correspondence, not formal Rust proofs. Partial syntax omits macro expansion, type resolution and semantic AST. Exact production implementation; no approximate surrogate or numerical error guarantee is claimed. Resource exhaustion is outside the successful-call model.

## evidence

### Evidence

Every path in an increasing-label DAG terminates. Enumeration covers 4,096 graph codes and 16,384 graph/source comparisons, plus relabeled comparisons; arbitrary labels test the production topological dependency.

These observations establish finite correspondence only. No timing superiority, exhaustive machine-domain proof, approximation error budget or overflow-error test coverage is claimed.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: Six tests: enumerate 4^6 four-vertex increasing-label DAGs with absent/-1/0/1 edges, every source and a fixed nontrivial relabeling, compare exhaustive path enumeration; then assert global ordered errors, i64 extrema widened to i128, duplicates and input preservation, potential transforms/nonunique fibers, and a 2048-vertex negative chain plus isolated vertex.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 4f9b012a44d45e1eff35f5badbb1ea9b35956aa71c47d588ce6cba8d53a27142.

Six tests for this algorithm passed in two debug and two release full-crate runs (179 tests each); normalized named outcomes were identical. Strict all-target Clippy and formatting checks passed. Source syntax was projected twice and compared byte-for-byte. A compiled production mutant was rejected by the independent finite oracle assertion. External run bundle: MOVA_DATA/runs/octopus-wave4-graphs-2026-10-30 (debug1/2, release1/2, normalized outcomes, clippy, format, dag_shortest_paths-mutant and syntax logs; validate.rs records commands).

### 2026-10-30 · prospective

#### Symbolic expectations and scope

2026-10-30: Expected exact path semantics and Finite enumeration 4^6 DAGs at 4 sources, path oracle exponential in general but n=4 here; other fixtures linear in 2048. Symbolic suite cost, no timing claim. These are mathematical expectations, not calibrated performance forecasts. The initial implementation plan selected existing topological ordering to reject cycles globally before relaxation. Finite tests subsequently check the stated fixtures; no runtime benchmark or probabilistic approximation is proposed.

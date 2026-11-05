# algorithm-stoer-wagner

## descriptant

### algorithm-stoer-wagner descriptant

#### Design

Validate all row lengths then upper-triangle symmetry. Run maximum-adjacency phases over active supervertices, selecting greatest accumulated connection with smallest original survivor ID as tie breaker. The last selected supervertex defines a phase cut; compare weight then oriented sorted side with previous phase cuts. Merge its weights and original-vertex group into the penultimate vertex. Return the best phase cut.

The combiner joins authoritative Rust, source-bound partial syntax, forward relation, first behavior complement, second reverse reconstruction, invariants and finite evidence. Oracle agreement constrains only observed projections. A returned optimal witness alone is insufficient to reconstruct the deterministic tie trace; inverse models preserve this ambiguity.

Canonical literature: Stoer and Wagner, A Simple Min-Cut Algorithm (1997). Original dense implementation, tests and models; canonical phase theorem is cited, not copied source.

#### Prospective predictions

The design predicts exact objective agreement with independent bounded enumeration, valid witnesses, deterministic normalized debug/release outcomes and rejection of the specified compiled mutant. Symbolic cost predicts no wall-clock speedup; no timing benchmark is claimed. These design predictions are separate from the measured records in model.md#evidence.

#### Retained observations and open work

This is the first modeling epoch for this file. Exact source/output agreement on finite fixtures does not prove the phase theorem, SCC implementation, or all possible heuristic graphs. Resource exhaustion and semantic AST analysis remain outside this epoch. No numerical approximation budget is justified for categorical decisions or arbitrary witness changes.

## model

# algorithm-stoer-wagner

## Domain

Finite square symmetric u64 adjacency matrices, interpreted as undirected nonnegative capacities; diagonal entries ignored. Results are errors, None for fewer than two vertices, or u128 cut weight with sorted proper side containing vertex zero. Allocations succeed; input immutable.

## Forward

Validate all row lengths then upper-triangle symmetry. Run maximum-adjacency phases over active supervertices, selecting greatest accumulated connection with smallest original survivor ID as tie breaker. The last selected supervertex defines a phase cut; compare weight then oriented sorted side with previous phase cuts. Merge its weights and original-vertex group into the penultimate vertex. Return the best phase cut.

## Invariant

Active groups partition original vertices. Between-group matrix entries equal sums of original crossing capacities and remain symmetric. Phase connection[v] equals weight from v to the selected set. The final phase vertex's connection equals its cut weight. The maximum-adjacency phase theorem ensures one phase cut or a cut retained after contraction realizes the global minimum. All additions are checked u128; ignored diagonals never enter a connection sum.

## Cost

At most V-1 phases with O(V^2) scanning/connection work each, hence O(V^3) time. The dense matrix uses O(V^2) storage; groups and phase vectors use O(V). Sorting and complementing a phase side costs at most O(V log(V+1)) and is dominated by cubic work. No empirical speedup is claimed.

## nonbehavior

Type: behavior_complement.

In the declared input/result universe exclude acceptance of a nonsquare/asymmetric matrix, wrong first error, a trivial or unsorted cut side, a side excluding vertex zero, a weight unequal to original crossing capacity, a suboptimal objective, and wrapped sums. This is a source-derived semantic exclusion model rather than an enumerated negative-test set.

## cut_preimages

Type: reverse_reconstruction.

For a full Cut(w,S), candidate antecedents are square symmetric nonnegative matrices where S is sorted, proper, contains zero, has crossing sum w and no cut has lower weight. Those constraints form an outer reconstruction relation: exact antecedents additionally require the deterministic phase sequence and equal-phase tie rule to select S. Every diagonal is free, giving explicit nonunique full-output fibers. None has all valid matrices of size zero or one as preimages.

## contraction_quotient

Type: quotient_relation.

Given the active partition P, quotient capacity W[A,B]=sum of original capacities across A and B. Merging A and B replaces each external capacity by W[A,C]+W[B,C]. Every quotient cut lifts to a union of original groups with exactly equal weight; cuts separating A from B are intentionally lost and accounted for by the phase theorem.

## witness_lift

Type: certificate_relation.

The stored last-supervertex group is nonempty and proper at every phase. Sorting it and complementing when it excludes zero preserve cut weight in a symmetric matrix. Summing original rows across the returned side independently certifies feasibility and objective value, though minimum optimality needs exhaustive cuts or the phase theorem.

## capacity_order

Type: order_relation.

Increasing one or more undirected capacities cannot reduce global minimum cut value, since each candidate partition sum is nondecreasing. Positive integer scaling multiplies the minimum value and preserves all capacity-order comparisons when representable in the input type. Cut witness identity under arbitrary increases is not promised.

## vertex_permutation

Type: equivariance_relation.

Simultaneously permuting rows and columns preserves the minimum value and maps every feasible cut to an equal-weight cut. The chosen side need not follow the permutation because survivor tie order and zero orientation change; the API makes no global lexicographic minimum-side promise.

## loop_fibers

Type: kernel_relation.

Arbitrary diagonal changes leave every cut sum, phase connection, merge to an external group and returned result unchanged. This exact discarded-coordinate relation supplies recoverable crossing data without claiming recovery of loop capacities.

## validation_and_arithmetic

Type: error_partition.

All nonsquare-row errors precede asymmetry errors; first bad row or lexicographic upper-triangle pair determines the payload. u64 inputs are nonnegative by construction. Checked u128 merge/connection sums cannot wrap; finite tests cross u64::MAX but do not exercise actual u128 overflow or allocation failure.

## Limits

Partial source-derived relations with bounded executable evidence, not formal Rust proofs. Syntax omits macro expansion, name/type resolution and semantic analysis. No executable approximate surrogate is attached: categorical satisfiability, witness feasibility and error behavior have no justified numerical approximation metric here. Resource exhaustion remains outside scope.

## evidence

### Finite evidence

Enumerate 729 four-vertex ternary-capacity graphs and 1,024 five-vertex binary graphs (1,753 total). Independently enumerate all proper partitions containing zero and rescan original matrix crossings. Check minimum objective, sorted/proper/zero-oriented witness and witness weight. Other fixtures cover shape/symmetry errors, empty/singleton, disconnected and tied cuts, contraction witness, u64::MAX totals, diagonal fibers, scaling, monotonicity and relabeling.

#### Scope and retained corrections

Raw logs and the full hash receipt are in the external run root. The initial strict Clippy pass rejected index-based matrix loops; these were rewritten to iterator/split-borrow loops before the repeated tests. The first Stoer-Wagner mutation selector matched both a contraction and a connection sum and stopped without editing that source; its selector was narrowed to the phase connection sum before compiling the successful negative control. u64 overflow boundaries are exercised through u128 totals; actual u128 exhaustion and allocation failure are not exercised. There is no live training, GPU, benchmark or school-quality evidence.

## History

### 2026-10-30 · descriptant

Heading `#### Design epoch 2026-10-30` stays in descriptant as `#### Design`.

### 2026-10-30 · evidence

Date: 2026-10-30. Source SHA-256: `99388dc692a562a7febda63f40e0024b17818d7926caecc33647b756cd6ba7c8`.

Validation completed after restoring the compiled mutations. External run root: `$MOVA_DATA/runs/octopus-wave7-graphs-2026-10-30/`. No teacher, GPU or live API execution is involved.

#### Executed validation

All 269 library tests pass twice in debug and twice in release, including six tests for this algorithm (18 additions across the three modules). Normalized sorted outcomes agree in all four executions: SHA-256 `f9d87c0f117ccf985e0cc849c0a0ed47d85c1acf513a9db7c52c1f16edc01337`. Strict `cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings` and focused rustfmt pass. Temporary library exports were restored before commit. Source SHA-256 `99388dc692a562a7febda63f40e0024b17818d7926caecc33647b756cd6ba7c8`; twice-generated syntax SHA-256 `d3fe219709a72715df0546786cdeba7031d3d6d721816b50cd995e3e6422ef3b`.

The production mutation is defined in this algorithm's test model. It compiled, executed the selected test and failed an assertion (not a compiler failure). Receipt `mutant-stoer_wagner.log` SHA-256 `e071720687d3d8f5d54a0ef3cb082998fbadd358ebf693207340197efcecbae6`. Restored production passes all 269 library tests. The changed-file gate reports all six new sources registered in its temporary scoped registry; groupant structural validation reports no diagnostics. These gates validate declarations and bindings, not semantics.

# algorithm-hungarian

## descriptant

### hungarian: mod.rs

This is a new implementation epoch, not a measured optimization. No speedup or numerical surrogate is proposed: exact objectives remain exact. Partial source syntax has opaque macro bodies; semantic analysis and unrestricted correctness remain outside the structural model. Four compiled objective/oracle controls are retained as falsification evidence.

## approximant

### Forward, invariant and cost

Forward: minimize sum_i C[i,p_i] over injections p from r rows into c columns. Return the deterministic primal witness and dual potentials. Invariant at return: distinct columns, u_i+v_j<=C_ij, equality on chosen pairs, v_j<=0 and v_j=0 for unused columns. Tight alternating-tree augmentations preserve optimality of the processed-row matching. Cost O(r^2*c) time and O(r+c) auxiliary/output storage, excluding input. Admission bounds r<=512, c<=4096 and r*r*c<=20000000. Work product is checked; all signed edge/potential arithmetic is widened to i128 under finite path bounds. The 1955 Kuhn paper supplies classical attribution; this source and fixtures are original.

## models

# Typed models: hungarian mod.rs

## behavior_complement

Universe: admitted rectangular signed matrices paired with candidate assignments and potentials. Excluded relation: repeated columns, wrong dimensions, objective unequal to selected edge sum, any u_i+v_j>c_ij, positive column potential, or nonzero potential on an unused column. This is a necessary-condition inner description of absent successful outputs, not an exhaustive operational complement. Evidence: exhaustive_small_assignments and boundaries_ties_and_extremes.

## reverse_reconstruction

Fix a returned assignment (p,z,u,v). Compatible antecedents include every admitted i64 matrix C with C[i,p_i]=u_i+v[p_i], C[i,j]>=u_i+v_j, sum_i C[i,p_i]=z and the same deterministic augmentation result. The inequalities alone form an outer relaxation because tie behavior is not inverted. Nonselected entries can vary, so recovery is ambiguous. Evidence checks forward inclusion, not complete enumeration of this fiber.

## dual_certificate

For all admitted C, let u_i+v_j<=C_ij, v_j<=0, equality on selected columns, and v_j=0 on unused columns. Then any injective q has sum C[i,q_i]>=sum u_i+sum v_j, attained by p. Observation is primal/dual objective equality plus all reduced costs. Test check independently validates every inequality and edge identity.

## augmenting_state

During one row insertion, visited columns form an alternating tree; owner maps occupied columns to distinct rows. Slack[j] is the minimum reduced edge from a reached row to unvisited j, with previous[j] retaining its first strict minimizer. Potential updates make at least one new column tight. First update can be negative for a previously unprocessed row; global dual feasibility is asserted at completion, not before that row is introduced. Exhaustive finite optima constrain the recurrence.

## tie_selection

With fixed matrix, ascending row and column scans, strict slack comparisons and predecessor retention select a repeatable witness. This specifies the procedural selector, not lexicographic optimization. Zero 2x3 selects [0,1]. Observation includes returned potentials as well as columns. Repeat equality in check is a finite stability oracle.

## row_translation

For mathematical row offsets d_i that keep entries in i64 and admission unchanged, adding d_i to every entry of row i preserves the set of optimal injections and shifts optimum by sum d_i. This is an objective/set counterfactual, not a promise that every internal dual coordinate is invariant. Exhaustive signed matrices exercise some instances; a universal Rust metamorphic experiment is not executed.

## resource_partition

Input partition ordered by rows>512, first row width>4096, any unequal row width, rows>columns, checked rows*rows*columns>20000000, allocation failure, then success. Later-row width alone triggers raggedness, not first-row cap. Product checking occurs before scratch allocation. Named precedence/work fixtures execute finite representatives; allocator failure is not injected.

## wide_arithmetic

At most 512 i64 edges sum into i128. Potentials and finite reduced costs are bounded by alternating paths of at most 512 rows and i64 cost differences, far below i128 magnitude. i128::MAX is a slack sentinel only until every unvisited column receives a finite first relaxation; no sentinel remains in subtraction. Extremal signed fixtures test objective and certificate identity. No arbitrary-dimension arithmetic claim is made.

## tests

Run cargo test -p mova-algorithms hungarian::tests. Three tests cover independent enumerations, objective/witness identities and shape/resource boundaries. Debug and release each repeat twice with identical sorted test outcomes. Strict all-targets Clippy and narrow rustfmt pass. Raw logs stay under external run key (see History); see model.md#validation for hashes. No cross-algorithm integration is executed in this leaf task; the root owns composition tests.

### Checks

## application-transfers

### Application transfers

Declared consumers of this algorithm's modeled mechanism:

- hungarian-scratch: [coref/src/score.rs](../../../../coref/src/score.rs.groupant/model.md#transfer)

[Root transfer catalog](../../../../models/mova.groupant/algorithm-transfers.json). These are scoped mechanism transfers; original algorithm ownership and numerical contracts remain unchanged. Evidence status is recorded in the catalog.

## validation

### Validation

Six owned tests pass twice in debug and twice in release. Three per module have identical normalized test identities across all four runs. Four compiled implementation/test-oracle objective controls fail named equality assertions; exact source bytes restore and positive focused checks pass. Strict all-targets algorithm Clippy passes. Partial syntax projections repeat byte-identically. No timing or general proof claim.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original exact implementation and fixtures. Deterministic ties, witness identity, explicit admission and wide objective arithmetic define the API. The implementation exports its mathematical certificate/witness directly; finite exhaustive checks inspect those identities independently.

### 2026-10-30 · document

Subject: adjacent source SHA-256 in manifest; epoch 2026-10-30. Observation is the pure call result or finite test outcome. All entries are partial relational specifications supported by source inspection and the named finite checks; formulas are not automatically verified. Allocation behavior is conditional on successful reservation. No general inverse, semantic AST or approximation-error guarantee is claimed.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms hungarian::tests. Three tests cover independent enumerations, objective/witness identities and shape/resource boundaries. Debug and release each repeat twice with identical sorted test outcomes. Strict all-targets Clippy and narrow rustfmt pass. Raw logs stay under external run key octopus-wave11-optimization-2026-10-30; see model.md#validation for hashes. No cross-algorithm integration is executed in this leaf task; the root owns composition tests.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External run key: octopus-wave11-optimization-2026-10-30. SHA-256 evidence:

- `hungarian-debug-1.log`: `3b4d415736ca9be57fd08c71d5a67711475d01b78a3c7536f9dfd902dee99ee2`

- `hungarian-debug-2.log`: `ac6e474af571e4ad581babb8d559de2cc6c34977328bd6225a092baebe6e20b1`

- `hungarian-release-1.log`: `1c8a970e6b676297726fd2f9f228b791d57d555d1299356039fe10a3ed3bfce6`

- `hungarian-release-2.log`: `a6ce6070323bccc7e41893d63441a833b4e9989b9c41a6ad28a0eb39c2eef74d`

- `hungarian-identity.txt`: `89ea7ecaf483075d797f0594ec86f9098caf82dffd4b03843a2b1fc2cd959031`

- `hungarian-mod.rs-mutant.log`: `f671959adb3232c522bd84ed03b8421424d96c11f610c25712f5701ef081f84e`

- `hungarian-mod.rs-mutant-restored.log`: `6bcebf0f2b759565b335d5fe4afe75b160dc449e238e7b211d91af801cc89c16`

- `clippy.log`: `5f04f6b51d898d08f9077b5d6c8fedf70a17c77579347c18617e41b4292617fb`

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

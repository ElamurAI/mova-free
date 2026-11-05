# algorithm-bounded-knapsack-tests

## descriptant

### bounded_knapsack: tests.rs

This epoch retains exact objectives rather than introducing a numerical surrogate: no useful approximate contract or speedup is claimed. Finite evidence establishes the listed observations only. Allocation exhaustion and general Eq implementations remain outside injected checks. Ties were made explicit: Hirschberg smallest oriented split can select the last repeated position in the longer input. Sources are source-bound partial syntax projections; macro tokens are opaque.

## approximant

### Forward, invariant and cost

Finite harness maps independent fixtures and a candidate implementation to pass or first violated assertion. Knapsack enumerates 4096 three-item cases at eight capacities plus count/remainder, limits and arithmetic fixtures; oracle recursion enumerates all multiplicities. LCS enumerates 63 binary words crossed with each other and enumerates all left subsequences, then checks length, ordering, equality, and repeat identity. Fixture arithmetic is bounded. Cost is fixture count times production plus exponential tiny oracle work; no scalability claim for tests.

## models

# Typed models: bounded_knapsack tests.rs

Subject/revision is the adjacent source SHA in manifest.json. Observation is one call (or one finite test execution for tests.rs). Status: partial source-derived relational specification. Unless a section states an error domain, allocation succeeds. Each section supplies a distinct operator; evidence and gaps are in model.md#tests. No automatic inverse, semantic AST or general proof is claimed.

## behavior_complement

Universe: executions of this fixed finite harness with candidate production functions. The accepted relation requires every asserted equality, index constraint and resource classification. Its complement includes a returned nonoptimal result or an invalid witness on an enumerated fixture. This source-derived exclusion is an inner model of rejection; compiler errors, panics in Eq and unexecuted cases are outside the finite observation.

## reverse_reconstruction

Given an observed pass, retain all production functions and fixture/oracle states agreeing at every executed assertion. Functions differing off the finite domain remain compatible. Given a failure name retain every state violating that named predicate, not a unique bug. This is an exact abstract assertion fiber, not an implemented inverse solver.

## independent_oracle

Knapsack enumerates every allowed multiplicity tuple directly, without binary bundles or a DP row; tiny weights/values prevent oracle overflow. LCS enumerates all left subsequences and greedily embeds each in the right input, without DP splits. Equality against these independent finite objectives tests optimality. Only the oracle present in the bound file is executed.

## mutation_sensitivity

Hold fixture ordering and source snapshot fixed; replace the production objective recurrence or independent objective oracle by the recorded false rule. The named exhaustive assertion must reject. This finite counterfactual checks a gate can turn red, not that all faults are detectable.

## boundary_partition

Knapsack distinguishes capacity, item-count, update-budget and genuine-value-overflow cases, including infeasible huge items. LCS distinguishes length/product limits, empty input and reserve-capacity overflow through private row. Allocator exhaustion is not injected; synthetic reservation capacity overflow does not simulate heap pressure.

## tests

### Checks and limits

Run `cargo test -p mova-algorithms bounded_knapsack::tests`. Four tests in this family cover independent finite optimization, boundaries and metamorphic behavior. Full crate runs twice per debug/release profile compare sorted test identities; exact evidence hashes are in model.md#validation. Production and oracle mutations are compiled and rejected by the corresponding exhaustive assertion, then exact source bytes restored. Heap exhaustion is not forced; LCS row capacity overflow is tested separately. No school/parser/GPU or timing experiment is represented.

## validation

### Validation

Full crate: 306 unit tests passed twice in debug and twice in release; sorted test identity/results equal within profiles. Eight owned tests are included. Strict algorithms Clippy all-targets passes. Four compiled negative controls reject named exhaustive assertions: two production recurrences and two false test oracles. Exact sources restored and full unit suite passes again. Partial syntax generation repeated byte-identically. No performance gain claimed.

## History

### 2026-10-30 · descriptant

Original Rust implementation and fixtures, epoch 2026-10-30. Prospective recurrence and storage predictions were recorded before implementation. Binary decomposition bounds multiplicity work; Hirschberg keeps only a row frontier and an index witness. Classical attribution is in the implementation module.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Raw evidence uses external run key `octopus-wave10-dp-2026-10-30`; SHA-256 bindings:

- `prospective.md`: `a1c6dc3520c16b26a3da58f687baadb8950e0eab32f7fd9352ceb58a283cbe3e`

- `debug1.log`: `dd9825b8d7817a24355d608d02198b49857a8e16569acafb7c3f3e92277ee7a8`

- `debug2.log`: `cefae10ce0e1a4b4765ae345d839f1f8df1f230ea2cf7e8fad727e84231efa4b`

- `release1.log`: `830b5f6e47332533901005a32e0a9da9dc0b636eaeb5ba67b09b6e6d95ccdaab`

- `release2.log`: `0f828dc97f9af622784409c560eb61916e1fbb9695fa64e47fae3fc1c5abc4e4`

- `debug-normalized.txt`: `b1530a0c7853fc435e2a567f1801a30b0fba0ff0fe7a3c5cc3f90f6b2f7dd106`

- `release-normalized.txt`: `b1530a0c7853fc435e2a567f1801a30b0fba0ff0fe7a3c5cc3f90f6b2f7dd106`

- `mutant-knapsack-production.log`: `ec7a52b673890aa20ca621a80349f88afff716d6bee12b9e63a17ebd84d023c8`

- `mutant-lcs-production.log`: `5103408a039151aef5a685775ebb4a1e3fbaba75dc00180cf5d861fe609766a6`

- `mutant-knapsack-oracle.log`: `76bad5d5bd9eed07f9563b861cec8853367c15928dfb1fb023ae92f4a9d63dca`

- `mutant-lcs-oracle.log`: `78e08faa5679e278d33ee29736b34c36ea52f637410d99cbad4ecd1a02fdf98e`

- `restored.log`: `942594dba6313e61343663bd7ff03dcda302667dd13b56f9d3b59b029d9773cb`

- `clippy.log`: `3eb73789db375d099cdecb139cfb88fe81f7a704adb1f008f04db141c506c51e`

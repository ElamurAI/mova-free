# algorithm-bounded-knapsack

## descriptant

### bounded_knapsack: mod.rs

This epoch retains exact objectives rather than introducing a numerical surrogate: no useful approximate contract or speedup is claimed. Finite evidence establishes the listed observations only. Allocation exhaustion and general Eq implementations remain outside injected checks. Ties were made explicit: Hirschberg smallest oriented split can select the last repeated position in the longer input. Sources are source-bound partial syntax projections; macro tokens are opaque.

## approximant

### Forward, invariant and cost

F(I,C)=max{sum k_i*v_i: 0<=k_i<=count_i, sum k_i*w_i<=C}. Positive counts truncate before arithmetic and decompose into bundles; descending row updates preserve feasible single use. Free items add their full value. Exact overflow indicates F>u128::MAX after resource admission. With B=sum O(log(1+min(count_i,C/w_i))) over positive weights, time O(n+C*B), space O(C). Update preflight itself is O(n+B). Resource error order is capacity, number of types, update count, allocation, then objective arithmetic. Invariant: every dp[c] is the optimum from processed bundles at weight at most c.

## models

# Typed models: bounded_knapsack mod.rs

Subject/revision is the adjacent source SHA in manifest.json. Observation is one call (or one finite test execution for tests.rs). Status: partial source-derived relational specification. Unless a section states an error domain, allocation succeeds. Each section supplies a distinct operator; evidence and gaps are in model.md#tests. No automatic inverse, semantic AST or general proof is claimed.

## behavior_complement

Universe: (items,C,result) with nonnegative fields and successful allocation after resource admission. Define F=max sum k_i*v_i over 0<=k_i<=count_i and sum k_i*w_i<=C. Exclude Ok(v) with v!=F, and exclude ValueOverflow if F<=u128::MAX. This source-derived inner exclusion also rules out capacity-exceeding selections; finite checks support but do not prove the implementation relation.

## reverse_reconstruction

For fixed admitted C and Ok(v), retain all item lists with F=v. Many antecedents share v: arbitrary zero-count items and many overweight items do not change F. Admission is part of the fiber because extra bundles can trigger WorkLimit. ValueOverflow retains F>u128::MAX only after resource and allocation success. No unique recovery from value exists.

## binary_coverage

Fix one positive weight w and t=min(count,C/w). Greedy bundle sizes 1,2,4,... plus the remainder sum to t and their subset sums cover every integer 0..t. A remainder may duplicate an earlier size; these are distinct bundles. This relation preserves feasible multiplicities, not one unique encoding. Count 19 and duplicate-item fixtures exercise remainders.

## descending_dependency

Before each bundle, dp[c] is optimum for earlier bundles at weight at most c. Descending c ensures dp[c-weight] still belongs to earlier bundles, so the new bundle is used at most once. Reversing traversal changes the problem to reuse; finite enumeration is the falsifiable oracle.

## overflow_feasibility

Every multiplied positive bundle fits C after truncation. Every candidate sum combines feasible earlier bundles with this bundle, so checked overflow proves some feasible solution exceeds u128. Free-item totals also represent feasible choices. Conversely when all feasible sums fit, none of these nonnegative partial sums overflow. Infeasible original count*value is never formed for positive weights.

## zero_weight

Fix admitted positive-weight items and free contribution Z=sum count*value for zero-weight items. F=Z+F_positive. Count zero contributes zero even at maximum value. An overflowing Z implies genuine objective overflow; zero capacity still permits all zero-weight copies. Tests cover zero multiplication and free-plus-positive overflow.

## monotonicity

Within successful resource admission and representable optima, increasing capacity or a count cannot decrease optimum; reordering items preserves optimum. Returned errors need not be monotone in this projected value order: workload changes admission. No selection tie or permutation-invariant error-order claim is inferred.

## resource_partition

Validation first rejects C>1,000,000, then n>100,000. Sum over positive bundles of C-bundle_weight+1 must be <=20,000,000 with checked sum. Allocation reserves C+1 u128 slots next. Value checks follow. Counts use arbitrary usize; positive effective counts are <=C, so doubling bundle size and bundle weights cannot overflow under the cap. Allocation refusal remains environmental.

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

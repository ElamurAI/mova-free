# algorithm-hirschberg

## descriptant

### hirschberg: mod.rs

This epoch retains exact objectives rather than introducing a numerical surrogate: no useful approximate contract or speedup is claimed. Finite evidence establishes the listed observations only. Allocation exhaustion and general Eq implementations remain outside injected checks. Ties were made explicit: Hirschberg smallest oriented split can select the last repeated position in the longer input. Sources are source-bound partial syntax projections; macro tokens are opaque.

## approximant

### Forward, invariant and cost

H(a,b) returns an exact maximal-length ordered index witness. L(i,j) is the standard prefix LCS length. At a midpoint select the smallest j maximizing forward[j]+backward[len(b)-j], then concatenate ordered recursive witnesses with offsets. Shorter input is row axis, swapping coordinates afterward if needed. Invariant: every output pair matches and both coordinates increase; each recursive domain remains a disjoint ordered rectangle. Time O(m*n) equality calls (geometric area sum across splits), two O(min(m,n)) rows reused across O(log max(m,n)) stack frames plus O(L) witness storage. Equality cost is additional. Empty inputs bypass row allocation after limits.

## models

# Typed models: hirschberg mod.rs

Subject/revision is the adjacent source SHA in manifest.json. Observation is one call (or one finite test execution for tests.rs). Status: partial source-derived relational specification. Unless a section states an error domain, allocation succeeds. Each section supplies a distinct operator; evidence and gaps are in model.md#tests. No automatic inverse, semantic AST or general proof is claimed.

## behavior_complement

Universe: (left,right,result) with stable Eq equivalence and successful allocation after admission. Exclude Ok(P) unless both index coordinates strictly increase, every indexed pair compares equal and |P| is the maximum common-subsequence length. Also exclude witnesses violating the oriented tie recurrence. This source-derived inner exclusion has finite exhaustive support, not a general Rust proof.

## reverse_reconstruction

For fixed P, compatible inputs are every admitted pair whose exact recurrence produces P, including all matching selected symbols and all unselected symbols consistent with optimality/ties. Pairwise equality alone gives an outer approximation: it omits maximality and tie constraints. Empty P has many disjoint-alphabet antecedents. No unique symbol recovery exists.

## row_recurrence

For prefixes of oriented inputs, L(i,j)=L(i-1,j-1)+1 when symbols equal, else max(L(i-1,j),L(i,j-1)), with zero boundaries. The diagonal temporary holds the overwritten previous-row predecessor; one mutable row retains the required dependency frontier. Reverse traversal applies the same relation to reversed suffixes without copying symbols.

## split_optimality

At midpoint h of the longer oriented sequence a, choose j maximizing prefixLCS(a[..h],b[..j])+suffixLCS(a[h..],b[j..]). Splitting any common subsequence at h supplies an upper-bound decomposition, and concatenating optimal subwitnesses attains it. Shared split j is essential; independently maximizing two incompatible cuts would overstate length.

## tie_policy

Orient shorter input as b, right when equal. Among maximizing j choose smallest; when a has length one choose first matching b position. Recursively concatenate left then right. Ties in ab/ba give (1,0); a/aaa gives (0,2) because orientation and repeated zero splits move the match to the last a position. This is deterministic, not globally earliest-index optimization.

## axis_transposition

Swapping output coordinates after solving swapped oriented inputs restores caller index meaning. LCS lengths are symmetric; exact witness vectors need not transpose under caller swapping on equal-length inputs, because orientation tie policy can change. Tests therefore assert symmetry only for length and independently validate every index/equality.

## storage_lifetime

Two rows of min(m,n)+1 usize cells are reserved once and reused after computing each split. Ancestors retain no rows, only split/offset frames; recursion depth is O(log(max(m,n))). Output reserves min(m,n) pairs and is bounded by that length. Total auxiliary space is O(min+log max); generic Eq cost and allocator metadata are excluded.

## resource_partition

Reject max length>1,000,000 before checked product>16,000,000. Empty admitted input returns empty without allocation. Otherwise reserve both rows then witness with fallible Vec reserve; allocation failure returns typed error. Prefix lengths, combined LCS length and index offsets are bounded by admitted lengths. Arbitrary expensive or panicking Eq remains caller behavior.

## tests

### Checks and limits

Run `cargo test -p mova-algorithms hirschberg::tests`. Four tests in this family cover independent finite optimization, boundaries and metamorphic behavior. Full crate runs twice per debug/release profile compare sorted test identities; exact evidence hashes are in model.md#validation. Production and oracle mutations are compiled and rejected by the corresponding exhaustive assertion, then exact source bytes restored. Heap exhaustion is not forced; LCS row capacity overflow is tested separately. No school/parser/GPU or timing experiment is represented.

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

# algorithm-binary-search

## descriptant

### binary_search descriptant

The local mathematical combiner reads the authoritative Rust source, source-bound partial syntax, forward relation, five typed models, invariants and finite test observations. It updates conditional claims while retaining earlier predictions and failure observations. This is a documented review operation, not an autonomous generator.

Two tests passed twice. Bounded oracle enumerates 3,280 sequences of lengths 0..7 over {-1,0,1}, sorts each, and checks five targets: 16,400 cut/first pairs. Literal empty, maximum, missing and borrowed-string cases supplement it. Mutating < to <= compiled and failed tests (exit 101).

Scope is successful calls on lawful generic implementations where applicable. Partial syntax omits macro expansion, cfg resolution and semantic typing. Finite oracles do not establish all inputs or full Mova integration.

Parent workspace wiring and higher-level registry composition are handled separately; these leaf relations establish no application integration by themselves.

Test-extraction epoch: the test harness is now adjacent in ../tests.rs. model.md#test-extraction retains the new oracle scope and mutation observations alongside this earlier epoch.

## model

# binary_search model

## Forward

For sorted A and target t, k=|{i:A[i]<t}|. lower_bound returns k. first returns Some(k) iff k<n and A[k]=t, else None.

## Invariant

0<=lo<=hi<=n; all i<lo satisfy A[i]<t; all i>=hi satisfy A[i]>=t. Each nonempty iteration strictly shortens hi-lo; midpoint lo+(hi-lo)/2 lies inside the interval.

## Cost

O(log(n+1)) comparisons, O(1) auxiliary storage; first adds at most one equality comparison. Assumes constant-cost comparisons; no timing measured.

## nonbehavior

Type: behavior_complement. Let U={(A,t,k): sorted A, 0<=k<=len(A)}. R is the forward cut relation. U minus R consists exactly of k with an earlier element >=t or a later element <t. Candidate source-derived exact characterization under preconditions; bounded oracle evidence, no general machine proof.

## antecedents

Type: reverse_reconstruction. For fixed output k, retain all sorted A and t with k<=n, every i<k having A[i]<t and every i>=k having A[i]>=t. Many lengths, values and targets remain possible; no unique inverse.

## membership

Type: observation_projection. For fixed A,t, project cut k to Some(k) iff k<n and A[k]=t; otherwise None. None combines t below/above or between keys and loses the insertion point. Linear position oracle checks this projection.

## duplicate_cut

Type: counterfactual_boundary. Change comparison < to <=: target relation becomes upper bound |{i:A[i]<=t}|. Difference from actual lower bound equals the count of equal keys. Tests with duplicates reject this changed rule as actual behavior.

## cut_fiber

Type: target_partition. Fix sorted A. For 0<k<n, targets mapping to k satisfy A[k-1]<t<=A[k]; k=0 requires t<=A[0], k=n requires A[n-1]<t. Empty A maps every target to zero. Equal neighbors can make interior fibers empty. Bounded linear oracle tests all five targets per input.

## tests

### Bounded evidence

Two tests passed twice. Bounded oracle enumerates 3,280 sequences of lengths 0..7 over {-1,0,1}, sorts each, and checks five targets: 16,400 cut/first pairs. Literal empty, maximum, missing and borrowed-string cases supplement it. Mutating < to <= compiled and failed tests (exit 101).

Commands: rustfmt --edition 2021 SOURCE; rustc --edition 2021 --test SOURCE -o EXTERNAL_BINARY; EXTERNAL_BINARY --test-threads=1. The two runs' status logs match after elapsed-time removal. Mutants compile from external copies and must exit 101; repository sources remain unchanged. Run artifacts are outside the repository under the task run directory in MOVA_DATA.

Scope is successful calls on lawful generic implementations where applicable. Partial syntax omits macro expansion, cfg resolution and semantic typing. Finite oracles do not establish all inputs or full Mova integration.

Current extraction epoch: tests now live in ../tests.rs; see model.md#test-extraction for four focused tests, new coverage and refreshed evidence. Earlier two-test results above describe the prior epoch.

## prospective

### Prospective

Saved before implementation. Assume a sorted slice and a lawful total Ord. Predict returned cut equals a linear first-not-less oracle, including empty slices, duplicates and absent targets. Half-open bounds should avoid index overflow; O(log(n+1)) comparisons and O(1) space.

Plan: independent bounded exhaustive oracle, literal boundaries, two deterministic test-status runs and one source mutation that must fail. No timing or full-system claim. No external source or solutions downloaded. Parent owns workspace/registry integration. Syntax projection follows rustfmt and stable source.

## test-extraction

### Test extraction

The implementation prefix is unchanged. Its cfg(test) module now loads ../tests.rs, whose adjacent groupant records the expanded finite domain. Two original tests remain and two new tests pass twice. Replacing midpoint < with <= is rejected by wider_duplicate_runs_linear_oracle (compiled external copy, exit 101).

Expected cut is the first linear >= position and first match is linear equality position; no interval narrowing is reused.

Reverse wrapper changes the lawful order; translation adds 1000 to both sorted values and targets and must preserve cut indices.

Partial syntax is refreshed because inline test items moved out. The production source hash changes for module placement, not algorithm behavior. Earlier prospective and executed evidence is retained; current detailed evidence is in ../tests.rs.groupant/model.md#evidence. Full crate check at this isolated base: 36 tests and one doctest pass; owned-source formatting and strict crate Clippy pass. Parent binding/registry checks remain integration work.

## History

### 2026-10-29 · descriptant

Epoch 2026-10-29. Prospective assumptions are preserved in model.md#prospective. Original repository-owned implementation of the standard algorithmic idea; no external solution copied.

### 2026-10-29 · tests

Heading `### Bounded evidence 2026-10-29` stays in tests as `### Bounded evidence`.

### 2026-10-29 · prospective

Heading `### Prospective epoch 2026-10-29` stays in prospective as `### Prospective`.

### 2026-10-29 · test-extraction

Heading `### Test extraction 2026-10-29` stays in test-extraction as `### Test extraction`.

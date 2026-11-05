# algorithm-merge-sort

## descriptant

### merge_sort descriptant

The local mathematical combiner reads the authoritative Rust source, source-bound partial syntax, forward relation, five typed models, invariants and finite test observations. It updates conditional claims while retaining earlier predictions and failure observations. This is a documented review operation, not an autonomous generator.

Two tests passed twice. Independent stable bucket oracle enumerates all 3,280 sequences of lengths 0..7 over three keys, checking exact key/identity output against concatenated original-key subsequences. Reverse length 1,025, numeric extremes and owned strings supplement it. Mutating merge <= to < compiled and failed the stability test (exit 101).

Scope is successful calls on lawful generic implementations where applicable. Partial syntax omits macro expansion, cfg resolution and semantic typing. Finite oracles do not establish all inputs or full Mova integration.

Parent workspace wiring and higher-level registry composition are handled separately; these leaf relations establish no application integration by themselves.

Test-extraction epoch: the test harness is now adjacent in ../tests.rs. model.md#test-extraction retains the new oracle scope and mutation observations alongside this earlier epoch.

## model

# merge_sort model

## Forward

S(A) is the unique ascending permutation of input occurrences in which equal-key occurrences retain their original relative order. Recursively sort the two halves, then merge by selecting the left head on equality.

## Invariant

Each merge prefix is sorted and contains exactly the consumed left/right occurrences; within each half encounter order is preserved. i and j count consumed positions and stay in bounds. Recursion halves length until less than two.

## Cost

T(n)=T(floor(n/2))+T(ceil(n/2))+O(n), hence O(n log(n+1)) comparisons/clone work. One n-element scratch allocation and O(log(n+1)) stack. Clone/comparison costs assumed bounded; element-owned allocations are excluded. No timing measured.

## nonbehavior

Type: behavior_complement. U consists of pairs (A,B) of finite occurrence sequences. R(A,B) means B is the stable ascending permutation. U minus R includes changed length/multiplicity, descending adjacent keys, or reversal of equal-key identities. Under lawful comparison/clone these exhaust unsuccessful output candidates; bounded evidence is not general proof.

## antecedents

Type: reverse_reconstruction. For fixed sorted B, the preimage contains every permutation A of B's occurrences preserving the order of each equal-key subsequence seen in B. Distinct-key interleavings remain ambiguous. Unsorted B has empty preimage.

## equal_key_order

Type: stability_projection. Fix key q. Filtering S(A) to occurrences with key q yields precisely filter(A,key=q), retaining identity order. Independent bucket oracle builds all such subsequences in ascending key order.

## tie_policy

Type: counterfactual_boundary. Change <= to < at merge ties. Sorted key output remains possible, but equal-key items can cross from right before left. Two equal keys with distinct identities witness the divergence; the stability oracle rejects this mutant.

## idempotence

Type: fixed_point. For stable sorted B, S(B)=B by occurrence identity. For arbitrary A, S(S(A))=S(A). Domain holds key and identity fixed. Direct two-call oracle is prospective; bounded bucket equality provides evidence for the premise and single-call relation.

## tests

### Bounded evidence

Two tests passed twice. Independent stable bucket oracle enumerates all 3,280 sequences of lengths 0..7 over three keys, checking exact key/identity output against concatenated original-key subsequences. Reverse length 1,025, numeric extremes and owned strings supplement it. Mutating merge <= to < compiled and failed the stability test (exit 101).

Commands: rustfmt --edition 2021 SOURCE; rustc --edition 2021 --test SOURCE -o EXTERNAL_BINARY; EXTERNAL_BINARY --test-threads=1. The two runs' status logs match after elapsed-time removal. Mutants compile from external copies and must exit 101; repository sources remain unchanged. Run artifacts are outside the repository under the task run directory in MOVA_DATA.

Scope is successful calls on lawful generic implementations where applicable. Partial syntax omits macro expansion, cfg resolution and semantic typing. Finite oracles do not establish all inputs or full Mova integration.

Current extraction epoch: tests now live in ../tests.rs; see model.md#test-extraction for four focused tests, new coverage and refreshed evidence. Earlier two-test results above describe the prior epoch.

## prospective

### Prospective

Saved before implementation. Assume lawful Ord and value-preserving Clone. Predict sorted permutation with equal-key identity order preserved by choosing left on equality. One O(n) scratch clone and O(log n) recursion, O(n log(n+1)) comparison/clone work.

Plan: independent bounded exhaustive oracle, literal boundaries, two deterministic test-status runs and one source mutation that must fail. No timing or full-system claim. No external source or solutions downloaded. Parent owns workspace/registry integration. Syntax projection follows rustfmt and stable source.

## test-extraction

### Test extraction

The implementation prefix is unchanged. Its cfg(test) module now loads ../tests.rs, whose adjacent groupant records the expanded finite domain. Two original tests remain and two new tests pass twice. Changing left-on-equality <= to < is rejected by wider_stable_identity_buckets (compiled external copy, exit 101).

Expected occurrence order is concatenated original equal-key subsequences in key order; it uses no recursive merging or standard sort.

A second sort must preserve every key/identity pair. Reverse wrappers require descending numeric key order while keeping the declared total-order contract.

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

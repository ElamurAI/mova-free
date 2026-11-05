# algorithm-damerau-levenshtein-tests

## descriptant

### Exact unrestricted unit-cost byte edit distance.

The implementation catalog records classical attribution and algorithm-specific resource assumptions; test catalogs retain oracle limitations and fault-detection evidence.

## approximant

### Forward, invariant and cost

Execute deterministic independent finite oracles and named assertions, returning a harness outcome. Test oracle costs can be exponential and are deliberately bounded by fixture dimensions.

Invariant: Every completed test has checked all its enumerated cases; passing does not constrain absent cases.

Cost: Finite bounded enumeration; output logs and fixture allocation are outside the production cost.

Limitations: source syntax is partial and macro bodies opaque. Mathematical descriptions and finite correspondence do not prove unrestricted Rust behavior. No timing or approximation-error advantage is claimed.

## models

# Typed relations

Subject/revision: source path and SHA-256 in manifest. Each section defines a separate observation and relation on immutable byte inputs. Status: partial source-derived specification with finite executed correspondence, not a semantic proof. Approximation direction is stated per relation; converse sets retain ambiguity. Assumptions: supported 32/64-bit Rust platforms, successful ordinary allocation and no external state. Evidence: four native tests per module, repeated debug/release runs and compiled controls in model.md#tests. Counterfactuals keep the named quantities fixed. No executable approximate surrogate or percentage-error claim is made.

## behavior_complement

Universe: finite fixture runs and pass/fail observations. Exclude successful completion when an executed production/oracle pair disagrees or an explicit boundary assertion fails. This conditional inner exclusion model assumes assertions remain enabled and the test process completes; it does not classify unseen fixtures.

## reverse_reconstruction

A passing test summary has many compatible implementations and fixtures, including implementations differing outside the finite domain. A failure name likewise has many faulty-code or faulty-oracle antecedents. Logs retain case/debug values where emitted, without claiming unique fault localization.

## independent_oracle

The implementation models catalog specifies the independent oracle: edit-graph BFS for Damerau, all substrings and suffixes for Eertree, and all partitions plus explicit rotations for Lyndon. Oracle assumptions and finite domains are retained; agreement does not prove either implementation beyond them.

## finite_domain

Four tests in this file execute the explicit exhaustive tiny domain, selected byte/period/witness examples, resource or reconstruction cases, and structural invariants. Input generation is deterministic Rust. Boundary tests allocate bounded vectors; machine allocation failure and impossible enormous slices are not injected.

## fault_detection

A production mutation and a purpose-specific false oracle are each compiled, run separately and rejected by named assertions. Exact source bytes are restored before the positive rerun. Mutation rejection demonstrates these faults are detectable, not mutation completeness or universal test adequacy.

## tests

### Executed evidence

Syntax projection generated twice with byte equality. Finite tests, structural registration and syntax hashes do not establish a semantic AST, universal proof, or full application integration. Parent joins are owned separately.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original implementation and independently enumerated fixtures added with adjacent source-bound models. This file owns test mechanics rather than production behavior. No optimization or speedup is claimed. The small oracle domains permit exact comparison; arbitrary long input correctness is not inferred from them. Output projection can lose input content; reverse models retain this ambiguity.

### 2026-10-30 · tests

2026-10-30: cargo test --offline -p mova-algorithms --lib passed 353 tests (12 new) twice in debug and twice in release. Sorted test identities/results were identical across all four runs: SHA-256 c3c1983072686ac513521c63abfe701413cba39ec62024d46f456e2a09319b61. Strict cargo clippy --offline -p mova-algorithms --all-targets -- -D warnings passed; narrow rustfmt checked the six owned Rust files.

Filter for this module: damerau_levenshtein::tests (four tests). Six compiled controls across the owned sources fail named assertions; this file's control log SHA-256 is 49a9841f4323af141607f79b6fb1abee446e45bd7cbcfd36a4a89bf3c5650db1. Exact bytes were restored; the final positive rerun passed all 353 tests and strict Clippy passed again. Raw evidence: $MOVA_DATA/runs/octopus-wave11-text-2026-10-30/.

Structural groupant reports repeated identically with SHA-256 c38c44782edf0a959f3e20d3736c47b2b46c5f09f52c3260f9d1c3966def1720; changed-source registration accepted all six owned sources. The external scaffold retains the unchanged base registry and adds these leaves under a temporary root without a materialized sidecar; integrated ancestors and joins remain the parent's responsibility.
